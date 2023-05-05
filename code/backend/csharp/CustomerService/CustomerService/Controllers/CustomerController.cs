using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;

namespace CustomerService.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class CustomerController : ControllerBase
    {
        private readonly CustomerDataContext _context;
        private readonly ILogger _logger;

        public CustomerController(CustomerDataContext context, ILogger<CustomerController> logger)
        {
            _context = context;
            _logger = logger;
        }

        // GET: api/Customer
        [HttpGet]
        public async Task<ActionResult<IEnumerable<Customer>>> GetCustomers()
        {
            _logger.LogInformation("get all customers");
            return await _context.Customers.ToListAsync();
        }

        // GET: api/Customer/5
        [HttpGet("/api/v1/customer/{email}")]
        public async Task<ActionResult<Customer>> GetCustomer(string email)
        {
            _logger.LogInformation("get   customer by email: {}",email);
            var customer = await _context.Customers.FirstOrDefaultAsync(m =>
                m.Email == email
            );

            if (customer == null)
            {
                _logger.LogInformation("get   customer by email: {}   --> not found",email);
                return NotFound();
            }

            _logger.LogInformation("get   customer by email: {}   --> got it",email);
            return customer;
        }
    }
}