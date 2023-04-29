package at.bumzack.foryouandyourfakewebshop.searcharticle;

import at.bumzack.foryouandyourfakewebshop.searcharticle.model.PriceEntry;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.PriceEntryRepository;
import at.bumzack.foryouandyourfakewebshop.searcharticle.model.SearchPricesRequest;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.enums.ParameterIn;
import io.swagger.v3.oas.annotations.media.ArraySchema;
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


@Controller("PriceServiceController")
@CrossOrigin
public class PriceServiceController {

    private static final Logger LOG = Loggers.getLogger(PriceServiceController.class);
    private final PriceEntryRepository priceEntryRepository;

    public PriceServiceController(final PriceEntryRepository priceEntryRepository) {
        this.priceEntryRepository = priceEntryRepository;
    }

    @NonNull
    public Mono<ServerResponse> getPriceForMovie(final ServerRequest request) throws WebClientResponseException {
        LOG.info("yo   getPriceForMovie");
        final var tconst = request.pathVariable("tconst");
        return priceEntryRepository.findByMovieTconst(tconst)
                .flatMap(entry -> ServerResponse.ok()
                        .contentType(APPLICATION_JSON)
                        .bodyValue(entry));
    }

    @NonNull
    public Mono<ServerResponse> getPricesForMovies(final ServerRequest request) throws WebClientResponseException {
        LOG.info("yoyo getPricseForMovies");
        return request.bodyToMono(SearchPricesRequest.class)
                .flatMap(req -> priceEntryRepository.findMovies(req.movieTconst())
                        .collectList()
                        .flatMap(entry -> ServerResponse.ok()
                                .contentType(APPLICATION_JSON)
                                .bodyValue(entry)))
                .switchIfEmpty(ServerResponse.status(HttpStatus.NOT_FOUND).bodyValue("no prices found"));
    }

    @RouterOperations({
            @RouterOperation(path = "/api/v1/price/{tconst}",
                    method = GET,
                    operation = @Operation(operationId = "getPriceForMovie",
                            parameters = {@Parameter(in = ParameterIn.PATH, name = "tconst", description = "Movie tconst ID", schema  = @Schema(implementation = String.class))
                            }, responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = PriceEntry.class))),
                    })
            ),
            @RouterOperation(path = "/api/v2/prices",
                    method = POST,
                    operation = @Operation(operationId = "getPricesForMovies",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = SearchPricesRequest.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = PriceEntry.class)))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> priceServiceRoutes() {
        return route()
                .nest(RequestPredicates.path("/api/"),
                        builder -> builder
                                .GET("/v1/price/{tconst}", this::getPriceForMovie)
                                .POST("/v2/prices", this::getPricesForMovies).build())
                .build();
    }
}
