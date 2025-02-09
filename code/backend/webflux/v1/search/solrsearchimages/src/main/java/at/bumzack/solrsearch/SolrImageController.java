package at.bumzack.solrsearch;

import at.bumzack.dto.Image;
import at.bumzack.search.SearchResult;
import at.bumzack.solr.SolrResponse;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
import org.springframework.http.HttpStatus;
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
import reactor.util.annotation.NonNull;

import java.util.Collection;

import static at.bumzack.dto.Image.TYPE_REF_IMAGE;
import static java.util.Objects.nonNull;
import static org.springframework.web.bind.annotation.RequestMethod.GET;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrImageController")
@CrossOrigin
public class SolrImageController {

    private static final Logger LOG = Loggers.getLogger(SolrImageController.class);

    private final SolrService solrService;

    public SolrImageController(final SolrService solrService) {
        this.solrService = solrService;
    }

    @NonNull
    public Mono<ServerResponse> search(final ServerRequest request) throws WebClientResponseException {
        final var mediaContainerQualifier = request.pathVariable("mediaContainerQualifier");
        LOG.info("search for mediaContainerQualifier '{}'", mediaContainerQualifier);
        // hahahahaaa Null safety
        return solrService.searchByMediaContainerQualifier(mediaContainerQualifier, TYPE_REF_IMAGE)
                .doOnNext(i -> {
                    LOG.info("got an img {}", i);
                })
                .flatMap(this::extractImageList)
                .switchIfEmpty(ServerResponse.status(HttpStatus.NOT_FOUND).body(BodyInserters.fromValue("no images found")));
    }

    private Mono<ServerResponse> extractImageList(final SolrResponse<Image> res) {
        final var result = new SearchResult<Image>();
        LOG.info("solr response {}", res);
        if (nonNull(res.getResponse())) {
            result.setItems(res.getResponse().getDocs());
            return ServerResponse.ok().body(BodyInserters.fromValue(result));
        }
        return ServerResponse.notFound().build();
    }

    @RouterOperations({
            @RouterOperation(path = "/solr/search/image/{mediaContainerQualifier}",
                    method = GET,
                    operation = @Operation(operationId = "searchImageByMediaContainerQualifier",
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = Collection.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrRoutes() {
        return route()
                .nest(RequestPredicates.path("/solr/search"),
                        builder -> builder
                                .GET("/image/{mediaContainerQualifier}", this::search)
                                .build())
                .build();
    }
}
