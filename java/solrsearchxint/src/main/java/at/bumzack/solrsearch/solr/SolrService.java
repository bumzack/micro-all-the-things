package at.bumzack.solrsearch.solr;

import org.springframework.stereotype.Component;
import org.springframework.web.reactive.function.client.ExchangeStrategies;
import org.springframework.web.reactive.function.client.WebClient;
import reactor.core.publisher.Mono;
import reactor.util.Logger;


@Component
public class SolrService {
    private static final Logger LOG = reactor.util.Loggers.getLogger(SolrService.class);

    public Mono<SolrResponse> searchByMediaContainerQualifier(final String mediaContainerQualifier) {
        final var url = new SolrRequestBuilder()
                .setCoreName("xinet")
                .setQuery("mediaContainerQualifier:" + mediaContainerQualifier)
                .addResponseField("id")
                .addResponseField("code")
                .addResponseField("mediaContainerQualifier")
                .addResponseField("url")
                .addResponseField("width")
                .addResponseField("height")
                .addResponseField("channel")
                .addResponseField("valid")
                .addResponseField("mediaFormat")
                .addResponseField("mime")
                .build();

        final var webClient = WebClient.builder()
                .baseUrl(url)
                .exchangeStrategies(ExchangeStrategies.builder()
                        .codecs(codecs -> codecs
                                .defaultCodecs()
                                .maxInMemorySize(1000 * 1024 * 1024))
                        .build())
                .build();

        final var responseSpec = webClient.get()
                .retrieve();
        LOG.info("searchByCode solr request URL   \n{}\n", url);

        return responseSpec.bodyToMono(SolrResponse.class);
    }
}
