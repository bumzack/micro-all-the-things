package at.solr.moviewriter;


import at.bumzack.common.dto.Category;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.http.MediaType;
import org.springframework.stereotype.Controller;
import org.springframework.web.reactive.function.BodyExtractors;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClient;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.RequestPredicates;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.Loggers;
import reactor.util.annotation.NonNull;

import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrMovieWriterController")
public class SolrMovieWriterController {

    private static final Logger LOG = Loggers.getLogger(SolrMovieWriterController.class);

    @NonNull
    public Mono<ServerResponse> addMovie(final ServerRequest request) throws WebClientResponseException {
        LOG.info("addMovie");
        final var category = request.body(BodyExtractors.toMono(String.class));
        final var url = "http://localhost:8984/solr/movies/update?commitWithin=100&overwrite=true&wt=json";
        final var webClient = WebClient.create();

        return category
                .map(a -> {
                    // oh boy thats ugly ...
                    return "[" + a + "]";
                })
                .flatMap(a -> webClient.post()
                        .uri(url)
                        .contentType(MediaType.APPLICATION_JSON)
                        .accept()
                        .body(BodyInserters.fromValue(a))
                        .retrieve()
                        .bodyToMono(String.class)
                        .doOnNext(e -> LOG.info("movie solr response {}", e))
                        .doOnError(e -> LOG.error("movie error from Solr '{}'", e.getMessage()))
                        .flatMap(e -> ServerResponse.ok().body(BodyInserters.fromValue("category SolrWriter says: all good"))));
    }

    @RouterOperations({
            @RouterOperation(path = "/solr/movie",
                    method = POST,
                    operation = @Operation(operationId = "movie",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = Category.class))),
                            responses = {@ApiResponse(responseCode = "200"),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrMovieWriterRoutes() {
        return route()
                .nest(RequestPredicates.path("/solr"),
                        builder -> builder
                                .POST("movie", this::addMovie)
                                .build())
                .build();
    }
}


