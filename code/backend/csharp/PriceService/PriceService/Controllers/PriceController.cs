using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;

namespace PriceService.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class PriceController : ControllerBase
    {
        private readonly PriceDataContext _context;

        public PriceController(PriceDataContext context)
        {
            _context = context;
        }

        // GET: api/Price
        [HttpGet]
        public async Task<ActionResult<IEnumerable<Price>>> GetPrice()
        {
            return await _context.Prices.ToListAsync();
        }

        // GET: api/Customer/5
        [HttpGet("/api/v1/price/{tconst}")]
        public async Task<ActionResult<Price>> GetPrice(string tconst)
        {
            var customer = await _context.Prices.FirstOrDefaultAsync(m =>
                m.MovieTconst == tconst
            );

            if (customer == null)
            {
                return NotFound();
            }

            return customer;
        }

        // GET: api/Customer/5
        [HttpPost("/api/v2/prices")]
        public async Task<ActionResult<IEnumerable<Price>>> GetPrices(PricesRequest request)
        {
            var customerPriceEntries = await _context.Prices
                .Where(p => request.movieTconst.Contains(p.MovieTconst))
                .ToListAsync();

            if (customerPriceEntries.Count == 0)
            {
                return NotFound();
            }

            return customerPriceEntries;
        }
    }
}