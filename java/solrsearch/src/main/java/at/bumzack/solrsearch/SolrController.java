package at.bumzack.solrsearch;

import at.bumzack.solrsearch.dto.SearchRequestData;
import at.bumzack.solrsearch.dto.SearchResult;
import at.bumzack.solrsearch.solr.SolrResponse;
import at.bumzack.solrsearch.solr.SolrService;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.http.HttpStatus;
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
import reactor.util.annotation.NonNull;

import java.util.Collection;

import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrController")
@CrossOrigin
public class SolrController {

    private static final Logger LOG = reactor.util.Loggers.getLogger(SolrController.class);

    private final SolrService solrService;

    public SolrController(final SolrService solrService) {
        this.solrService = solrService;
    }

    @NonNull
    public Mono<ServerResponse> searchArticleCode(final ServerRequest request) throws WebClientResponseException {
        return request.bodyToMono(SearchRequestData.class)
                .flatMap(requestData -> {
                    if (nonNull(requestData.getCode())) {
                        return solrService.searchByCode(requestData.getText(), requestData.getPageSize())
                                .flatMap(this::extractArticleList);
                    }
                    return ServerResponse.status(HttpStatus.NOT_FOUND).body(BodyInserters.fromValue("not found"));
                });
    }

    private Mono<ServerResponse> extractArticleList(final SolrResponse res) {
        final var result = new SearchResult();
        if (nonNull(res.getResponse())) {
            result.setArticles(res.getResponse().getDocs());
        }
        return ServerResponse.ok().body(BodyInserters.fromValue(result));
    }

    @NonNull
    public Mono<ServerResponse> searchText(final ServerRequest request) throws WebClientResponseException {
        return request.bodyToMono(SearchRequestData.class)
                .flatMap(requestData -> {
                    if (nonNull(requestData.getCode())) {
                        return solrService.searchByCode(requestData.getText(), requestData.getPageSize())
                                .flatMap(this::extractArticleList);
                    }
                    return ServerResponse.status(HttpStatus.NOT_FOUND).body(BodyInserters.fromValue("not found"));
                });
    }

    @RouterOperations({
            @RouterOperation(path = "/solr/search/articlecode",
                    method = POST,
                    operation = @Operation(operationId = "searchByArticleCode",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = SearchRequestData.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = Collection.class)))),
                            })
            ),
            @RouterOperation(path = "/solr/search/text",
                    method = POST,
                    operation = @Operation(operationId = "searchByText",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = SearchRequestData.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = Collection.class)))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("/solr/search"),
                        builder -> builder
                                .POST("searchByArticleCode", this::searchArticleCode)
                                .POST("searchByText", this::searchText)
                                .build())
                .build();
    }
}
