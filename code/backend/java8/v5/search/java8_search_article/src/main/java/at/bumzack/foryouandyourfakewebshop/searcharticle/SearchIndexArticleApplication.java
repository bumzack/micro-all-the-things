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
            public void addCorsMappings(CorsRegistry registry) {
                registry.addMapping("*").allowedOrigins("*").allowedMethods("*");
            }
        };
    }


}



