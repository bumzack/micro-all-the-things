package at.bumzack.foryouandyourfakewebshop.searcharticle;

import at.bumzack.common.microthingisregistry.RegisterMicroService;
import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@OpenAPIDefinition(
        info = @Info(title = "Solr Search Article Service", version = "1.0", description = "Solr Search Article Service"),
        servers = {
                @Server(
                        description = "Solr Search Article Service",
                        url = "http://localhost:8600")
        })
@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common", "at.bumzack.foryouandyourfakewebshop"})
public class SearchIndexArticleApplication {
    private final RegisterMicroService microService;

    public SearchIndexArticleApplication(final RegisterMicroService microService) {
        this.microService = microService;
    }

    public static void main(String[] args) {
        SpringApplication.run(SearchIndexArticleApplication.class, args);
    }
}



