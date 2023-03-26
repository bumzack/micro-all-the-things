package at.bumzack.solr;


import at.bumzack.common.dto.ArticleData;
import at.bumzack.common.dto.Image;
import at.bumzack.common.dto.Product;
import at.bumzack.common.search.SearchRequestData;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.http.HttpStatusCode;
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
import reactor.util.Loggers;
import reactor.util.annotation.NonNull;
import at.bumzack.common.exception.NotificationException;
import java.util.List;

import static at.bumzack.common.dto.Image.TYPE_REF_SEARCH_RESULT_IMAGE;
import static at.bumzack.common.dto.Product.TYPE_REF_SEARCH_RESULT_PRODUCT;
import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrMovieWriterController")
@CrossOrigin
public class SolrMovieWriterController {

    private static final Logger LOG = Loggers.getLogger(SolrMovieWriterController.class);

    private String getProductSearchCodeUrl() {
        return "http://localhost:8201/solr/search/product/code";
    }

    private String getProductSearchTextUrl() {
        return "http://localhost:8201/solr/search/product/text";
    }

    private String getImageUrl() {
        return "http://localhost:8202/solr/search/image";
    }

    private String getImageSelectorUrl() {
        return "http://localhost:8400/image/desktop";
    }

    @NonNull
    public Mono<ServerResponse> addMovie(final ServerRequest request) throws WebClientResponseException {
        final var webClient = WebClient.create();
        return request.bodyToMono(at.bumzack.common.search.SearchRequestData.class)
                .map(req -> fetchArticles(webClient, getProductSearchCodeUrl(), req).log("search article: articles")
                        .flatMap(a -> findImages2(webClient, a).log("search article: find images")))
                .flatMap(articles -> ServerResponse.ok().body(articles, at.bumzack.common.dto. ArticleData.class))
                .log("search article: response")
                .switchIfEmpty(ServerResponse.notFound().build());
    }

    private Mono<ArticleData> findImages2(final WebClient webClient, final at.bumzack.common.dto.Product a) {
        if (nonNull(a.getMainImageContainerQualifier())) {
            return fetchImages(webClient, getImageUrl(), a.getMainImageContainerQualifier())
                    .onErrorContinue(WebClientResponseException.NotFound.class, (e, o) -> {
                        final ArticleData article = (ArticleData) o;
                        LOG.error("No image found for {}", article.getProduct().getCode(), e.getMessage());
                    })
                    .doOnNext(img -> {
                        LOG.info("Found image for  article code  {}, mainImageContainerQualifier {},   image = {}", a.getCode(), a.getMainImageContainerQualifier(), img);
                        img.setUrl("https://qstatic.rwa-test.at" + img.getUrl());
                    })
                    .switchIfEmpty(Flux.empty())
                    .log("after switch empty")
                    .doOnNext(i -> {
                        LOG.info("doOnNext   after switch empty   image ={} ", i);
                    })
                    .doOnError(i -> {
                        LOG.error("doOnError after switch empty   image ={} ", i);
                    })
                    .collectList()
                    .doOnNext(l -> {
                        LOG.info("got a list of images wth size {}", l.size());
                    })
                    .doOnError(i -> {
                        LOG.error("doOnError after collectList ", i);
                    })
                    .flatMapMany(images -> findDesktopImageFromService(webClient, images))
                    .doOnNext(l -> {
                        LOG.info("got ONE image after calling ImageSelectorService  image = {}", l);
                    })
                    .next()
                    .map(i -> convertToArticleData(a, i))
                    .doOnNext(l -> {
                        LOG.info("final ArticleData   = {}", l);
                    });
        }
        LOG.info("No  mainImageContainerQualifier found on article code  {}", a.getCode());
        return Mono.just(convertToArticleData(a, null));
    }

    private Mono<at.bumzack.common.dto.Image> findDesktopImageFromService(final WebClient webClient, final List<at.bumzack.common.dto.Image> i) {
        return webClient.post()
                .uri(getImageSelectorUrl())
                .body(BodyInserters.fromValue(i))
                .accept(MediaType.APPLICATION_JSON)
                .retrieve()
                .bodyToMono(at.bumzack.common.dto.Image.class)
                .onErrorContinue(WebClientResponseException.NotFound.class, (e, o) -> {
                    // final ArticleData article = (ArticleData) o;
                    LOG.error("No image found for requestData.code {} / requestData.text {}");
                })
                .doOnNext(e -> {
                    LOG.info("image with code found '{}'     image: {}", e.getCode(), e);
                });
    }

    private ArticleData convertToArticleData(final Product product, final Image img) {
        final var articleData = new ArticleData();
        LOG.info("convertToArticleData    article with code found '{}'     image with code: {}", product.getCode(), nonNull(img) ? img.getCode() : "no image found");
        articleData.setImage(img);
        articleData.setProduct(product);
        return articleData;
    }

