package at.bumzack.solrsearch.solr;

import org.springframework.stereotype.Component;
import org.springframework.web.reactive.function.client.ExchangeStrategies;
import org.springframework.web.reactive.function.client.WebClient;
import reactor.core.publisher.Mono;
import reactor.util.Logger;


@Component
public class SolrService {
    private static final Logger LOG = reactor.util.Loggers.getLogger(SolrService.class);

    public Mono<SolrResponse> searchByCode(final String searchText, final int pageSize) {
        final var url = new SolrRequestBuilder()
                .setCoreName("categories")
                .setQuery("code:" + searchText)
                .setRowCount(pageSize)
                .addResponseField("id")
                .addResponseField("code")
                .addResponseField("name")
                .addResponseField("superCategories")
                .addResponseField("allSuperCategories")
                .build();

        final var webClient = WebClient.builder()
                .baseUrl(url)
                .exchangeStrategies(ExchangeStrategies.builder()
                        .codecs(codecs -> codecs
                                .defaultCodecs()
                                .maxInMemorySize(1000*1024 * 1024))
                        .build())
                        .build();

        final var responseSpec = webClient
                .get()
                .retrieve();
        LOG.info("searchByCode solr request URL   \n{}\n", url);

        return responseSpec.bodyToMono(SolrResponse.class);
    }

    public Mono<SolrResponse> allCategories() {
        final var url = new SolrRequestBuilder()
                .setCoreName("categories")
                .setQuery("*:*")
                .setRowCount(1000000)
                .addResponseField("id")
                .addResponseField("code")
                .addResponseField("name")
                .addResponseField("superCategories")
                .addResponseField("allSuperCategories")
                .build();
        final var webClient = WebClient.builder()
                .baseUrl(url)
                .exchangeStrategies(ExchangeStrategies.builder()
                        .codecs(codecs -> codecs
                                .defaultCodecs()
                                .maxInMemorySize(1000*1024 * 1024))
                        .build())
                .build();

        final var responseSpec = webClient
                .get()
                .retrieve();

        LOG.info("search all solr request URL   \n{}\n", url);

        return responseSpec.bodyToMono(SolrResponse.class);
    }
}
