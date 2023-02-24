package at.bumzack.pimdbreader;

import at.bumzack.pimdbreader.dto.CategoryData;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.http.MediaType;
import org.springframework.stereotype.Controller;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClient;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.RequestPredicates;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.annotation.NonNull;

import java.util.Collection;
import java.util.List;

import static org.springframework.web.bind.annotation.RequestMethod.GET;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrCategoryController")
public class PimCategoryDbReaderController {
    private static final Logger LOG = reactor.util.Loggers.getLogger(PimCategoryDbReaderController.class);

    // curl http://localhost:8080/solr/categories/0/5

    @NonNull
    public Mono<ServerResponse> readCategoriesFromDbAndSendToSolrWriter(final ServerRequest request) throws WebClientResponseException {
        LOG.info("start    private String code");
        return execRequest( );
    }

    private Mono<ServerResponse> execRequest() {
        final var webClient = WebClient.create();
        return fetchArticles(webClient )
                .onErrorReturn(-1)
                .flatMap(articles -> {
                    LOG.info("execRequest done");
                    return ServerResponse.ok().body(BodyInserters.fromValue("cnt: " + articles + " categories processed"));
                });
    }

    private  String getUrl( ) {
        return "https://localhost:8002/rwaportalwebservices/public/categories";
    }

    private Mono<Integer> fetchArticles(final WebClient webClient) {
        final String url = getUrl();

        return webClient.get()
                .uri(url)
                .accept(MediaType.APPLICATION_JSON)
                .retrieve()
                .bodyToFlux(CategoryData.class)
                .doOnNext(a -> a.setId(a.getCode()))
                .flatMap(this::sendToSolrWriter)
                .onErrorReturn("error processing the articles")
                .doOnNext(e -> {
                    LOG.info("articles {}", String.join("/", e));
                })
                .collectList()
                .map(List::size);
    }

    private Mono<String> sendToSolrWriter(final CategoryData data) {
        final SolrWriter solrWriter = new SolrWriter("localhost", "8100", "/solr/category");
//        LOG.info("URL of Solr Writer {}", solrWriter.getRequestUrl());
        final var webClient = WebClient.create();
        LOG.info("sendToSolrWriter  category.code {} ", data.getCode());

        return webClient.post()
                .uri(solrWriter.getRequestUrl())
                .contentType(MediaType.APPLICATION_JSON)
                .body(BodyInserters.fromValue(data))
                .retrieve()
                .bodyToMono(String.class)
                //.doOnNext(e -> LOG.info("article  {}", e))
                .doOnError(e -> LOG.error("category error {}", e.getMessage()));
    }

    @RouterOperations({
            @RouterOperation(path = "/solr/categories",
                    method = GET,
                    operation = @Operation(operationId = "categories",
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = Collection.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("/solr"),
                        builder -> builder
                                .GET("categories", this::readCategoriesFromDbAndSendToSolrWriter)
                                .build())
                .build();
    }
}
