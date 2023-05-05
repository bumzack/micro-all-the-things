using Microsoft.EntityFrameworkCore;

namespace SearchDocService;

public class AuthenticationEntryContext : DbContext
{
    public AuthenticationEntryContext(DbContextOptions<AuthenticationEntryContext> options) : base(options)
    {
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.UseSerialColumns();
    }

    public DbSet<SearchDocService.AuthenticationEntry> AuthenticationEntries { get; set; }
}