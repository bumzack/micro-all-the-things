using AuthenticationEntry;
using AuthenticationService;
using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.

builder.Services.AddControllers();
// Learn more about configuring Swagger/OpenAPI at https://aka.ms/aspnetcore/swashbuckle
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

builder.Services.AddDbContext<AuthenticationEntryContext>(o =>
    o.UseNpgsql(builder.Configuration.GetConnectionString("Ef_Postgres_Db"))
        .UseSnakeCaseNamingConvention()
        .UseLoggerFactory(LoggerFactory.Create(b => b.AddConsole()))
        .EnableSensitiveDataLogging()
);

builder.Services.AddRouting(options => options.LowercaseUrls = true);


var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

// app.UseHttpsRedirection();

// app.UseAuthorization();

app.MapControllers();

app.Run();