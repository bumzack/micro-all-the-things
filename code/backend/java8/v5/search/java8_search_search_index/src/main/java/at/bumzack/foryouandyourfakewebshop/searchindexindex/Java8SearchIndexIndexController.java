package at.bumzack.foryouandyourfakewebshop.searchindexindex;

import at.bumzack.common.solr.SolrResponse;
import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.MovieSearchResult;
import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.SearchDoc;
import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.SearchMovieIndexRequest;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import reactor.util.Logger;
import reactor.util.Loggers;

import static at.bumzack.foryouandyourfakewebshop.searchindexindex.model.SearchDoc.TYPE_REF_SEARCH_DOC;
import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.POST;


@Controller("Java8SearchIndexIndexController")
@CrossOrigin
public class Java8SearchIndexIndexController {

    private static final Logger LOG = Loggers.getLogger(Java8SearchIndexIndexController.class);

    private final SolrService solrService;

    public Java8SearchIndexIndexController(final SolrService solrService) {
        this.solrService = solrService;
    }

    @RouterOperation(path = "/solr/v1/solr/searchindex/search",
            method = POST,
            operation = @Operation(operationId = "searchDocs",
                    requestBody = @io.swagger.v3.oas.annotations.parameters.RequestBody(content = @Content(schema = @Schema(implementation = SearchMovieIndexRequest.class))),
                    responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = MovieSearchResult.class))),
                    })
    )
    @NonNull
    @ResponseBody
    @PostMapping(value = "/solr/v1/solr/searchindex/search", produces = MediaType.APPLICATION_JSON_VALUE)
    public ResponseEntity<MovieSearchResult> searchDocs(@RequestBody final SearchMovieIndexRequest req) throws WebClientResponseException {
        LOG.info("searchDocs  request  {}", req);
        final SolrResponse<SearchDoc> res = solrService.searchByText(req.q(), req.limit(), req.offset(), TYPE_REF_SEARCH_DOC);
        final MovieSearchResult response = extractSearchDocList(res);
        if (nonNull(response)) {
            return ResponseEntity.ok(response);
        }
        return ResponseEntity.notFound().build();
    }

    private MovieSearchResult extractSearchDocList(final SolrResponse<SearchDoc> res) {
        LOG.info("solr response {}", res);
        if (nonNull(res.getResponse())) {
            LOG.info("setting docs with {} items on SearchResult", res.getResponse().getDocs().size());
            final MovieSearchResult movieSearchResult = new MovieSearchResult();
            movieSearchResult.setMovies(res.getResponse().getDocs());
            return movieSearchResult;
        }
        return null;
    }
}
