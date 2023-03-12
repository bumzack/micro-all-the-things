package at.bumzack.postgreswriterprices;

import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
@SpringBootApplication
@OpenAPIDefinition(
		info = @Info(title = "Postgres Writer Application", version = "1.0", description = "Postgres Writer Application"),
		servers = {
				@Server(
						description = "ATS Client Backend",
						url = "http://localhost:8500")
		})
public class PostgresWriterPricesApplication {
	public static void main(String[] args) {
		SpringApplication.run(PostgresWriterPricesApplication.class, args);
	}
}
