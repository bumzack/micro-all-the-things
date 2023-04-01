package at.bumzack.solr;


import at.bumzack.common.dto.MovieAkas;
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

@Controller("SolrMovieAkasWriterController")
@CrossOrigin
public class SolrMovieAkasWriterController extends SolrEntityWriter<MovieAkas> {

    private static final Logger LOG = Loggers.getLogger(SolrMovieAkasWriterController.class);

    @NonNull
    public Mono<ServerResponse> addMovieAka(final ServerRequest request) throws WebClientResponseException {
        LOG.info("got a request");
        return processTsvLinesRequest(request, "MovieAkas");
    }

    @Override
    public MovieAkas mapToEntity(final TsvLine tsvLine) {
        final MovieAkas movieAkas = new MovieAkas();
        final List<String> entries = tsvLine.getEntries();
        movieAkas.setTitleId(entries.get(0));
        movieAkas.setOrdering(getInteger(entries.get(1)));
        movieAkas.setTitle(getNullableValue(entries.get(2)));
        movieAkas.setRegion(getNullableValue(entries.get(3)));
        movieAkas.setLanguage(getNullableValue(entries.get(4)));
        movieAkas.setTypes(getList(entries.get(5)));
        movieAkas.setAttributes(getList(entries.get(6)));
        movieAkas.setOriginalTitle(getBoolean(entries.get(7)));
        movieAkas.setId(String.join("_", movieAkas.getTitleId(), movieAkas.getOrdering().toString()));
        return movieAkas;
    }


    @RouterOperations({
            @RouterOperation(path = "/v2/api/movieaka",
                    method = POST,
                    operation = @Operation(operationId = "addMovieAkaToSolr",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = TsvLines.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = String.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrMovieAkaRoutes() {
        return route()
                .nest(RequestPredicates.path("/v2/api/"),
                        builder -> builder
                                .POST("movieaka", this::addMovieAka)
                                .build())
                .build();
    }
}
