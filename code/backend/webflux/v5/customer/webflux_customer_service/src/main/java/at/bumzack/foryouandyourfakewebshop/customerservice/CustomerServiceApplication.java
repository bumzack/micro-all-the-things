package at.bumzack.foryouandyourfakewebshop.customerservice;

import at.bumzack.common.microthingisregistry.RegisterMicroService;
import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@OpenAPIDefinition(
        info = @Info(title = "Customer Service", version = "1.0", description = "Customer Service"),
        servers = {
                @Server(
                        description = "Customer Service",
                        url = "http://localhost:8980")
        })
@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common", "at.bumzack.foryouandyourfakewebshop"})
public class CustomerServiceApplication {
    private final RegisterMicroService microService;

    public CustomerServiceApplication(final RegisterMicroService microService) {
        this.microService = microService;
    }

    public static void main(String[] args) {
        SpringApplication.run(CustomerServiceApplication.class, args);
    }
}
