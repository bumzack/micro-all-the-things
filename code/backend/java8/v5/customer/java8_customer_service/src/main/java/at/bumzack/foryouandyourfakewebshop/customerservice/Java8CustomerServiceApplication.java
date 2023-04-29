package at.bumzack.foryouandyourfakewebshop.customerservice;

import at.bumzack.foryouandyourfakewebshop.customerservice.model.Customer;
import at.bumzack.foryouandyourfakewebshop.customerservice.model.CustomerRepository;
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
import org.springframework.web.servlet.config.annotation.CorsRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

import static java.util.Objects.nonNull;

@OpenAPIDefinition(
        info = @Info(title = "Java8 Customer Service", version = "1.0", description = "Java8 Customer Service"),
        servers = {
                @Server(
                        description = "Java8 Customer Service",
                        url = "http://localhost:28980")
        })
@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common", "at.bumzack.foryouandyourfakewebshop"})
public class Java8CustomerServiceApplication {
    private static final Logger LOG = LogManager.getLogger(CustomerRepository.class);
    private final CustomerRepository repository;

    public Java8CustomerServiceApplication(final CustomerRepository repository) {
        this.repository = repository;
    }

    public static void main(String[] args) {
        SpringApplication.run(Java8CustomerServiceApplication.class, args);
    }

    @Bean
    public WebMvcConfigurer corsConfigurer() {
        return new WebMvcConfigurer() {
            @Override
            public void addCorsMappings(CorsRegistry registry) {
                registry.addMapping("*").allowedOrigins("*").allowedMethods("*");
            }
        };
    }

    @Bean
    public CommandLineRunner readDb() {
        return args -> {
            //  repository.findAll().forEach(e -> LOG.info("entry {}", e));
            final String email = "bumzack@bumzack.at";
            final Customer customer = repository.findByEmail(email);
            if (nonNull(customer)) {
                LOG.info("found customer {}", customer);
            } else {
                LOG.info("no customer found for email {}", email);
            }
        };
    }
}
