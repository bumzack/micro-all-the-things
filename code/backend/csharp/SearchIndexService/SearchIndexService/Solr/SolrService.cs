namespace SearchIndexService.Solr;

public class SolrService
{
    private string _solrHost = "solr01.bumzack.at";

    private int _solrPort = 80;

    public async Task<SolrResponse<SearchDoc>> SearchByText(string searchText, int limit, int offset)
    {
        IEnumerable<string> fieldNames = new List<string>
        {
            "tconst",
            "titles",
            "actors",
            "directors",
            "genres",
            "characters"
        };

        IEnumerable<string> queries = fieldNames.Select(fn => { return string.Join(":", fn, searchText); });

        Console.WriteLine("queries  " + queries);
        string query = String.Join(" OR ", queries);
        Console.WriteLine("put together the query " + query);


        var builder = new SolrRequestBuilder()
            .SetCoreName("searchindex")
            .SetQuery(query)
            .SetOffset(offset)
            .SetLimit(limit);

        builder.SetHost(_solrHost);
        builder.SetPort(_solrPort);

        var url = builder
            .Build();

        Console.WriteLine("full Solr Request URL "+ url);
        var ex = new SolrRequestExecutor();

        return await ex.GetAsync(url);
    }
}