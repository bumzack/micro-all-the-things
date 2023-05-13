using Newtonsoft.Json;

namespace SearchIndexService.Solr;

public class SolrResponseHeader
{
    public int Status { get; set; }
    public int QTime { get; set; }

    [JsonProperty(PropertyName = "params")]
    public SolrParams pparams { get; set; }
}