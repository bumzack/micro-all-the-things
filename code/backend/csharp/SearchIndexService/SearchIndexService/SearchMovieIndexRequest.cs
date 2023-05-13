namespace SearchIndexService;

public class SearchMovieIndexRequest
{
    public int Offset { get; set; }

    public int Limit { get; set; }

    public string Q { get; set; }
}
