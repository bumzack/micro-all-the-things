package at.bumzack.foryouandyourfakewebshop.searchindexindex;

import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.CustomerPriceEntry;
import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.CustomerPriceEntryRepository;
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

import java.util.List;
import java.util.stream.Collectors;

import static java.util.Objects.nonNull;

@OpenAPIDefinition(
        info = @Info(title = "Java8 CustomerPrice Service", version = "1.0", description = "Java8 CustomerPrice Service"),
        servers = {
                @Server(
                        description = "Java8 CustomerPrice Service",
                        url = "http://localhost:28981")
        })
@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common", "at.bumzack.foryouandyourfakewebshop"})
public class Java8CustomerPriceServiceApplication {
    private static final Logger LOG = LogManager.getLogger(Java8CustomerPriceServiceController.class);

    private final CustomerPriceEntryRepository repository;

    public Java8CustomerPriceServiceApplication(final CustomerPriceEntryRepository repository) {
        this.repository = repository;
    }

    public static void main(String[] args) {
        SpringApplication.run(Java8CustomerPriceServiceApplication.class, args);
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
            final List<CustomerPriceEntry> priceEntries = repository.findByCustomerId(1L);
            if (nonNull(priceEntries)) {
                LOG.info("found prices for customer  {}", priceEntries.stream().map(CustomerPriceEntry::toString).collect(Collectors.joining(" / ")));
            } else {
                LOG.info("no customer found for email {}", email);
            }
        };
    }
}
