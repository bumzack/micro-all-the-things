using Microsoft.EntityFrameworkCore;

namespace AuthenticationService;

public class AuthenticationEntryContext : DbContext
{
    public AuthenticationEntryContext(DbContextOptions<AuthenticationEntryContext> options) : base(options)
    {
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.UseSerialColumns();
    }

    public DbSet<AuthenticationEntry> AuthenticationEntries { get; set; }
}