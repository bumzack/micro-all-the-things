package at.bumzack.common.webflux;

import org.springframework.web.reactive.function.client.ExchangeStrategies;
import org.springframework.web.reactive.function.client.WebClient;

import static java.util.Objects.isNull;

public class WebClientFactory {
    private static WebClient client = null;

    @Deprecated(since = "use singleton stuff")
    public static WebClient getWebClient(final String url) {
        return WebClient.builder()
                .exchangeStrategies(ExchangeStrategies.builder()
                        .codecs(codecs -> codecs
                                .defaultCodecs()
                                .maxInMemorySize(1000 * 1024 * 1024))
                        .build())
                .baseUrl(url)
                .build();
    }

    private WebClientFactory() {
        // empty on purpose
    }

    public static final WebClient getClient() {
        if (isNull(client)) {
            client = WebClient.builder()
                    .exchangeStrategies(ExchangeStrategies.builder()
                            .codecs(codecs -> codecs
                                    .defaultCodecs()
                                    .maxInMemorySize(1000 * 1024 * 1024))
                            .build())
                    .build();
        }
        return client;
    }
}
