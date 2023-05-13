namespace SearchIndexService;


public class SearchDoc
{
    public string tconst { get; set; }

    public List<string>? Titles { get; set; }
    public List<string>? Actors { get; set; }
    public List<string>? Directors { get; set; }
    public List<string>? Genres { get; set; }
    public List<string>? Characters { get; set; }

    public int? RuntimeMinutes { get; set; }
    public int? Year { get; set; }

    public bool? Adult { get; set; }
    public string? TitleType { get; set; }
}