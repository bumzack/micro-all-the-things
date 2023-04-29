package at.bumzack.foryouandyourfakewebshop.searchindexindex;

import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.CustomerPriceEntry;
import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.CustomerPriceEntryRepository;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.enums.ParameterIn;
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.RequestPredicates;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.Loggers;

import static org.springframework.http.MediaType.APPLICATION_JSON;
import static org.springframework.web.bind.annotation.RequestMethod.GET;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;


@Controller("CustomerPriceServiceController")
@CrossOrigin
public class CustomerPriceServiceController {

    private static final Logger LOG = Loggers.getLogger(CustomerPriceServiceController.class);

    private final CustomerPriceEntryRepository customerPriceEntryRepository;

    public CustomerPriceServiceController(final CustomerPriceEntryRepository customerPriceEntryRepository) {
        this.customerPriceEntryRepository = customerPriceEntryRepository;
    }

    @NonNull
    public Mono<ServerResponse> customerPrices(final ServerRequest request) throws WebClientResponseException {
        LOG.info("yo");
        final var customerId = request.pathVariable("customerId");
        final Mono<ServerResponse> map = customerPriceEntryRepository.findByCustomerId(customerId)
                .collectList()
                .flatMap(entries -> ServerResponse.ok()
                        .contentType(APPLICATION_JSON)
                        .bodyValue(entries));
        return map;

    }

    @RouterOperations({
            @RouterOperation(path = "/api/v1/customerprices/{customerId}",
                    method = GET,
                    operation = @Operation(operationId = "customerPrices",
                            parameters = {@Parameter(in = ParameterIn.PATH, name = "customerId", description = "Customer ID", schema  = @Schema(implementation = Long.class))
                            },
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = CustomerPriceEntry.class)))),
                            })
            ),
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("/api/v1"),
                        builder -> builder
                                .GET("/customerprices/{customerId}", this::customerPrices)
                                .build())
                .build();
    }
}
