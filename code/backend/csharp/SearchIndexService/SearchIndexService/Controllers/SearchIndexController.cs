using Microsoft.AspNetCore.Mvc;
using SearchIndexService.Solr;

namespace SearchIndexService.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class SearchIndexController : ControllerBase
    {
        private SolrService _solrService;
        private readonly ILogger _logger;

        public SearchIndexController(SolrService solrService, ILogger<SearchIndexController> logger)
        {
            _solrService = solrService;
            _logger = logger;
        }

        [HttpPost("/solr/v1/solr/searchindex")]
        public async Task<ActionResult<MovieSearchResult>> GetPrice(SearchMovieIndexRequest req)
        {
            _logger.LogInformation("got a request q {}, limit {}, offset {}", req.Q, req.Limit, req.Offset);
            var docs = await _solrService.SearchByText(req.Q, req.Limit, req.Offset);

            var response = new MovieSearchResult
            {
                Movies = docs.Response.Docs
            };

            return response;
        }
    }
}