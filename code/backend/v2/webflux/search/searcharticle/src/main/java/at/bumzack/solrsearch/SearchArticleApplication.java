package at.bumzack.solrsearch;

import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
@SpringBootApplication
@OpenAPIDefinition(
		info = @Info(title = "Article Search Application", version = "1.0", description = "Article Search Application"),
		servers = {
				@Server(
						description = "ATS Client Backend",
						url = "http://localhost:8300")
		})
public class SearchArticleApplication {
	public static void main(String[] args) {
		SpringApplication.run(SearchArticleApplication.class, args);
	}
}
