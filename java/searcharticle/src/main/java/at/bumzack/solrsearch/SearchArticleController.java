package at.bumzack.solrsearch;


import at.bumzack.solrsearch.dto.ArticleData;
import at.bumzack.solrsearch.dto.B2BArticleChannelInfoData;
import at.bumzack.solrsearch.dto.SearchRequestData;
import at.bumzack.solrsearch.dto.SearchResultProduct;
import at.bumzack.solrsearch.dto.XinetData;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.http.MediaType;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClient;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.RequestPredicates;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.annotation.NonNull;

import java.util.Collection;
import java.util.List;

import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SearchArticleController")
@CrossOrigin
public class SearchArticleController {

    private static final Logger LOG = reactor.util.Loggers.getLogger(SearchArticleController.class);

    private String getProductSearchCodeUrl() {
        return "http://localhost:8201/solr/search/product/code";
    }

    private String getXinetUrl() {
        return "http://localhost:8202/solr/search/xinet/";
    }

    @NonNull
    public Mono<ServerResponse> searchProduct(final ServerRequest request) throws WebClientResponseException {
        final var webClient = WebClient.create();
        return request.bodyToMono(SearchRequestData.class)
                .map(req -> fetchArticles(webClient, getProductSearchCodeUrl(), req).log("articles")
                        .flatMap(a -> findImages2(webClient, a).log("images")))
                .flatMap(articles -> ServerResponse.ok().body(articles, ArticleData.class))
                .switchIfEmpty(ServerResponse.notFound().build());
    }

    private Mono<ArticleData> findImages2(final WebClient webClient, final B2BArticleChannelInfoData a) {
        if (nonNull(a.getMainImageContainerQualifier())) {
            return fetchXinet(webClient, getXinetUrl(), a.getMainImageContainerQualifier())
                    .onErrorContinue(WebClientResponseException.NotFound.class, (e, o) -> {
                        final ArticleData article = (ArticleData) o;
                        LOG.error("No image found for {}", article.getArticle().getCode(), e.getMessage());
                    })
                    .doOnNext(img -> {
                        LOG.info("Found image for  article code  {}, mainImageContainerQualifier {}", a.getCode(), a.getMainImageContainerQualifier());
                        img.setURL("https://qstatic.rwa-test.at" + img.getURL());
                    })
                    .collectList()
                    .map(img -> convertToArticleData(a, img));
        }
        LOG.info("No  mainImageContainerQualifier found on article code  {}", a.getCode());
        return Mono.just(convertToArticleData(a, null));
    }

    private ArticleData convertToArticleData(final B2BArticleChannelInfoData a, final List<XinetData> img) {
        final var articleData = new ArticleData();
        articleData.setImages(img);
        articleData.setArticle(a);
        return articleData;
    }

    @NonNull
    public Mono<ServerResponse> searchText(final ServerRequest request) throws WebClientResponseException {
        final var webClient = WebClient.create();
        return request.bodyToMono(SearchRequestData.class)
                .map(req -> fetchArticles(webClient, getProductSearchCodeUrl(), req)
                        .flatMap(a -> findImages2(webClient, a)))
                .flatMap(articles -> ServerResponse.ok().body(articles, ArticleData.class))
                .switchIfEmpty(ServerResponse.notFound().build());
    }

    private Flux<B2BArticleChannelInfoData> fetchArticles(final WebClient webClient, final String url, final SearchRequestData requestData) {
        return webClient.post()
                .uri(url)
                .body(BodyInserters.fromValue(requestData))
                .accept(MediaType.APPLICATION_JSON)
                .retrieve()
                .bodyToMono(SearchResultProduct.class)
                .flatMapMany(e -> {
                    if (nonNull(e) && nonNull(e.getProducts())) {
                        return Flux.fromIterable(e.getProducts());
                    }
                    LOG.info("no articles found for requestData  code '{}' / text {}", requestData.getCode(), requestData.getText());
                    return Flux.empty();
                })
                .onErrorContinue(WebClientResponseException.NotFound.class, (e, o) -> {
                    // final ArticleData article = (ArticleData) o;
                    LOG.error("No article found for requestData.code {} / requestData.text {}", requestData.getCode(), requestData.getText());
                })
                .doOnNext(e -> {
                    LOG.info("article code found '{}'", e.getCode());
                });
    }

    private Flux<XinetData> fetchXinet(final WebClient webClient, final String url, final String imageContainerQualifier) {
        LOG.info("searching for xinet  imageContainerQualifier {}", imageContainerQualifier);
        return webClient.get()
                .uri(url + "/" + imageContainerQualifier)
                .accept(MediaType.APPLICATION_JSON)
                .retrieve()
                .bodyToFlux(XinetData.class);
    }

    @RouterOperations({
            @RouterOperation(path = "/solr/search/article/code",
                    method = POST,
                    operation = @Operation(operationId = "searchByArticleCode",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = SearchRequestData.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = Collection.class)))),
                            })
            ),
            @RouterOperation(path = "/solr/search/article/text",
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
                .nest(RequestPredicates.path("/solr/search/article"),
                        builder -> builder
                                .POST("code", this::searchProduct)
                                .POST("text", this::searchText)
                                .build())
                .build();
    }
}
