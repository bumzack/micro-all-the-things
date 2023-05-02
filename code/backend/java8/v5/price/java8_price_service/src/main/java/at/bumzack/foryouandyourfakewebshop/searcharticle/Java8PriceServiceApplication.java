package at.bumzack.foryouandyourfakewebshop.searcharticle;


import at.bumzack.foryouandyourfakewebshop.searcharticle.model.PriceEntry;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.PriceEntryRepository;
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

import static java.util.Objects.nonNull;

@OpenAPIDefinition(
        info = @Info(title = "Java8 Price Service", version = "1.0", description = "Java8 Price Service"),
        servers = {
                @Server(
                        description = "Java 8 Price Service",
                        url = "http://localhost:28800")
        })
@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common", "at.bumzack.foryouandyourfakewebshop"})
public class Java8PriceServiceApplication {

    private static final Logger LOG = LogManager.getLogger(Java8PriceServiceApplication.class);
    private final PriceEntryRepository repository;

    public Java8PriceServiceApplication(final PriceEntryRepository repository) {
        this.repository = repository;
    }

    public static void main(String[] args) {
        SpringApplication.run(Java8PriceServiceApplication.class, args);
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
            final var tconst = "tt0172645";
            final PriceEntry byMovieTconst = repository.findByMovieTconst(tconst);
            if (nonNull(byMovieTconst)) {
                LOG.info("found movie {}", byMovieTconst);
            } else {
                LOG.info("no price found for tconst {}", tconst);
            }

            final var tconsts = List.of("tt0172645", "tt10170890");
            repository.findPrices(tconsts).forEach(e -> LOG.info("findPrices  results   entry {}", e));
        };
    }
}



