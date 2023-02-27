package at.bumzack.solrsearch;

import at.bumzack.dto.Category;
import at.bumzack.search.SearchRequestData;
import at.bumzack.search.SearchResult;
import at.bumzack.solr.SolrResponse;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.core.ParameterizedTypeReference;
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

import static at.bumzack.dto.Category.TYPE_REF_CATERORY;
import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.GET;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrCategoryController")
@CrossOrigin
public class SolrCategorySearchController {

    private static final Logger LOG = reactor.util.Loggers.getLogger(SolrCategorySearchController.class);

    private final SolrService solrService;

    public SolrCategorySearchController(final SolrService solrService) {
        this.solrService = solrService;
    }

    @NonNull
    public Mono<ServerResponse> searchCategoryByCode(final ServerRequest request) throws WebClientResponseException {

        return request.bodyToMono(SearchRequestData.class)
                .flatMap(requestData -> {
                    if (nonNull(requestData.getCode())) {
                        return solrService.searchByCode(requestData.getCode(), requestData.getPageSize(), TYPE_REF_CATERORY)
                                .flatMap(this::extractCategoryList);
                    }
                    return ServerResponse.status(HttpStatus.NOT_FOUND).body(BodyInserters.fromValue("not found"));
                });
    }

    private Mono<ServerResponse> extractCategoryList(final SolrResponse<Category> res) {
        final var result = new SearchResult<Category>();
        if (nonNull(res.getResponse())) {
            result.setItems(res.getResponse().getDocs());
        }
        return ServerResponse.ok().body(BodyInserters.fromValue(result));
    }

    @NonNull
    public Mono<ServerResponse> all(final ServerRequest request) throws WebClientResponseException {
        final ParameterizedTypeReference<SolrResponse<Category>> typeRef = new ParameterizedTypeReference<SolrResponse<Category>>() {
        };
        return solrService.allCategories(typeRef)
                .flatMap(this::extractCategoryList);
    }

    @NonNull
    public Mono<ServerResponse> rootCategories(final ServerRequest request) throws WebClientResponseException {
        final ParameterizedTypeReference<SolrResponse<Category>> typeRef = new ParameterizedTypeReference<SolrResponse<Category>>() {
        };
        return solrService.rootCategories(typeRef)
                .flatMap(this::extractCategoryList);
    }

    @NonNull
    public Mono<ServerResponse> children(final ServerRequest request) throws WebClientResponseException {
        final ParameterizedTypeReference<SolrResponse<Category>> typeRef = new ParameterizedTypeReference<SolrResponse<Category>>() {
        };
        final var code = request.queryParam("code").orElse(null);
        return solrService.children(code, typeRef)
                .flatMap(this::extractCategoryList);
    }

    @RouterOperations({
            @RouterOperation(path = "/solr/search/category/code",
                    method = POST,
                    operation = @Operation(operationId = "searchByCategoryCode",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = SearchRequestData.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = SearchResult.class)))),
                            })
            ),
            @RouterOperation(path = "/solr/search/category",
                    method = GET,
                    operation = @Operation(operationId = "allCategories",
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = SearchResult.class)))),
                            })
            ),
            @RouterOperation(path = "/solr/search/category/root",
                    method = GET,
                    operation = @Operation(operationId = "rootCategories",
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = SearchResult.class)))),
                            })
            ),
            @RouterOperation(path = "/solr/search/category/children/{code}",
                    method = GET,
                    operation = @Operation(operationId = "childCategories",
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = SearchResult.class)))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("/solr/search/category"),
                        builder -> builder
                                .POST("/code", this::searchCategoryByCode)
                                .GET("", this::all)
                                .GET("/children/{code}", this::children)
                                .GET("/root", this::rootCategories).build())
                .build();
    }
}
