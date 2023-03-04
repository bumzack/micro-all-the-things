package at.bumzack.pimdbreader;

import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@OpenAPIDefinition(
		info = @Info(title = "PIM Image dDB Reader", version = "1.0", description = "PIM Image dDB Reader"),
		servers = {
				@Server(
						description = "ATS Client Backend",
						url = "http://localhost:8092")
		})

public class PimiImageDbReaderApplication {
	public static void main(String[] args) {
		SpringApplication.run(PimiImageDbReaderApplication.class, args);
	}

}
