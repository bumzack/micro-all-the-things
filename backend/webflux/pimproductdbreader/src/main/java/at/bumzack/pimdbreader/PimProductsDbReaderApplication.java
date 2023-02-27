package at.bumzack.pimdbreader;

import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@OpenAPIDefinition(
		info = @Info(title = "PIM Product DB  Reader", version = "1.0", description = "PIM Product DB  Reader"),
		servers = {
				@Server(
						description = "ATS Client Backend",
						url = "http://localhost:8090")
		})

public class PimProductsDbReaderApplication {
	public static void main(String[] args) {
		SpringApplication.run(PimProductsDbReaderApplication.class, args);
	}
}
