package at.bumzack.solrsearch;

import at.bumzack.solrsearch.solr.SolrService;
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

import java.util.Collection;

import static org.springframework.web.bind.annotation.RequestMethod.GET;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrXinetController")
@CrossOrigin
public class SolrXinetController {

    private static final Logger LOG = reactor.util.Loggers.getLogger(SolrXinetController.class);

    private final SolrService solrService;

    public SolrXinetController(final SolrService solrService) {
        this.solrService = solrService;
    }

    @NonNull
    public Mono<ServerResponse> search(final ServerRequest request) throws WebClientResponseException {
        final var mediaContainerQualifier = request.pathVariable("mediaContainerQualifier");
        LOG.info("search for mediaContainerQualifier '{}'", mediaContainerQualifier);
        // hahahahaaa Null safety
        return solrService.searchByMediaContainerQualifier(mediaContainerQualifier)
                .map(res -> res.getResponse().getDocs())
                .flatMap(searchResult -> ServerResponse.ok().body(BodyInserters.fromValue(searchResult)));
    }

    @RouterOperations({
            @RouterOperation(path = "/solr/search/xinet/{mediaContainerQualifier}",
                    method = GET,
                    operation = @Operation(operationId = "searchByArticleCode",
                            responses = {@ApiResponse(responseCode = "200", content = @Content(array = @ArraySchema(schema = @Schema(implementation = Collection.class)))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("/solr/search"),
                        builder -> builder
                                .GET("/xinet/{mediaContainerQualifier}", this::search)
                                .build())
                .build();
    }
}
