package at.bumzack.webflux;

import org.springframework.web.reactive.function.client.ExchangeStrategies;
import org.springframework.web.reactive.function.client.WebClient;

public class WebClientFactory {
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
}
