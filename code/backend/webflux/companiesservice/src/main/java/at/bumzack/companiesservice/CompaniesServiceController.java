package at.bumzack.companiesservice;


import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.ArraySchema;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
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
import reactor.util.annotation.NonNull;

import static org.springframework.web.bind.annotation.RequestMethod.GET;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("CompaniesServiceController")
@CrossOrigin
public class CompaniesServiceController {
    private static final Logger LOG = reactor.util.Loggers.getLogger(CompaniesServiceController.class);

    private final CompanyService companyService;

    public CompaniesServiceController(final CompanyService companyService) {
        this.companyService = companyService;
    }

    @NonNull
    public Mono<ServerResponse> all(final ServerRequest request) throws WebClientResponseException {
        return companyService.findAll()
                .collectList()
                .flatMap(companies -> ServerResponse.ok().body(BodyInserters.fromValue(companies)));
    }

    @RouterOperations({
            @RouterOperation(path = "/company",
                    method = GET,
                    operation = @Operation(operationId = "all",
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = Company.class)))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("/company/"),
                        builder -> builder
                                .GET("desktop", this::all)
                                .build())
                .build();
    }
}
