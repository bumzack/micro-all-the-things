using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace PriceService;


[Table("price")]
public class Price
{
    [Key, Required]
    public long Id { get; set; }

    public string MovieTconst  { get; set; }

    public double Amount { get; set; }
    
    public DateTime? Created { get; set; }
}
