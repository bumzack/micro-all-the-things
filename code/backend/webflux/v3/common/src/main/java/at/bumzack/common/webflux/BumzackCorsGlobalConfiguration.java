package at.bumzack.common.webflux;

import org.springframework.http.codec.ServerCodecConfigurer;
import org.springframework.stereotype.Component;
import org.springframework.web.reactive.config.CorsRegistry;
import org.springframework.web.reactive.config.WebFluxConfigurer;
import reactor.util.Logger;

@Component
public class BumzackCorsGlobalConfiguration implements WebFluxConfigurer {

    private static final Logger LOG = reactor.util.Loggers.getLogger(BumzackCorsGlobalConfiguration.class);

    @Override
    public void addCorsMappings(final CorsRegistry corsRegistry) {
        LOG.info("cors !!!");
        corsRegistry.addMapping("/**")
                .allowedOrigins("http://www.bumzack.at", "http://www.bumzack.at:3000", "http://localhost:3000", "http://locahost:3001", "http://localhost:3002")
                .allowedMethods("POST", "GET", "PUT")
                .maxAge(3600);
    }

    @Override
    public void configureHttpMessageCodecs(ServerCodecConfigurer configurer) {
        LOG.info("setting limit");
        configurer.defaultCodecs().maxInMemorySize(1000 * 1024 * 1024);
    }
}