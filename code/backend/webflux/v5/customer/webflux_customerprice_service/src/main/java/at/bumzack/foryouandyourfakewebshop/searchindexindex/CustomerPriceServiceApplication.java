package at.bumzack.foryouandyourfakewebshop.searchindexindex;

import at.bumzack.common.microthingisregistry.RegisterMicroService;
import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@OpenAPIDefinition(
        info = @Info(title = "CustomerPrice Service", version = "1.0", description = "CustomerPrice Service"),
        servers = {
                @Server(
                        description = "CustomerPrice Service",
                        url = "http://localhost:8981")
        })
@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common", "at.bumzack.foryouandyourfakewebshop"})
public class CustomerPriceServiceApplication {

    private final RegisterMicroService microService;

    public CustomerPriceServiceApplication(final RegisterMicroService microService) {
        this.microService = microService;
    }

    public static void main(String[] args) {
        SpringApplication.run(CustomerPriceServiceApplication.class, args);
    }
}
