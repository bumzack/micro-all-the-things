using SearchIndexService;
using Microsoft.EntityFrameworkCore;
using SearchIndexService.Solr;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.

builder.Services.AddControllers();
// Learn more about configuring Swagger/OpenAPI at https://aka.ms/aspnetcore/swashbuckle
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();
// builder.Services.AddDbContext<PriceDataContext>(o =>
//     o.UseNpgsql(builder.Configuration.GetConnectionString("Ef_Postgres_Db"))
//         .UseSnakeCaseNamingConvention()
//         .UseLoggerFactory(LoggerFactory.Create(builder => builder.AddConsole()))
//         .EnableSensitiveDataLogging()
// );
//
// builder.Services.AddRouting(options => options.LowercaseUrls = true);


builder.Services.AddScoped<SolrService, SolrService>();

builder.Logging.ClearProviders();
builder.Logging.AddConsole();

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