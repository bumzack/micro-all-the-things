package at.bumzack.solr;


import at.bumzack.common.dto.Crew;
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

import static at.bumzack.common.tsv.TsvUtils.getList;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrCrewWriterController")
@CrossOrigin
public class SolrCrewWriterController extends SolrEntityWriter<Crew> {
    private static final Logger LOG = Loggers.getLogger(SolrCrewWriterController.class);

    @NonNull
    public Mono<ServerResponse> addCrew(final ServerRequest request) throws WebClientResponseException {
        LOG.info("got a request");
        return processTsvLinesRequest(request, "Crew");
    }

    @Override
    public Crew mapToEntity(final TsvLine tsvLine) {
        final Crew crew = new Crew();
        final List<String> entries = tsvLine.getEntries();
        crew.setTconst(entries.get(0));
        crew.setDirectors(getList(entries.get(1)));
        crew.setWriters(getList(entries.get(2)));
        crew.setId(crew.getTconst());

        return crew;
    }


    @RouterOperations({
            @RouterOperation(path = "/v2/api/crew",
                    method = POST,
                    operation = @Operation(operationId = "addCrewToSolr",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = TsvLines.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = String.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrCrewRoutes() {
        return route()
                .nest(RequestPredicates.path("/v2/api/"),
                        builder -> builder
                                .POST("crew", this::addCrew)
                                .build())
                .build();
    }
}