    @NonNull
    public Mono<ServerResponse> searchText(final ServerRequest request) throws WebClientResponseException {
        final var webClient = WebClient.create();
        return request.bodyToMono(SearchRequestData.class)
                .log("request data")
                .doOnNext(d -> {
                    LOG.info("got request data {}", d);
                })
                .onErrorContinue((ex, obj) -> {
                    LOG.error("error {}", ex);
                    LOG.error("obj {}", obj);
                })
                .map(req -> {
                    LOG.info("starting with req {}", req);
                    return fetchArticles(webClient, getProductSearchTextUrl(), req)
                            .doOnNext(p -> {
                                LOG.info("article found. code {}", p.getCode());
                            })
                            .onErrorContinue((ex, obj) -> {
                                LOG.error("error {}", ex);
                                LOG.error("obj {}", obj);
                            })
                            .log("fetch articles")
                            .flatMap(a -> {
                                LOG.info("finding images for article {} with mainImageContainerQualifier {}", a.getCode(), a.getMainImageContainerQualifier());
                                return findImages2(webClient, a).log("findimages 2")
                                        .doOnNext(article -> {
                                            LOG.info("product and image found. article.code {}", article.getProduct().getCode());
                                        });
                            });
                })
                .flatMap(articles -> ServerResponse.ok().body(articles, ArticleData.class)).log(LOG)
                .switchIfEmpty(ServerResponse.notFound().build());
    }

    private Flux<Product> fetchArticles(final WebClient webClient, final String url, final SearchRequestData requestData) {
        webClient.post()
                .uri(url)
                .body(BodyInserters.fromValue(requestData))
                .accept(MediaType.APPLICATION_JSON)
                .retrieve()
                .onStatus(HttpStatusCode::isError, clientResponse -> {
                    LOG.error("client response {}", clientResponse);
                    return Mono.error(NotificationException::new);
                })
                .bodyToMono(String.class)
                .subscribe(response -> {
                    LOG.info("the returned value  {}", response);
                });
        LOG.info("URL  {} ", url);

        return webClient.post()
                .uri(url)
                .body(BodyInserters.fromValue(requestData))
                .accept(MediaType.APPLICATION_JSON)
                .retrieve()
                .bodyToMono(TYPE_REF_SEARCH_RESULT_PRODUCT)
                .onErrorContinue((ex, obj) -> {
                    LOG.error("error while retrieving the products from solr  obj ", obj);
                    LOG.error("error while retrieving the products from solr  ex ", ex);
                })
                .doOnNext(res -> {
                    LOG.info("Solr response fetchArticles  '{}'", res.toString());
                })
                .flatMapMany(e -> {
                    // wtf ?
                    LOG.info("SolrResponseResponse<Product> {}", e);
                    if (nonNull(e) && nonNull(e.getItems())) {
                        return Flux.fromIterable(e.getItems());
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

    private Flux<Image> fetchImages(final WebClient webClient, final String url, final String imageContainerQualifier) {
        LOG.info("searching for image  imageContainerQualifier {}", imageContainerQualifier);

        final String uri = url + "/" + imageContainerQualifier;
        webClient.get()
                .uri(uri)
                .accept(MediaType.APPLICATION_JSON)
                .retrieve()
                .bodyToMono(String.class)
                .subscribe(s -> {
                    LOG.info("response from {} -> {}", uri, s);
                });

        final Flux<Image> objectFlux = webClient.get()
                .uri(url + "/" + imageContainerQualifier)
                .accept(MediaType.APPLICATION_JSON)
                .retrieve()
                .bodyToMono(TYPE_REF_SEARCH_RESULT_IMAGE)
                .flatMapMany(e -> {
                    // wtf ?
                    LOG.info("SolrResponseResponse<Image> {}", e);
                    if (nonNull(e) && nonNull(e.getItems())) {
                        return Flux.fromIterable(e.getItems());
                    }
                    LOG.info("no images found for imageContainerQualifier {}", imageContainerQualifier);
                    return Flux.empty();
                })
                .onErrorContinue(WebClientResponseException.NotFound.class, (e, o) -> {
                    LOG.error("ERROR    no image found for imageContainerQualifier {}", imageContainerQualifier);
                })
                .doOnNext(e -> {
                    LOG.info("image found '{}'", e.getCode());
                });
        return objectFlux;
    }

    @RouterOperations({
            @RouterOperation(path = "/solr/movie",
                    method = POST,
                    operation = @Operation(operationId = "addMovieToSolr",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = SearchRequestData.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = ArticleData.class)))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("/solr/"),
                        builder -> builder
                                .POST("movie", this::addMovie)
                                 .build())
                .build();
    }
}
