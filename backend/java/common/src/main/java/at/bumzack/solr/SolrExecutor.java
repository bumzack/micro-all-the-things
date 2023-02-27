package at.bumzack.solr;

import org.springframework.core.ParameterizedTypeReference;
import reactor.core.publisher.Mono;
import reactor.util.Logger;

import static at.bumzack.webflux.WebClientFactory.getWebClient;

public class SolrExecutor {
    private static final Logger LOG = reactor.util.Loggers.getLogger(SolrExecutor.class);

    public static final <T> Mono<SolrResponse<T>> executeSolrGet(final ParameterizedTypeReference<SolrResponse<T>> typeRef, final String url) {
        LOG.info("solr GET to URL:   \n{}\n", url);

        return getWebClient(url)
                .get()
                .retrieve()
                .bodyToMono(typeRef);
    }
}
