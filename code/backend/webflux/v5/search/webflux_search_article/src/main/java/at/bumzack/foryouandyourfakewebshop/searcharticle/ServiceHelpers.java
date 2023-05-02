package at.bumzack.foryouandyourfakewebshop.searcharticle;

import at.bumzack.authenticationservice.model.AuthenticationEntry;
import at.bumzack.customerpriceservice.model.CustomerPriceEntry;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.SearchCustomer;
import at.bumzack.priceservice.model.PriceEntry;
import at.bumzack.priceservice.model.SearchPricesRequest;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.util.List;


public class ServiceHelpers {

    public static Mono<AuthenticationEntry> getAuthenticated(final SearchCustomer searchCustomer) {
        final at.bumzack.authenticationservice.ApiClient authApiClient = new at.bumzack.authenticationservice.ApiClient();
        final at.bumzack.authenticationservice.api.DefaultApi authDefaultApi = new at.bumzack.authenticationservice.api.DefaultApi(authApiClient);
        return authDefaultApi.loggedin(searchCustomer.customerId());
    }

    public static Flux<PriceEntry> getMoviesPrice(final List<String> movieTconsts) {
        final at.bumzack.priceservice.ApiClient priceApiClient = new at.bumzack.priceservice.ApiClient();
        final at.bumzack.priceservice.api.DefaultApi priceApi = new at.bumzack.priceservice.api.DefaultApi(priceApiClient);
        final var req = new SearchPricesRequest();
        req.setMovieTconst(movieTconsts);
        return priceApi.getPricesForMovies(req);
    }

    public static Flux<CustomerPriceEntry> getCustomerPrices(final Long customerId) {
        final at.bumzack.customerpriceservice.ApiClient customerPriceApiClient = new at.bumzack.customerpriceservice.ApiClient();
        final at.bumzack.customerpriceservice.api.DefaultApi customerPriceApi = new at.bumzack.customerpriceservice.api.DefaultApi(customerPriceApiClient);
        return customerPriceApi.customerPrices(customerId);
    }


}
