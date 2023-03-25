package at.solr.moviewriter.config;

import org.springframework.context.annotation.Configuration;
import org.springframework.http.codec.ServerCodecConfigurer;
import org.springframework.web.reactive.config.WebFluxConfigurer;
import reactor.util.Logger;

@Configuration
class ControllerConfig implements WebFluxConfigurer {
    private static final Logger LOG = reactor.util.Loggers.getLogger(ControllerConfig.class);

    @Override
    public void configureHttpMessageCodecs(final ServerCodecConfigurer configurer) {
        LOG.info("ControllerConfig");
        configurer.defaultCodecs().enableLoggingRequestDetails(true);
    }
}