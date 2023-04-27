package at.bumzack.common.solr;

import at.bumzack.common.webflux.WebClientFactory;
import org.springframework.core.ParameterizedTypeReference;
import reactor.core.publisher.Mono;
import reactor.util.Logger;


public class SolrExecutor {
    private static final Logger LOG = reactor.util.Loggers.getLogger(SolrExecutor.class);

    public static <T> Mono<SolrResponse<T>> executeSolrGet(final ParameterizedTypeReference<SolrResponse<T>> typeRef, final String url) {
        LOG.info("solr GET to URL:   \n{}\n", url);

//        getWebClient(url)
//                .get()
//                .retrieve()
//                .bodyToMono(String.class)
//                .subscribe(res -> {
//                    LOG.info("raw string response {}", res);
//                });

        final var client = WebClientFactory.getClient(url);
        return client
                .get()
                .retrieve()
                .bodyToMono(typeRef);
    }
}
