package at.bumzack.solrwriterimages;

import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@OpenAPIDefinition(
        info = @Info(title = "Solr Writer Application", version = "1.0", description = "Solr Writer"),
        servers = {
                @Server(
                        description = "ATS Client Backend",
                        url = "http://localhost:8080")
        })

public class SolrWriterImagesApplication {
    public static void main(String[] args) {
        SpringApplication.run(SolrWriterImagesApplication.class, args);
    }
}
