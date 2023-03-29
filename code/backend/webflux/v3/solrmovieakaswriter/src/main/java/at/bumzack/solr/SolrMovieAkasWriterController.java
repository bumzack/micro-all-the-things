package at.bumzack.solr;


import at.bumzack.common.dto.MovieAkas;
import at.bumzack.common.dto.TsvLine;
import at.bumzack.common.solr.SolrUtils;
import at.bumzack.common.webflux.WebClientFactory;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.http.MediaType;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.CrossOrigin;
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

import java.util.List;

import static at.bumzack.common.tsv.TsvUtils.getBoolean;
import static at.bumzack.common.tsv.TsvUtils.getList;
import static at.bumzack.common.tsv.TsvUtils.getNullableValue;
import static java.util.Objects.isNull;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrMovieAkasWriterController")
@CrossOrigin
public class SolrMovieAkasWriterController {

    private static final Logger LOG = Loggers.getLogger(SolrMovieAkasWriterController.class);

    private static final String COMMAND = "/update?commitWithin=100&overwrite=true&wt=json";
    private static final String SCHEMA = "http";

    @Value("${solr.host}")
    private String solrHost;

    @Value("${solr.port}")
    private String solrPort;

    @Value("${solr.core}")
    private String solrCore;

    private String solrUrl = null;

    private String getSolrUrl(final String solrHost, final String solrPort, final String solrCore, final String command, final String schema) {
        if (isNull(solrUrl)) {
            solrUrl = SolrUtils.getSolrUrl(solrHost, solrPort, solrCore, command, schema);
        }
        return solrUrl;
    }

    @NonNull
    public Mono<ServerResponse> addMovieAka(final ServerRequest request) throws WebClientResponseException {
        final var tsvLine = request
                .bodyToMono(TsvLine.class)
//                .doOnSuccess(tsv -> {
//                    LOG.info("processing movieAka {}", tsv);
//                })
                .doOnError(e -> {
                    LOG.error("error movieAka   {}", e);
                });
        final var webClient = WebClientFactory.getClient();

        return tsvLine
                .map(this::mapToMovieAka)
                .flatMap(a -> execSolrPost(webClient, a));
    }

    private MovieAkas mapToMovieAka(final TsvLine t) {
        final MovieAkas movieAkas = new MovieAkas();
        final List<String> entries = t.getEntries();
        movieAkas.setTitleId(entries.get(0));
        movieAkas.setOrdering(getNullableValue(entries.get(1)));
        movieAkas.setTitle(getNullableValue(entries.get(2)));
        movieAkas.setRegion(getNullableValue(entries.get(3)));
        movieAkas.setLanguage(getNullableValue(entries.get(4)));
        movieAkas.setTypes(getList(entries.get(5)));
        movieAkas.setAttributes(getList(entries.get(6)));
        movieAkas.setOriginalTitle(getBoolean(entries.get(7)));
        movieAkas.setId(String.join("_", movieAkas.getTitleId(), movieAkas.getOrdering()));
        return movieAkas;
    }

    private Mono<ServerResponse> execSolrPost(final WebClient webClient, final MovieAkas movieAka) {
        final String url = getSolrUrl(solrHost, solrPort, solrCore, COMMAND, SCHEMA);

        return webClient.post()
                .uri(url)
                .contentType(MediaType.APPLICATION_JSON)
                .accept()
                .body(BodyInserters.fromValue(List.of(movieAka)))
                .retrieve()
                .bodyToMono(String.class)
                //       .doOnNext(e -> LOG.info("movieaka solr response {}", e))
                .doOnError(e -> LOG.error("movieaka  error from Solr '{}'", e.getMessage()))
                .flatMap(e -> ServerResponse.ok().body(BodyInserters.fromValue("movieaka  SolrWriter says: all good")));
    }

    @RouterOperations({
            @RouterOperation(path = "/solr/movieaka",
                    method = POST,
                    operation = @Operation(operationId = "addMovieAkaToSolr",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = TsvLine.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = String.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrMovieAkaRoutes() {
        return route()
                .nest(RequestPredicates.path("/api/"),
                        builder -> builder
                                .POST("movieaka", this::addMovieAka)
                                .build())
                .build();
    }
}
