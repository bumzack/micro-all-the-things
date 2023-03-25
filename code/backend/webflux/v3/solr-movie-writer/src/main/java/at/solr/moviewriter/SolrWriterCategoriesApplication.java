package at.solr.moviewriter;

import at.bumzack.common.microthingisregistry.RegisterMicroService;
import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@ComponentScan(basePackages = "at.bumzack.common")
@SpringBootApplication
@OpenAPIDefinition(
        info = @Info(title = "Solr Movie Writer Service", version = "1.0", description = "Solr Writer"),
        servers = {
                @Server(
                        description = "ATS Client Backend",
                        url = "http://localhost:8080")
        })
public class SolrWriterCategoriesApplication {

    private final RegisterMicroService registerMicroService;

    public SolrWriterCategoriesApplication(RegisterMicroService registerMicroService) {
        this.registerMicroService = registerMicroService;
    }

    public static void main(String[] args) {
        SpringApplication.run(SolrWriterCategoriesApplication.class, args);
    }
}
