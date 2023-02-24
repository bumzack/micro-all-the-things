package at.bumzack.pimdbreader;

import at.bumzack.pimdbreader.dto.B2BArticleChannelInfoData;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.http.HttpStatus;
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

@Controller("SolrController")
public class PimProductsDbReaderController {
    private static final Logger LOG = reactor.util.Loggers.getLogger(PimProductsDbReaderController.class);

    // curl http://localhost:8091/solr/articles/0/5/12

    @NonNull
    public Mono<ServerResponse> readProductsFromDbAndSendToSolrWriter(final ServerRequest request) throws WebClientResponseException {
        final var start = Integer.parseInt(request.pathVariable("start"));
        final var pageSize = Integer.parseInt(request.pathVariable("pagesize"));
        final var limit = Integer.parseInt(request.pathVariable("limit"));
        LOG.info("findArticlesSolr   start {}, pagesize {},  limit {}", start, pageSize, limit);

        return execRequest(start, pageSize, limit);
    }

    private Mono<ServerResponse> execRequest(final int start, final int pageSize, final int limit) {
        final var webClient = WebClient.create();
        return fetchArticles(webClient, pageSize, start)
                .expand(cnt -> {
                    // TODO fix infinite loop if limit > max_cnt of entries
                    LOG.info("cnt {} of limit {} (limit)", cnt, limit);
                    if (cnt >= limit || cnt == 0) {
                        return Mono.empty();
                    }
                    if (cnt + pageSize > limit) {
                        return fetchArticles(webClient, limit - cnt, cnt)
                                .map(size -> size + cnt);
                    }
                    return fetchArticles(webClient, pageSize, cnt)
                            .map(size -> size + cnt);
                })
                .onErrorReturn(-1)
                .collectList()
                .flatMap(articles -> {
                    LOG.info("execRequest done");
                    final long count = articles.stream().filter(res -> res < 0).count();
                    if (count > 0) {
                        LOG.error("Products {} error(s) occurred during the indexing ", count);
                        return ServerResponse.status(HttpStatus.INTERNAL_SERVER_ERROR)
                                .contentType(MediaType.TEXT_PLAIN)
                                .body(BodyInserters.fromValue("products " + count + " error(s) occurred during the indexing"));
                    }
                    LOG.info("{} products successfully indexed", articles.size());
                    return ServerResponse.ok().body(BodyInserters.fromValue("cnt: " + articles + " products processed"));
                });
    }

    private static String getUrl(final int start, final int pageSize) {
        final var s = "start=" + start;
        final var ps = "pagesize=" + pageSize;
        return "https://localhost:8002/rwaportalwebservices/public/products/readpaginated?" + s + "&" + ps;
    }

    private Mono<Integer> fetchArticles(final WebClient webClient, final int pageSize, final int start) {
        final String url = getUrl(start, pageSize);

        return webClient.get()
                .uri(url)
                .accept(MediaType.APPLICATION_JSON)
                .retrieve()
                .bodyToFlux(B2BArticleChannelInfoData.class)
                .doOnNext(a -> a.setId(a.getCode()))
                .flatMap(this::sendToSolrWriter)
                .onErrorReturn("error processing the products")
                .doOnNext(e -> {
                    LOG.info("articles {}", String.join("/", e));
                })
                .collectList()
                .map(List::size);
    }

    private Mono<String> sendToSolrWriter(final B2BArticleChannelInfoData data) {
        final SolrWriter solrWriter = new SolrWriter("localhost", "8100", "/solr/product");
//        LOG.info("URL of Solr Writer {}", solrWriter.getRequestUrl());
        final var webClient = WebClient.create();
        LOG.info("sendToSolrWriter  data.code {} ", data.getCode());

        return webClient.post()
                .uri(solrWriter.getRequestUrl())
                .contentType(MediaType.APPLICATION_JSON)
                .body(BodyInserters.fromValue(data))
                .retrieve()
                .bodyToMono(String.class)
                //.doOnNext(e -> LOG.info("article  {}", e))
                .doOnError(e -> LOG.error("products error {}", e.getMessage()));
    }

    @RouterOperations({
            @RouterOperation(path = "/solr/products/{start}/{pagesize}/{limit}",
                    method = GET,
                    operation = @Operation(operationId = "products",
                            parameters = {@Parameter(name = "start", schema = @Schema(description = "start", type = "int")),
                                    @Parameter(name = "pagesize", schema = @Schema(description = "pagesize", type = "int")),
                                    @Parameter(name = "limit", schema = @Schema(description = "limit", type = "int"))
                            },
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = Collection.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("/solr"),
                        builder -> builder
                                .GET("products/{start}/{pagesize}/{limit}", this::readProductsFromDbAndSendToSolrWriter)
                                .build())
                .build();
    }
}
