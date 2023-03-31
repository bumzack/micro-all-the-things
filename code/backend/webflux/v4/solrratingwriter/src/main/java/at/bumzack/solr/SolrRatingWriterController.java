package at.bumzack.solr;


import at.bumzack.common.dto.Rating;
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

import static at.bumzack.common.tsv.TsvUtils.getNullableValue;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrRatingWriterController")
@CrossOrigin
public class SolrRatingWriterController extends SolrEntityWriter<Rating> {

    private static final Logger LOG = Loggers.getLogger(SolrRatingWriterController.class);

    @NonNull
    public Mono<ServerResponse> addRating(final ServerRequest request) throws WebClientResponseException {
        LOG.info("got a request");
        return processTsvLinesRequest(request, "Rating");
    }

    @Override
    public Rating mapToEntity(final TsvLine tsvLine) {
        final Rating rating = new Rating();
        final List<String> entries = tsvLine.getEntries();
        rating.setTconst(entries.get(0));
        rating.setAverageRating(getNullableValue(entries.get(1)));
        rating.setNumVotes(getNullableValue(entries.get(2)));
        rating.setId(rating.getTconst());

        return rating;
    }

    @RouterOperations({
            @RouterOperation(path = "/v2/api/rating",
                    method = POST,
                    operation = @Operation(operationId = "addRatingToSolr",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = TsvLines.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = String.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrRatingRoutes() {
        return route()
                .nest(RequestPredicates.path("/v2/api/"),
                        builder -> builder
                                .POST("rating", this::addRating)
                                .build())
                .build();
    }
}
