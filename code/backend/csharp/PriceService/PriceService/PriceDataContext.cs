using Microsoft.EntityFrameworkCore;

namespace PriceService;

public class PriceDataContext : DbContext
{
    public PriceDataContext(DbContextOptions<PriceDataContext> options) : base(options)
    {
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.UseSerialColumns();
    }

    public DbSet<Price> Prices { get; set; }
}