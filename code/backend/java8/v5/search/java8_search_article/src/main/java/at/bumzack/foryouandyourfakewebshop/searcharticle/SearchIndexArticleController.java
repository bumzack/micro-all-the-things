package at.bumzack.foryouandyourfakewebshop.searcharticle;

import at.bumzack.authenticationservice.ApiException;
import at.bumzack.authenticationservice.model.AuthenticationEntry;
import at.bumzack.customerpriceservice.model.CustomerPriceEntry;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.ArticleSearchResult;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.SearchArticleRequest;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.SearchArticleResponse;
import at.bumzack.priceservice.model.PriceEntry;
import at.bumzack.searchindexservice.model.MovieSearchResult;
import at.bumzack.searchindexservice.model.SearchDoc;
import at.bumzack.searchindexservice.model.SearchMovieIndexRequest;
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
import reactor.util.Logger;
import reactor.util.Loggers;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import static at.bumzack.foryouandyourfakewebshop.searcharticle.ServiceHelpers.getAuthenticated;
import static at.bumzack.foryouandyourfakewebshop.searcharticle.ServiceHelpers.getCustomerPrices;
import static at.bumzack.foryouandyourfakewebshop.searcharticle.ServiceHelpers.getMovies;
import static at.bumzack.foryouandyourfakewebshop.searcharticle.ServiceHelpers.getMoviesPrice;
import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.POST;


@Controller("SearchIndexArticleController")
@CrossOrigin
public class SearchIndexArticleController {

    private static final Logger LOG = Loggers.getLogger(SearchIndexArticleController.class);

    @NonNull
    @RouterOperation(path = "/api/v1/solr/article",
            method = POST,
            operation = @Operation(operationId = "searchArticle",
                    requestBody = @io.swagger.v3.oas.annotations.parameters.RequestBody(content = @Content(schema = @Schema(implementation = SearchArticleRequest.class))),
                    responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = SearchArticleResponse.class))),
                    })
    )
    @ResponseBody
    @PostMapping(value = "/api/v1/solr/article", produces = MediaType.APPLICATION_JSON_VALUE)
    public ResponseEntity<SearchArticleResponse> searchArticles(@RequestBody final SearchArticleRequest req) throws ApiException, at.bumzack.searchindexservice.ApiException, at.bumzack.customerpriceservice.ApiException, at.bumzack.priceservice.ApiException {
        LOG.info("searchArticles  request  {}", req);
        final var searchMovieIndexRequest = new SearchMovieIndexRequest();
        searchMovieIndexRequest.q(req.getQ());
        searchMovieIndexRequest.limit(req.getLimit());
        searchMovieIndexRequest.offset(req.getOffset());

        final MovieSearchResult movieSearchResult = getMovies(searchMovieIndexRequest);

        final var tconsts = movieSearchResult.getMovies().stream()
                .map(SearchDoc::getTconst)
                .collect(Collectors.toList());

        final AuthenticationEntry authenticated = getAuthenticated(req.getCustomer());
        final List<CustomerPriceEntry> customerPriceEntries = getCustomerPrices(req.getCustomer().getCustomerId());
        final Map<String, PriceEntry> prices = getMoviesPrice(tconsts).stream()
                .map(pe -> Map.entry(pe.getMovieTconst(), pe))
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue, (a, b) -> a));

        final var res = new ArrayList<ArticleSearchResult>();

        movieSearchResult.getMovies().forEach(m -> {
            final var result = new ArticleSearchResult();
            final var price = prices.get(m.getTconst());
            result.setArticle(m);
            result.setPrice(price.getAmount());
            if (nonNull(authenticated)) {
                final var customerPrice = customerPriceEntries.stream().filter(pe -> pe.getStartYear() <= m.getYear() && m.getYear() <= pe.getEndYear())
                        .findFirst()
                        .orElse(null);
                if (nonNull(customerPrice)) {
                    result.setCustomerPrice((100.0 - customerPrice.getDiscount()) * price.getAmount() / 100.0);
                }
            }
            res.add(result);
        });

        final var result = new SearchArticleResponse();
        result.setArticles(res);

        return ResponseEntity.ok(result);
    }
}
