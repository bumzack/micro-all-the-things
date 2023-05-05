using System.Net;
using Newtonsoft.Json;

namespace SearchIndexService.Solr;

public class SolrRequestExecutor
{

    public SolrRequestExecutor()
    {
        
    }

    public   async Task<SolrResponse<SearchDoc>> GetAsync(string uri)
    {
        using var client = new HttpClient();
        try
        {
            HttpResponseMessage response = await client.GetAsync(uri);

            var responseContent = await response.Content.ReadAsStringAsync();
            var user = JsonConvert.DeserializeObject<SolrResponse<SearchDoc>>(responseContent);

            return user;
        }
        catch (Exception ex)
        {
            Console.WriteLine("an error occurred while requesting Solr " +ex.Message);
            return null;
        }
    }
}