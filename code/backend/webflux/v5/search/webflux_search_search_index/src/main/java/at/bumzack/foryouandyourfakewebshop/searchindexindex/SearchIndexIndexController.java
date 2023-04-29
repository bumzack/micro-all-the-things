package at.bumzack.foryouandyourfakewebshop.searchindexindex;

import at.bumzack.common.search.SearchResult;
import at.bumzack.common.solr.SolrResponse;
import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.MovieSearchResult;
import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.SearchDoc;
import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.SearchMovieIndexRequest;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.http.HttpStatus;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.RequestPredicates;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.Loggers;

import static at.bumzack.foryouandyourfakewebshop.searchindexindex.model.SearchDoc.TYPE_REF_SEARCH_DOC;
import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;


@Controller("SearchIndexIndexController")
@CrossOrigin
public class SearchIndexIndexController {

    private static final Logger LOG = Loggers.getLogger(SearchIndexIndexController.class);

    private final SolrService solrService;

    public SearchIndexIndexController(final SolrService solrService) {
        this.solrService = solrService;
    }

    @NonNull
    public Mono<ServerResponse> searchIndex(final ServerRequest request) throws WebClientResponseException {
        return request.bodyToMono(SearchMovieIndexRequest.class)
                .flatMap(req -> {
                    if (nonNull(req.q())) {
                        LOG.info("search term found");
                        return solrService.searchByText(req.q(), req.limit(), req.offset(), TYPE_REF_SEARCH_DOC)
                                .flatMap(this::extractSearchDocList);
                    }
                    LOG.info("no term found provided");
                    return ServerResponse.status(HttpStatus.NOT_FOUND).body(BodyInserters.fromValue("not found"));
                });
    }

    private Mono<ServerResponse> extractSearchDocList(final SolrResponse<SearchDoc> res) {
        LOG.info("solr response {}", res);
        if (nonNull(res.getResponse())) {
            LOG.info("setting docs with {} items on SearchResult", res.getResponse().getDocs().size());
            final var result = new MovieSearchResult(res.getResponse().getDocs());
            return ServerResponse.ok().body(BodyInserters.fromValue(result));
        }
        LOG.info("returning 404");
        return ServerResponse.notFound().build();
    }

    @RouterOperations({
            @RouterOperation(path = "/solr/v1/solr/searchindex/search",
                    method = POST,
                    operation = @Operation(operationId = "searchDocs",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = SearchMovieIndexRequest.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = MovieSearchResult.class))),
                            })
            ),
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("/solr/v1/solr/searchindex/"),
                        builder -> builder
                                .POST("search", this::searchIndex)
                                .build())
                .build();
    }
}
