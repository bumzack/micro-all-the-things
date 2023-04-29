package at.bumzack.foryouandyourfakewebshop.authenticationservice;

import at.bumzack.foryouandyourfakewebshop.authenticationservice.model.AuthenticationRepository;
import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.boot.CommandLineRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.ComponentScan;

import static java.util.Objects.nonNull;

@OpenAPIDefinition(
        info = @Info(title = "Java8 Authentication Service", version = "1.0", description = "Java8 Authentication Service"),
        servers = {
                @Server(
                        description = "Java8 Authentication Service",
                        url = "http://localhost:28982")
        })
@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common", "at.bumzack.foryouandyourfakewebshop"})
public class Java8AuthenticationServiceApplication {

    private static final Logger LOG = LogManager.getLogger(Java8AuthenticationServiceApplication.class);


    private final AuthenticationRepository repository;

    public Java8AuthenticationServiceApplication(final AuthenticationRepository repository) {
        this.repository = repository;
    }

    public static void main(String[] args) {
        SpringApplication.run(Java8AuthenticationServiceApplication.class, args);
    }


    @Bean
    public CommandLineRunner readDbData() {

        return args -> {
            final var auth = repository.findByCustomerId(1L);
            if (nonNull(auth)) {
                LOG.info("found auth for customer id 1:   auth {}", auth);
            } else {
                LOG.info("no auth found for customerId 1");
            }
        };
    }
}



