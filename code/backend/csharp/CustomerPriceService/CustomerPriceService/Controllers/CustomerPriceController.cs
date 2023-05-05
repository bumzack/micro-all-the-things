using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;

namespace CustomerService.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class CustomerPriceController : ControllerBase
    {
        private readonly CustomerPriceDataContext _context;

        public CustomerPriceController(CustomerPriceDataContext context)
        {
            _context = context;
        }

        // GET: api/CustomerPrice
        [HttpGet]
        public async Task<ActionResult<IEnumerable<CustomerPriceEntry>>> GetCustomerPrices()
        {
            return await _context.CustomerPrices.ToListAsync();
        }

        // GET: api/CustomerPrice/5
        [HttpGet("/api/v1/customerprices/{customerId}")]
        public async Task<ActionResult<IEnumerable<CustomerPriceEntry>>>  GetCustomerPriceEntry(long customerId)
        {
            var customerPriceEntries =await _context.CustomerPrices
                .Where(p => p.CustomerId== customerId)
                .ToListAsync();

            if (customerPriceEntries.Count==0)
            {
                return NotFound();
            }

            return customerPriceEntries;
        }
        //
        // // GET: api/CustomerPrice/5
        // [HttpGet("{id}")]
        // public async Task<ActionResult<CustomerPriceEntry>> GetCustomerPriceEntry(long id)
        // {
        //     var customerPriceEntry = await _context.CustomerPrices.FindAsync(id);
        //
        //     if (customerPriceEntry == null)
        //     {
        //         return NotFound();
        //     }
        //
        //     return customerPriceEntry;
        // }

        // // PUT: api/CustomerPrice/5
        // // To protect from overposting attacks, see https://go.microsoft.com/fwlink/?linkid=2123754
        // [HttpPut("{id}")]
        // public async Task<IActionResult> PutCustomerPriceEntry(long id, CustomerPriceEntry customerPriceEntry)
        // {
        //     if (id != customerPriceEntry.Id)
        //     {
        //         return BadRequest();
        //     }
        //
        //     _context.Entry(customerPriceEntry).State = EntityState.Modified;
        //
        //     try
        //     {
        //         await _context.SaveChangesAsync();
        //     }
        //     catch (DbUpdateConcurrencyException)
        //     {
        //         if (!CustomerPriceEntryExists(id))
        //         {
        //             return NotFound();
        //         }
        //         else
        //         {
        //             throw;
        //         }
        //     }
        //
        //     return NoContent();
        // }
        //
        // // POST: api/CustomerPrice
        // // To protect from overposting attacks, see https://go.microsoft.com/fwlink/?linkid=2123754
        // [HttpPost]
        // public async Task<ActionResult<CustomerPriceEntry>> PostCustomerPriceEntry(CustomerPriceEntry customerPriceEntry)
        // {
        //     _context.CustomerPrices.Add(customerPriceEntry);
        //     await _context.SaveChangesAsync();
        //
        //     return CreatedAtAction("GetCustomerPriceEntry", new { id = customerPriceEntry.Id }, customerPriceEntry);
        // }
        //
        // // DELETE: api/CustomerPrice/5
        // [HttpDelete("{id}")]
        // public async Task<IActionResult> DeleteCustomerPriceEntry(long id)
        // {
        //     var customerPriceEntry = await _context.CustomerPrices.FindAsync(id);
        //     if (customerPriceEntry == null)
        //     {
        //         return NotFound();
        //     }
        //
        //     _context.CustomerPrices.Remove(customerPriceEntry);
        //     await _context.SaveChangesAsync();
        //
        //     return NoContent();
        // }
        //
        // private bool CustomerPriceEntryExists(long id)
        // {
        //     return _context.CustomerPrices.Any(e => e.Id == id);
        // }
    }
}
