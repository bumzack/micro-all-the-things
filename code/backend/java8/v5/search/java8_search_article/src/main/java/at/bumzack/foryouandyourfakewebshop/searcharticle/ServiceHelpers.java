package at.bumzack.foryouandyourfakewebshop.searcharticle;

import at.bumzack.authenticationservice.model.AuthenticationEntry;
import at.bumzack.customerpriceservice.ApiException;
import at.bumzack.customerpriceservice.model.CustomerPriceEntry;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.SearchCustomer;
import at.bumzack.priceservice.model.PriceEntry;
import at.bumzack.priceservice.model.SearchPricesRequest;
import at.bumzack.searchindexservice.model.MovieSearchResult;
import at.bumzack.searchindexservice.model.SearchMovieIndexRequest;

import java.util.List;


public class ServiceHelpers {

    public static AuthenticationEntry getAuthenticated(final SearchCustomer searchCustomer) throws at.bumzack.authenticationservice.ApiException {
        final at.bumzack.authenticationservice.ApiClient authApiClient = new at.bumzack.authenticationservice.ApiClient();
        authApiClient.setPort(28982);
        final at.bumzack.authenticationservice.api.DefaultApi authDefaultApi = new at.bumzack.authenticationservice.api.DefaultApi(authApiClient);
        return authDefaultApi.loggedin(searchCustomer.getCustomerId());
    }

    public static List<PriceEntry> getMoviesPrice(final List<String> movieTconsts) throws at.bumzack.priceservice.ApiException {
        final at.bumzack.priceservice.ApiClient priceApiClient = new at.bumzack.priceservice.ApiClient();
        priceApiClient.setPort(28800);
        final at.bumzack.priceservice.api.DefaultApi priceApi = new at.bumzack.priceservice.api.DefaultApi(priceApiClient);
        final var req = new SearchPricesRequest();
        req.setMovieTconst(movieTconsts);
        return priceApi.getPricesForMovies(req);
    }

    public static List<CustomerPriceEntry> getCustomerPrices(final Long customerId) throws ApiException {
        final at.bumzack.customerpriceservice.ApiClient customerPriceApiClient = new at.bumzack.customerpriceservice.ApiClient();
        customerPriceApiClient.setPort(28981);
        final at.bumzack.customerpriceservice.api.DefaultApi customerPriceApi = new at.bumzack.customerpriceservice.api.DefaultApi(customerPriceApiClient);
        return customerPriceApi.customerPrices(customerId);
    }

    public static MovieSearchResult getMovies(final SearchMovieIndexRequest req) throws at.bumzack.searchindexservice.ApiException {
        final at.bumzack.searchindexservice.ApiClient searchApiClient = new at.bumzack.searchindexservice.ApiClient();
        searchApiClient.setPort(28320);
        final at.bumzack.searchindexservice.api.DefaultApi movieApi = new at.bumzack.searchindexservice.api.DefaultApi(searchApiClient);
        return movieApi.searchDocs(req);
    }


}
