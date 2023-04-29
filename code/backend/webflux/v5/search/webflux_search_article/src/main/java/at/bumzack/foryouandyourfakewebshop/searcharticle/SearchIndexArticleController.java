package at.bumzack.foryouandyourfakewebshop.searcharticle;

import at.bumzack.authenticationservice.model.AuthenticationEntry;
import at.bumzack.customerpriceservice.model.CustomerPriceEntry;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.ArticleSearchResult;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.SearchArticleRequest;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.SearchArticleResponse;
import at.bumzack.priceservice.model.PriceEntry;
import at.bumzack.searchindexservice.ApiClient;
import at.bumzack.searchindexservice.api.DefaultApi;
import at.bumzack.searchindexservice.model.MovieSearchResult;
import at.bumzack.searchindexservice.model.SearchDoc;
import at.bumzack.searchindexservice.model.SearchMovieIndexRequest;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Controller;
import org.springframework.util.CollectionUtils;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.RequestPredicates;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;
import reactor.core.scheduler.Schedulers;
import reactor.util.Logger;
import reactor.util.Loggers;

import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.stream.Collectors;

import static at.bumzack.foryouandyourfakewebshop.searcharticle.ServiceHelpers.getAuthenticated;
import static at.bumzack.foryouandyourfakewebshop.searcharticle.ServiceHelpers.getCustomerPrices;
import static at.bumzack.foryouandyourfakewebshop.searcharticle.ServiceHelpers.getMoviesPrice;
import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;


@Controller("SearchIndexArticleController")
@CrossOrigin
public class SearchIndexArticleController {

    private static final Logger LOG = Loggers.getLogger(SearchIndexArticleController.class);

    @NonNull
    public Mono<ServerResponse> searchArticles(final ServerRequest request) throws WebClientResponseException {
        request.headers().asHttpHeaders().forEach((k,v)-> {
            LOG.info("{} -> {}", k,v);
        });
        final ApiClient apiClient = new ApiClient();
        final DefaultApi api = new DefaultApi(apiClient);
        return request.bodyToMono(SearchArticleRequest.class)
                .flatMap(req -> searchMovies(api, req));
    }

    private Mono<ServerResponse> searchMovies(final DefaultApi api, final SearchArticleRequest req) {
        final var searchMovieIndexRequest = new SearchMovieIndexRequest();
        searchMovieIndexRequest.q(req.q());
        searchMovieIndexRequest.limit(req.limit());
        searchMovieIndexRequest.offset(req.offset());

        LOG.info("sending request {}", searchMovieIndexRequest);

        final Mono<AuthenticationEntry> authenticated = getAuthenticated(req.customer());
        final Mono<MovieSearchResult> movieSearchResultMono = api.searchArticle(searchMovieIndexRequest);

        final Mono<List<ArticleSearchResult>> articleSearchResult = Mono.zip(authenticated, movieSearchResultMono)
                .publishOn(Schedulers.boundedElastic())
                .flatMap(t -> {
                    final AuthenticationEntry auth = t.getT1();
                    final MovieSearchResult movieSearchResult = t.getT2();
                    LOG.info("found movies {}", movieSearchResult.getMovies().size());
                    LOG.info("found authentication entry  {}", auth);
                    if (CollectionUtils.isEmpty(movieSearchResult.getMovies())) {
                        return Mono.empty();
                    }
                    final List<SearchDoc> movies = movieSearchResult.getMovies();

                    final Set<String> tconsts = movies.stream().map(SearchDoc::getTconst).collect(Collectors.toSet());

                    // oh boy
                    final Flux<PriceEntry> moviesPrice = getMoviesPrice(tconsts.stream().toList());
                    final List<PriceEntry> priceEntries = moviesPrice.collectList().block();

                    if (CollectionUtils.isEmpty(priceEntries)) {
                        LOG.info("no prices found  for movie tconsts  {}", tconsts);
                        return Mono.empty();
                    }

                    final Map<String, PriceEntry> moviePrices = priceEntries
                            .stream()
                            .map(priceEntry -> Map.entry(priceEntry.getMovieTconst(), priceEntry))
                            .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue, (m1, m2) -> m1));
                    final List<CustomerPriceEntry> customerprices = getCustPrices(auth).collectList().block();

                    final List<ArticleSearchResult> articles = movies.stream().map(m -> {
                        final var price = moviePrices.get(m.getTconst());
                        final CustomerPriceEntry customerPrice = getCustomerPriceForMovie(customerprices, m);
                        final var cp = calcCustomerPrice(price, customerPrice);
                        return new ArticleSearchResult(m, price.getAmount(), cp);
                    }).toList();

                    // everything in here is illegal and BS
                    return Mono.just(articles);
                });

        return articleSearchResult
                .map(SearchArticleResponse::new)
                .flatMap(re -> ServerResponse.ok().body(BodyInserters.fromValue(re)));
    }

    private CustomerPriceEntry getCustomerPriceForMovie(final List<CustomerPriceEntry> customerprices, final SearchDoc m) {
        if (nonNull(customerprices)) {
            return customerprices.stream()
                    .filter(cp -> yearMatching(m, cp))
                    .findFirst()
                    .orElse(null);
        }
        return null;
    }

    private boolean yearMatching(final SearchDoc m, final CustomerPriceEntry cp) {
        if (nonNull(cp.getStartYear()) && nonNull(cp.getEndYear()) && nonNull(m.getYear())) {
            return cp.getStartYear() <= m.getYear() && m.getYear() <= cp.getEndYear();
        }
        return false;
    }

    private Double calcCustomerPrice(final PriceEntry price, final CustomerPriceEntry customerPrice) {
        if (nonNull(customerPrice) && nonNull(customerPrice.getDiscount()) && nonNull(price.getAmount())) {
            return (100.0 - customerPrice.getDiscount()) * price.getAmount() / 100.0;
        }
        return null;
    }

    private Flux<CustomerPriceEntry> getCustPrices(final AuthenticationEntry auth) {
        if (nonNull(auth)) {
            return getCustomerPrices(auth.getCustomerId());
        }
        return Flux.empty();
    }

    @RouterOperations({
            @RouterOperation(path = "/api/v1/solr/article",
                    method = POST,
                    operation = @Operation(operationId = "searchArticle",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = SearchArticleRequest.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = SearchArticleResponse.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> articleSearch() {
        return route()
                .nest(RequestPredicates.path("/api/v1/solr/"),
                        builder -> builder
                                .POST("article", this::searchArticles)
                                .build())
                .build();
    }
}
