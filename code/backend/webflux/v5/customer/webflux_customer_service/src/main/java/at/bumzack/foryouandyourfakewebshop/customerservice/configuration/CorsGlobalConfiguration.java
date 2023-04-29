package at.bumzack.foryouandyourfakewebshop.customerservice.configuration;

import org.springframework.context.annotation.Configuration;
import org.springframework.web.reactive.config.CorsRegistry;
import org.springframework.web.reactive.config.EnableWebFlux;
import org.springframework.web.reactive.config.WebFluxConfigurer;
import reactor.util.Logger;
import reactor.util.Loggers;


@Configuration
@EnableWebFlux
public class CorsGlobalConfiguration implements WebFluxConfigurer {
    private static final Logger LOG = Loggers.getLogger(CorsGlobalConfiguration.class);

    public void addCorsMappings(final CorsRegistry corsRegistry) {
        LOG.info("cors !!!");
        corsRegistry.addMapping("/**")
                .allowedOrigins("http://www.bumzack.at", "http://www.bumzack.at:3000", "http://localhost:3000", "http://locahost:3001", "http://localhost:3002")
                .allowedMethods("POST", "GET", "PUT")
                .maxAge(3600);
    }
}
