package at.bumzack.foryouandyourfakewebshop.searcharticle.configuration;

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
                .allowedOrigins("*")
                .allowedMethods("*")
                .allowedHeaders( "User-Agent",
                        "Sec-Fetch-Mode",
                        "Referer",
                        "Origin",
                        "content-type",
                        "Access-Control-Request-Method",
                        "Access-Control-Request-Headers",
                        "Access-Control-Allow-Headers",
                        "Access-Control-Allow-Methods",
                        "Access-Control-Allow-Origin",
                        "Access-Control-Expose-Headers",
                        "Access-Control-Request-Headers",
                        "Access-Control-Request-Methods",
                        "Accept-Encoding",
                        "Accept-Language",
                        "Accept-Post",
                        "Access-Control-Allow-Credentials",
                        "keep-alive",
                        "x-duration",
                        "x-provided-by",
                        "x-initiated-by",
                        "x-processed-by")
                .maxAge(3600);
    }
}
