package at.bumzack.foryouandyourfakewebshop.searcharticle;

import at.bumzack.common.search.SearchRequestData;
import org.springframework.http.HttpStatus;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.Loggers;

import static java.util.Objects.nonNull;


@Controller("SearchIndexArticleController")
@CrossOrigin
public class SearchIndexArticleController {

    private static final Logger LOG = Loggers.getLogger(SearchIndexArticleController.class);

    private final SolrService solrService;

    public SearchIndexArticleController(final SolrService solrService) {
        this.solrService = solrService;
    }

//    @NonNull
//    public Mono<ServerResponse> searchProductCode(final ServerRequest request) throws WebClientResponseException {
//        return request.bodyToMono(SearchRequestData.class)
//                .flatMap(requestData -> {
//                    if (nonNull(requestData.getCode())) {
//                        return solrService.searchByCode(requestData.getCode(), requestData.getPageSize(), TYPE_REF_PRODUCT)
//                                .flatMap(this::extractProductList);
//                    }
//                    return ServerResponse.status(HttpStatus.NOT_FOUND).body(BodyInserters.fromValue("not found"));
//                });
//    }
//
//    private Mono<ServerResponse> extractProductList(final SolrResponse<Product> res) {
//        final var result = new SearchResult<Product>();
//        LOG.info("solr response {}", res);
//        if (nonNull(res.getResponse())) {
//            result.setItems(res.getResponse().getDocs());
//            return ServerResponse.ok().body(BodyInserters.fromValue(result));
//        }
//        return ServerResponse.notFound().build();
//    }
//
//    @NonNull
//    public Mono<ServerResponse> searchText(final ServerRequest request) throws WebClientResponseException {
//        return request.bodyToMono(SearchRequestData.class)
//                .flatMap(requestData -> {
//                    if (nonNull(requestData.getText())) {
//                        return solrService.searchTexts(requestData.getText(), requestData.getPageSize(), TYPE_REF_PRODUCT)
//                                .doOnNext(res->{
//                                    LOG.info("response from solr {}", res);
//                                })
//                                .flatMap(this::extractProductList);
//                    }
//                    return ServerResponse.status(HttpStatus.NOT_FOUND).body(BodyInserters.fromValue("not found"));
//                });
//    }
//
//    @RouterOperations({
//            @RouterOperation(path = "/solr/search/product/code",
//                    method = POST,
//                    operation = @Operation(operationId = "searchProductCode",
//                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = SearchRequestData.class))),
//                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = SearchResult.class)))),
//                            })
//            ),
//            @RouterOperation(path = "/solr/search/text",
//                    method = POST,
//                    operation = @Operation(operationId = "searchByText",
//                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = SearchRequestData.class))),
//                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = SearchResult.class)))),
//                            })
//            )
//    })
//
//    @Bean
//    public RouterFunction<ServerResponse> solrRoutes() {
//        return route()
//                .nest(RequestPredicates.path("/solr/search/product"),
//                        builder -> builder
//                                .POST("code", this::searchProductCode)
//                                .POST("text", this::searchText)
//                                .build())
//                .build();
//    }
}
