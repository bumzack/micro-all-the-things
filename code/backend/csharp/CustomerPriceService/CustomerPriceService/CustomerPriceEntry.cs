using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace CustomerService;


[Table("customer_price")]
public class CustomerPriceEntry
{

    [Key, Required]
    public long Id { get; set; }

    public long CustomerId  { get; set; }

    public double? Discount { get; set; }
    
    public int? StartYear { get; set; }

    public int? EndYear { get; set; }

    public DateTime? Created { get; set; }
}
