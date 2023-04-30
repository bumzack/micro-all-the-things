package at.bumzack.foryouandyourfakewebshop.customerservice;

import at.bumzack.foryouandyourfakewebshop.customerservice.model.AddCustomerRequest;
import at.bumzack.foryouandyourfakewebshop.customerservice.model.Customer;
import at.bumzack.foryouandyourfakewebshop.customerservice.model.CustomerRepository;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.enums.ParameterIn;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.http.HttpStatus;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.RequestPredicates;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.Loggers;

import static org.springframework.web.bind.annotation.RequestMethod.GET;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;


@Controller("CustomerServiceController")
@CrossOrigin
public class CustomerServiceController {

    private static final Logger LOG = Loggers.getLogger(CustomerServiceController.class);

    private final CustomerRepository customerRepository;

    public CustomerServiceController(final CustomerRepository customerRepository) {
        this.customerRepository = customerRepository;
    }

    @NonNull
    public Mono<ServerResponse> getCustomer(final ServerRequest request) throws WebClientResponseException {
        final String email = request.pathVariable("email");
        return customerRepository.findByEmail(email)
                .flatMap(c -> {
                    LOG.info("customer found with email {},    {}  ", email, c);
                    return ServerResponse.ok().body(BodyInserters.fromValue(c));
                })
                .switchIfEmpty(customerNotFound(email));
    }

    private Mono<ServerResponse> customerNotFound(final String email) {
        LOG.info("could not find customer with email {}  in DB", email);
        return ServerResponse.notFound().build();
    }

    public Mono<ServerResponse> addCustomer(final ServerRequest request) throws WebClientResponseException {
        return request.bodyToMono(AddCustomerRequest.class)
                .flatMap(r -> {
                    final var c = new Customer(null, r.firstName(), r.lastName(), r.email(), r.password(), null);
                    LOG.info("inserting new customer {}", c);
                    return customerRepository
                            .save(c)
                            .flatMap(e -> ServerResponse.ok().body(BodyInserters.fromValue(e)))
                            .switchIfEmpty(ServerResponse.status(HttpStatus.INTERNAL_SERVER_ERROR).body(BodyInserters.fromValue("error saving the new customer ")));

                });
    }


    @RouterOperations({
            @RouterOperation(path = "/api/v1/customer/{email}",
                    method = GET,
                    operation = @Operation(operationId = "getCustomer",
                            parameters = {@Parameter(in = ParameterIn.PATH, name = "email", description = "Email address of customer", schema = @Schema(implementation = String.class))
                            },
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = Customer.class))),
                            })
            ),
            @RouterOperation(path = "/api/v1/customer",
                    method = POST,
                    operation = @Operation(operationId = "addCustomer",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = AddCustomerRequest.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = Customer.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> customerRoutes() {
        return route()
                .nest(RequestPredicates.path("/api/v1/"),
                        builder -> builder
                                .GET("/customer/{email}", this::getCustomer)
                                .POST("/customer", this::addCustomer)
                                .build())
                .build();
    }


}
