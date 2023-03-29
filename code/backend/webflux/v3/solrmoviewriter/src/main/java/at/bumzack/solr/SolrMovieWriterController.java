package at.bumzack.solr;


import at.bumzack.common.dto.Movie;
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

@Controller("SolrMovieWriterController")
@CrossOrigin
public class SolrMovieWriterController {

    public static final String SCHEMA = "http";
    public static final String COMMAND = "/update?commitWithin=1000&overwrite=true&wt=json";
    private static final Logger LOG = Loggers.getLogger(SolrMovieWriterController.class);
    @Value("${solr.host}")
    private String solrHost;

    @Value("${solr.port}")
    private String solrPort;

    @Value("${solr.core}")
    private String solrCore;

    private String solrUrl = null;

    private String getSolrUrl() {
        if (isNull(solrUrl)) {
            solrUrl = SolrUtils.getSolrUrl(solrHost, solrPort, solrCore, COMMAND, SCHEMA);
        }
        return solrUrl;
    }

    @NonNull
    public Mono<ServerResponse> addMovie(final ServerRequest request) throws WebClientResponseException {
        final var tsvLine = request
                .bodyToMono(TsvLine.class)
//                .doOnSuccess(tsv -> {
//                    LOG.info("processing movie {}", tsv);
//                })
                .doOnError(e -> {
                    LOG.error("error    {}", e);
                });
        final var webClient = WebClientFactory.getClient();

        return tsvLine
                .map(this::mapToMovie)
                .flatMap(a -> execSolrPost(webClient, a));
    }

    private Movie mapToMovie(final TsvLine tsvLine) {
        final Movie movie = new Movie();
        final List<String> entries = tsvLine.getEntries();
        movie.setTconst(entries.get(0));
        movie.setTitleType(getNullableValue(entries.get(1)));
        movie.setPrimaryTitle(getNullableValue(entries.get(2)));
        movie.setOriginalTitle(getNullableValue(entries.get(3)));
        movie.setAdult(getBoolean(entries.get(4)));
        movie.setStartYear(getNullableValue(entries.get(5)));
        movie.setEndYear(getNullableValue(entries.get(6)));
        movie.setRuntimeMinutes(getNullableValue(entries.get(7)));
        movie.setGenres(getList(entries.get(8)));
        movie.setId(movie.getTconst());

        return movie;
    }

    private Mono<ServerResponse> execSolrPost(final WebClient webClient, final Movie movie) {
        final var url = getSolrUrl();

        return webClient.post()
                .uri(url)
                .contentType(MediaType.APPLICATION_JSON)
                .accept()
                .body(BodyInserters.fromValue(List.of(movie)))
                .retrieve()
                .bodyToMono(String.class)
                //  .doOnNext(e -> LOG.info("movie solr response {}", e))
                .doOnError(e -> LOG.error("movie  error from Solr '{}'", e.getMessage()))
//                .doOnSuccess(s -> {
//                    // LOG.info("movie solr success {}", s);
//                })
                .flatMap(e -> ServerResponse.ok().body(BodyInserters.fromValue("movie  SolrWriter says: all good")));
    }

    @RouterOperations({
            @RouterOperation(path = "/api/movie",
                    method = POST,
                    operation = @Operation(operationId = "addMovieToSolr",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = TsvLine.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = String.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrMovieRoutes() {
        return route()
                .nest(RequestPredicates.path("/api/"),
                        builder -> builder
                                .POST("movie", this::addMovie)
                                .build())
                .build();
    }
}
