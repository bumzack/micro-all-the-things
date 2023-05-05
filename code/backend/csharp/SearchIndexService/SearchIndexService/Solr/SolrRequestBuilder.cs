namespace SearchIndexService.Solr;

public class SolrRequestBuilder
{
    private static readonly string Solr = "solr";
    private readonly List<string> _responseFields = new List<string>();
    private readonly List<string> _queryField = new List<string>();

    private string _solrHost = "localhost";
    private int _solrPort = 80;
    private string _core = "searchindex";
    private readonly string _command = "select";

    private string? _query = null;
    private int _offset = 25;
    private int _limit = 25;

    public SolrRequestBuilder()
    {
    }


    public SolrRequestBuilder SetQuery(string query)
    {
        this._query = query;
        return this;
    }

    public SolrRequestBuilder AddResponseField(string fieldName)
    {
        _responseFields.Add(fieldName);
        return this;
    }

    public SolrRequestBuilder AddResponseFields(List<string> fieldNames)
    {
        _responseFields.AddRange(fieldNames);
        return this;
    }

    public SolrRequestBuilder AddQueryField(string name, string value)
    {
        _queryField.Add(string.Join(":", name, value));
        return this;
    }

    public SolrRequestBuilder SetCoreName(string core)
    {
        this._core = core;
        return this;
    }

    public SolrRequestBuilder SetPort(int port)
    {
        this._solrPort = port;
        return this;
    }

    public SolrRequestBuilder SetCommand(string command)
    {
        this._core = command;
        return this;
    }


    public SolrRequestBuilder SetOffset(int offset)
    {
        this._offset = offset;
        return this;
    }

    public SolrRequestBuilder SetLimit(int limit)
    {
        this._limit = limit;
        return this;
    }

    public SolrRequestBuilder SetHost(string host)
    {
        this._solrHost = host;
        return this;
    }

    public string Build()
    {
        var host = $"http://{_solrHost}:{_solrPort}/";
        Console.WriteLine($"host:  {host}");

        var url = $"{host}{Solr}/{_core}/{_command}";

        if (_query != null)
        {
            _query = "q=" + _query;
        }

        var fq = String.Join("&", _queryField.Select(f => { return $"fq={f}"; }));

        var fl = String.Join(",", _responseFields);
        var limit = $"limit={this._limit}";
        var offset = $"offset={this._offset}";

        var urlParams = $"{_query}&{fl}&{limit}&{offset}&{fq}";
            
        Console.WriteLine("urlParams "+ urlParams);
        
        return String.Join("?", url, urlParams);

        //  var q = Optional.of(query).map(q1->stringUtils.join("q=", q1)).orElse(null);
        //  var fq = queryField.stream()
        //     .map(fieldQuery->stringUtils.join("fq", "=", fieldQuery))
        //     .collect(toList());
        //  var fl = stringUtils.join("fl", "=", string.join(",", responseFields));
        //  var limit = stringUtils.join("limit", "=", string.valueOf(this.limit));
        //  var offset = stringUtils.join("offset", "=", String.valueOf(this.offset));
        //
        // LOG.info("\nurl         \n{}\n", url);
        // LOG.info("\nq           {}", q);
        // LOG.info("\nfq          {}", String.join(" // ", fq));
        // LOG.info("\nfl          {}", fl);
        //
        //  var urlParams = Stream.concat(Stream.of(q, fl, limit, offset), fq.stream())
        //     .map(StringUtils::trimToNull)
        //     .filter(Objects::nonNull)
        //     .collect(Collectors.joining(AMPERSAND));

    }
}