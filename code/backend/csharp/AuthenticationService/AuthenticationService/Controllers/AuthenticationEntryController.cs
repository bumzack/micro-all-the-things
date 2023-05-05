using System.IdentityModel.Tokens.Jwt;
using System.Security.Claims;
using AuthenticationEntry;
using CustomerService;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using Microsoft.IdentityModel.Tokens;
using NSwag;
using NSwag.CodeGeneration.CSharp;

namespace AuthenticationService.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class AuthenticationEntryController : ControllerBase
    {
        private readonly AuthenticationEntryContext _context;
        private readonly ILogger _logger;


        public AuthenticationEntryController(AuthenticationEntryContext context,
            ILogger<AuthenticationEntryController> logger)
        {
            _context = context;
            _logger = logger;
        }

        // GET: api/Price
        [HttpGet("/api/v1/authenticated/{customerId}")]
        public async Task<ActionResult<AuthenticationEntry>> GetAuthenticated(long customerId)
        {
            _logger.LogInformation("looking for auth for customer id {}", customerId);
            var authenticationEntry = await _context.AuthenticationEntries.FirstOrDefaultAsync(m =>
                m.CustomerId == customerId && m.Jwt != null
            );
            if (authenticationEntry == null)
            {
                _logger.LogInformation("looking for auth for customer id {}   - not found", customerId);
                return NotFound();
            }

            _logger.LogInformation("looking for auth for customer id {}   -  found it  {}", customerId,
                authenticationEntry);
            return authenticationEntry;
        }

        [HttpGet("/api/v1/stuff")]
        public async Task<ActionResult<string>> GetStuff()
        {
            string path =
                @"/Users/gsc/stoff/micro-all-the-things/code/backend/csharp/AuthenticationService/AuthenticationService/webflux_customer_service.json";

            var document = await OpenApiDocument.FromFileAsync(path);

            var settings = new CSharpClientGeneratorSettings
            {
                ClassName = "CustomerClient",
                CSharpGeneratorSettings =
                {
                    Namespace = "CustomerService"
                }
            };

            var generator = new CSharpClientGenerator(document, settings);
            var code = generator.GenerateFile();


            Console.WriteLine(code);

            string path_out =
                @"/Users/gsc/stoff/micro-all-the-things/code/backend/csharp/AuthenticationService/AuthenticationService/client/CustomerService.cs";


            // This text is added only once to the file.
            using (StreamWriter writer = System.IO.File.CreateText(path_out))
            {
                writer.WriteLine(code);
            }

            return code;
        }

        [HttpPost("/api/v1/authentication/login")]
        public async Task<ActionResult<AuthenticationEntry>> PostLogin(LoginRequest request)
        {
            var client = new HttpClient();
            var customerClient = new CustomerClient(client);
            customerClient.BaseUrl = "http://localhost:38980";
            try
            {
                var c = await customerClient.GetCustomerAsync(request.Email);
                _logger.LogInformation("login user   request {}   - customer {}", request, c);

                if (c == null)
                {
                    _logger.LogInformation("login user  no customer {} found", request.Email);
                    return NotFound();
                }

                var authenticationEntry = await _context.AuthenticationEntries.FirstOrDefaultAsync(m =>
                    m.CustomerId == c.Id
                );

                if (authenticationEntry == null)
                {
                    _logger.LogInformation("login user  no existing auth entry found {}", c.Id);
                    if (c.Email.Equals(request.Email) && c.Password.Equals(request.Password))
                    {
                        _logger.LogInformation("login user email and password matching {}", c.Id);
                        var auth = new AuthenticationEntry();
                        auth.CustomerId = c.Id;
                        auth.LoggedIn = DateTime.UtcNow;
                        auth.Jwt =  GenerateJSONWebToken(c.Email);

                        _context.AuthenticationEntries.Add(auth);
                        await _context.SaveChangesAsync();

                        _logger.LogInformation("login user email and password matching {}  -> auth entry  {}", c.Id,
                            auth);

                        return auth;
                    }
                }
                else
                {
                    if (authenticationEntry.Jwt != null)
                    {
                        return authenticationEntry;
                    }

                    if (c.Email.Equals(request.Email) && c.Password.Equals(request.Password))
                    {
                        _logger.LogInformation(
                            "login user. existing auth entry found {} , but jwt is null. email and password matching {}",
                            authenticationEntry, c.Id);
                        authenticationEntry.Jwt = GenerateJSONWebToken(c.Email);

                        _context.Entry(authenticationEntry).State = EntityState.Modified;
                        try
                        {
                            await _context.SaveChangesAsync();
                        }
                        catch (DbUpdateConcurrencyException)
                        {
                            if (!CustomerPriceEntryExists(authenticationEntry.Id))
                            {
                                return NotFound();
                            }

                            throw;
                        }

                        _logger.LogInformation("login user email and password matching {}  -> auth entry  {}", c.Id,
                            authenticationEntry);

                        return authenticationEntry;
                    }
                }
            }
            catch (ApiException e)
            {
                _logger.LogError("CustomerService returned an error {} ", e.Message);
                return NotFound();
            }

            return NotFound();
        }

        [HttpPost("/api/v1/authentication/logout")]
        public async Task<ActionResult<AuthenticationEntry>> PostLogout(LogoutRequest request)
        {
            var authenticationEntry = await _context.AuthenticationEntries.FirstOrDefaultAsync(m =>
                m.CustomerId == request.CustomerId
            );

            if (authenticationEntry == null)
            {
                return NotFound();
            }

            authenticationEntry.Jwt = null;
            authenticationEntry.LoggedOut = DateTime.UtcNow;

            _context.Entry(authenticationEntry).State = EntityState.Modified;
            try
            {
                await _context.SaveChangesAsync();
            }
            catch (DbUpdateConcurrencyException)
            {
                if (!CustomerPriceEntryExists(authenticationEntry.Id))
                {
                    return NotFound();
                }

                throw;
            }

            _logger.LogInformation("logout user -> auth entry  {}", authenticationEntry);

            return authenticationEntry;
        }

        // // GET: api/Customer/5
        // [HttpPost("/api/v2/prices")]
        // public async Task<ActionResult<IEnumerable<Price>>> GetPrices(LoginRequest request)
        // {
        //     var customerPriceEntries = await _context.AuthenticationEntries
        //         .Where(p => request.movieTconst.Contains(p.MovieTconst))
        //         .ToListAsync();
        //
        //     if (customerPriceEntries.Count == 0)
        //     {
        //         return NotFound();
        //     }
        //
        //     return customerPriceEntries;
        // }

        private bool CustomerPriceEntryExists(long id)
        {
            return _context.AuthenticationEntries.Any(e => e.Id == id);
        }
        
        private string GenerateJSONWebToken(string email)
        {
            var securityKey = new SymmetricSecurityKey("ThisismySecretKey"u8.ToArray());
            var credentials = new SigningCredentials(securityKey, SecurityAlgorithms.HmacSha256);

            var claims = new[]
            {
                new Claim(JwtRegisteredClaimNames.Email, email),
                new Claim(JwtRegisteredClaimNames.Jti, Guid.NewGuid().ToString())
            };

            var token = new JwtSecurityToken("foryouandyourfakewebshop.at",
                "foryouandyourfakewebshop.at",
                claims,
                expires: DateTime.UtcNow.AddMinutes(120),
                signingCredentials: credentials);

            return new JwtSecurityTokenHandler().WriteToken(token);
        }
    }
}