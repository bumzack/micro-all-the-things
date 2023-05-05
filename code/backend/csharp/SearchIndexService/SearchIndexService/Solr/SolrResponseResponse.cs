using Newtonsoft.Json;

namespace SearchIndexService.Solr;

public class SolrResponseResponse<DOC_TYPE>
{
    [JsonProperty(PropertyName = "numFound")]
    public int? NumFound { get; set; }

    [JsonProperty(PropertyName = "start")] 
    public int? Start { get; set; }

    [JsonProperty(PropertyName = "docs")] 
    public List<DOC_TYPE> Docs { get; set; }
}