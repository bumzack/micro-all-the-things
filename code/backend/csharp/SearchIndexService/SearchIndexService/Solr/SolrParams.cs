namespace SearchIndexService.Solr;

public class SolrParams
{
    public string? q { get; set; }
    public string? fl { get; set; }
    public List<string>? fq { get; set; }

    public int? rows { get; set; }
    //  public in timeStamp;
}