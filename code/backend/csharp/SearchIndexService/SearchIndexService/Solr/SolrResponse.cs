using Newtonsoft.Json;

namespace SearchIndexService.Solr;

public class SolrResponse<DOC_TYPE>
{
    [JsonProperty(PropertyName = "responseHeader")]
    public  SolrResponseHeader ResponseHeader  { get; set; }

    [JsonProperty(PropertyName = "response")]
    public SolrResponseResponse<DOC_TYPE> Response { get; set; }

}