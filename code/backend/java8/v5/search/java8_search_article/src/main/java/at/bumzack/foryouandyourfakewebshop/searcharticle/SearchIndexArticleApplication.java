package at.bumzack.foryouandyourfakewebshop.searcharticle;

import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.ComponentScan;
import org.springframework.web.servlet.config.annotation.CorsRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

@OpenAPIDefinition(
        info = @Info(title = "Solr Search Article Service", version = "1.0", description = "Solr Search Article Service"),
        servers = {
                @Server(
                        description = "Solr Search Article Service",
                        url = "http://localhost:28600")
        })
@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common", "at.bumzack.foryouandyourfakewebshop"})
public class SearchIndexArticleApplication {
    public static void main(String[] args) {
        SpringApplication.run(SearchIndexArticleApplication.class, args);
    }

    @Bean
    public WebMvcConfigurer corsConfigurer() {
        return new WebMvcConfigurer() {
            @Override
            public void addCorsMappings(final CorsRegistry registry) {
                registry
                        .addMapping("/**")
                        .allowedOrigins("*")
                        .allowedMethods("*")
                        .allowedHeaders("User-Agent",
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
                                "x-processed-by");
            }
        };
    }
}



