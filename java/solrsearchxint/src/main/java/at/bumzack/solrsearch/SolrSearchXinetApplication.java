package at.bumzack.solrsearch;

import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
@SpringBootApplication
@OpenAPIDefinition(
		info = @Info(title = "Solr Search Application", version = "1.0", description = "Solr Search"),
		servers = {
				@Server(
						description = "ATS Client Backend",
						url = "http://localhost:8082")
		})

public class SolrSearchXinetApplication {

	public static void main(String[] args) {
		SpringApplication.run(SolrSearchXinetApplication.class, args);
	}

}
