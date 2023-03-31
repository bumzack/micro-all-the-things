package at.bumzack.solr.config;

import org.springframework.context.annotation.Configuration;
import org.springframework.http.codec.ServerCodecConfigurer;
import org.springframework.web.reactive.config.EnableWebFlux;
import org.springframework.web.reactive.config.WebFluxConfigurer;

@Configuration
@EnableWebFlux
class ControllerConfig implements WebFluxConfigurer {

    @Override
    public void configureHttpMessageCodecs(final ServerCodecConfigurer configurer) {
        configurer.defaultCodecs().enableLoggingRequestDetails(true);
        configurer.defaultCodecs().maxInMemorySize(1000 * 1024 * 1024);
    }
}