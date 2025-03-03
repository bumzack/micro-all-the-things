using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace CustomerService;


[Table("customer")]
public class Customer
{

    [Key, Required]
    public long Id { get; set; }

    public string? FirstName  { get; set; }

    public string? LastName { get; set; }
    
    public string Email { get; set; }

    public string Password { get; set; }

    public DateTime? Created { get; set; }
}
