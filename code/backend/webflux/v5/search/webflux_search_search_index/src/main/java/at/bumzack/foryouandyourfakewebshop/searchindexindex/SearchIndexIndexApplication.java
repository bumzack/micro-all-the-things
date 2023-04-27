package at.bumzack.foryouandyourfakewebshop.searchindexindex;

import at.bumzack.common.microthingisregistry.RegisterMicroService;
import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@OpenAPIDefinition(
        info = @Info(title = "Search Index Index Service", version = "1.0", description = "Search Index Index Service"),
        servers = {
                @Server(
                        description = "Search Index Index Service",
                        url = "http://localhost:8320")
        })
@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common", "at.bumzack.foryouandyourfakewebshop"})
public class SearchIndexIndexApplication {

    private final RegisterMicroService microService;

    public SearchIndexIndexApplication(final RegisterMicroService microService) {
        this.microService = microService;
    }

    public static void main(String[] args) {
        SpringApplication.run(SearchIndexIndexApplication.class, args);
    }
}
