using Microsoft.EntityFrameworkCore;

namespace CustomerService;

public class CustomerPriceDataContext:DbContext
{
    public CustomerPriceDataContext(DbContextOptions < CustomerPriceDataContext > options): base(options) {}
    protected override void OnModelCreating(ModelBuilder modelBuilder) {
        modelBuilder.UseSerialColumns();
    }
    public DbSet <CustomerPriceEntry> CustomerPrices {
        get;
        set;
    }
}