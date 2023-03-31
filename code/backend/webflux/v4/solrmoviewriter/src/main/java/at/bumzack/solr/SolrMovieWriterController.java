package at.bumzack.solr;


import at.bumzack.common.dto.Movie;
import at.bumzack.common.dto.TsvLine;
import at.bumzack.common.dto.TsvLines;
import at.bumzack.common.entitywriter.SolrEntityWriter;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.media.Content;
import io.swagger.v3.oas.annotations.media.Schema;
import io.swagger.v3.oas.annotations.parameters.RequestBody;
import io.swagger.v3.oas.annotations.responses.ApiResponse;
import org.springdoc.core.annotations.RouterOperation;
import org.springdoc.core.annotations.RouterOperations;
import org.springframework.context.annotation.Bean;
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
import reactor.util.annotation.NonNull;

import java.util.List;

import static at.bumzack.common.tsv.TsvUtils.*;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrMovieWriterController")
@CrossOrigin
public class SolrMovieWriterController extends SolrEntityWriter<Movie> {

    private static final Logger LOG = Loggers.getLogger(SolrMovieWriterController.class);

    @NonNull
    public Mono<ServerResponse> addMovie(final ServerRequest request) throws WebClientResponseException {
        LOG.info("got a request");
        return processTsvLinesRequest(request, "Movie");
    }

    @Override
    public Movie mapToEntity(final TsvLine tsvLine) {
        final Movie movie = new Movie();
        final List<String> entries = tsvLine.getEntries();
        movie.setTconst(entries.get(0));
        movie.setTitleType(getNullableValue(entries.get(1)));
        movie.setPrimaryTitle(getNullableValue(entries.get(2)));
        movie.setOriginalTitle(getNullableValue(entries.get(3)));
        movie.setAdult(getBoolean(entries.get(4)));
        movie.setStartYear(getInteger(entries.get(5)));
        movie.setEndYear(getInteger(entries.get(6)));
        movie.setRuntimeMinutes(getInteger(entries.get(7)));
        movie.setGenres(getList(entries.get(8)));
        movie.setId(movie.getTconst());

        return movie;
    }

    @RouterOperations({
            @RouterOperation(path = "/v2/api/movie",
                    method = POST,
                    operation = @Operation(operationId = "addMovieToSolr",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = TsvLines.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = String.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrMovieRoutes() {
        return route()
                .nest(RequestPredicates.path("/v2/api/"),
                        builder -> builder
                                .POST("movie", this::addMovie)
                                .build())
                .build();
    }
}
