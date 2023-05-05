using Microsoft.EntityFrameworkCore;

namespace CustomerService;

public class CustomerDataContext:DbContext
{
    public CustomerDataContext(DbContextOptions < CustomerDataContext > options): base(options) {}
    protected override void OnModelCreating(ModelBuilder modelBuilder) {
        modelBuilder.UseSerialColumns();
    }
    public DbSet <Customer> Customers {
        get;
        set;
    }
}