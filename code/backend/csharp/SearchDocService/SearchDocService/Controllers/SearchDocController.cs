using Microsoft.AspNetCore.Mvc;
using NSwag;
using NSwag.CodeGeneration.CSharp;

namespace SearchDocService.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class SearchDocController : ControllerBase
    {
        private readonly AuthenticationEntryContext _context;
        private readonly ILogger _logger;

        public SearchDocController(AuthenticationEntryContext context,
            ILogger<SearchDocController> logger)
        {
            _context = context;
            _logger = logger;
        }

        private async void writeClient(string? json, string? clientName, string path, string pathOut)
        {
            var inFile = path + json;
            var document = await OpenApiDocument.FromFileAsync(inFile);

            var settings = new CSharpClientGeneratorSettings
            {
                ClassName = clientName,
                CSharpGeneratorSettings =
                {
                    Namespace = clientName
                }
            };

            var generator = new CSharpClientGenerator(document, settings);
            var code = generator.GenerateFile();

            Console.WriteLine(code);

            var outName = pathOut + clientName + ".cs";

            using (StreamWriter writer = System.IO.File.CreateText(outName))
            {
                writer.WriteLine(code);
            }
        }

        // GET: api/Price
        [HttpGet("/api/v1/stuff")]
        public async Task<ActionResult<string>> GetStuff()
        {
            _logger.LogInformation("get stuff for auth for customer ");

            var jsons = new List<string>();
            jsons.Add("webflux_authentication_service.json");
            jsons.Add("webflux_customer_service.json");
            jsons.Add("webflux_customerprice_service.json");
            jsons.Add("webflux_price_service.json");
            jsons.Add("webflux_search_search_index.json");

            var clients = new List<string>();
            clients.Add("AuthenticationService");
            clients.Add("CustomerService");
            clients.Add("CustomerpriceService");
            clients.Add("PriceService");
            clients.Add("SearchIndexService");

            var len = jsons.Count;

            var path =
                @"/Users/gsc/stoff/micro-all-the-things/code/backend/csharp/SearchDocService/SearchDocService/scripts/";
            var pathOut =
                @"/Users/gsc/stoff/micro-all-the-things/code/backend/csharp/SearchDocService/SearchDocService/clients/";

            for (int i = 0; i < len; i++)
            {
                writeClient(jsons[i], clients[i], path, pathOut);
            }

            return "all good";
        }

        // GET: api/Price
        [HttpGet("/api/v1/as/{customerId}")]
        public async Task<ActionResult<AuthenticationEntry>> GetAuthenticated(long customerId)
        {
            _logger.LogInformation("looking for auth for customer id {}", customerId);
            return NotFound();
        }
    }
}