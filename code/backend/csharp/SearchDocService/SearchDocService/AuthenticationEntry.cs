using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace SearchDocService;


[Table("authentication")]
public class AuthenticationEntry
{
    [Key, Required]
    public long Id { get; set; }

    public long CustomerId { get; set; }
    public string? Jwt  { get; set; }
    
    public DateTime? Created { get; set; }
    public DateTime? LoggedIn { get; set; }
    public DateTime? LoggedOut { get; set; }
    
}
