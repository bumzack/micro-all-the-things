package at.bumzack.foryouandyourfakewebshop.authenticationservice;

import at.bumzack.common.microthingisregistry.RegisterMicroService;
import at.bumzack.foryouandyourfakewebshop.authenticationservice.model.AuthenticationRepository;
import io.swagger.v3.oas.annotations.OpenAPIDefinition;
import io.swagger.v3.oas.annotations.info.Info;
import io.swagger.v3.oas.annotations.servers.Server;
import org.springframework.boot.CommandLineRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.ComponentScan;
import reactor.core.publisher.Mono;

@OpenAPIDefinition(
        info = @Info(title = "Authentication Service", version = "1.0", description = "Authentication Service"),
        servers = {
                @Server(
                        description = "Authentication Service",
                        url = "http://localhost:8982")
        })
@SpringBootApplication
@ComponentScan(basePackages = {"at.bumzack.common", "at.bumzack.foryouandyourfakewebshop"})
public class AuthenticationServiceApplication {

//    static {
//        BlockHound.builder()
//              //   .allowBlockingCallsInside("java.io.RandomAccessFile", "readBytes")
//                .install();
//    }

    private final RegisterMicroService microService;
    private final AuthenticationRepository repository;

    public AuthenticationServiceApplication(final RegisterMicroService microService, final AuthenticationRepository repository) {
        this.microService = microService;
        this.repository = repository;
    }

    public static void main(String[] args) {
        SpringApplication.run(AuthenticationServiceApplication.class, args);
    }


    @Bean
    public CommandLineRunner readDbData() {

        return args -> repository.findByCustomerId(1L)
                .flatMap(auth -> {
                    System.out.println("got an auth entry for customer id 1 " + auth);
                    return Mono.just("found it");
                })
                .switchIfEmpty(Mono.just("not found"))
                .subscribe(System.out::println);

    }
}



