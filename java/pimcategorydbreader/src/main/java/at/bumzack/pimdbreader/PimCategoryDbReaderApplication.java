package at.bumzack.pimdbreader;

import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@OpenAPIDefinition(
		info = @Info(title = "Solr Reader Client API", version = "1.0", description = "Solr Reader"),
		servers = {
				@Server(
						description = "ATS Client Backend",
						url = "http://localhost:8080")
		})

public class PimCategoryDbReaderApplication {
	public static void main(String[] args) {
		SpringApplication.run(PimCategoryDbReaderApplication.class, args);
	}

}
