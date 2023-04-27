package at.bumzack.common.webflux;

import org.springframework.web.reactive.function.client.WebClient;

import static java.util.Objects.isNull;

public class WebClientFactory {
    private static WebClient client = null;

    private WebClientFactory() {
        // empty on purpose
    }

    public static WebClient getClient(final String url ) {
        if (isNull(client)) {
            client = WebClient
                    .builder()
                    .baseUrl(url)
                    .build();
        }
        return client;
    }
}
