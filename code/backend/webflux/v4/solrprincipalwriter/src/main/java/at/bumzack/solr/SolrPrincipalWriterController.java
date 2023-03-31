package at.bumzack.solr;


import at.bumzack.common.dto.Principal;
import at.bumzack.common.dto.TsvLine;
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
import static at.bumzack.common.tsv.TsvUtils.getNullableValue;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrPrincipalWriterController")
@CrossOrigin
public class SolrPrincipalWriterController extends SolrEntityWriter<Principal> {
    private static final Logger LOG = Loggers.getLogger(SolrPrincipalWriterController.class);

    @NonNull
    public Mono<ServerResponse> addPrincipal(final ServerRequest request) throws WebClientResponseException {
        LOG.info("got a request");
        return processTsvLinesRequest(request, "Principal");
    }

    @Override
    public Principal mapToEntity(final TsvLine tsvLine) {
        final Principal principal = new Principal();
        final List<String> entries = tsvLine.getEntries();
        principal.setTconst(entries.get(0));
        principal.setOrdering(getNullableValue(entries.get(1)));
        principal.setNconst(getNullableValue(entries.get(2)));
        principal.setCategory(getNullableValue(entries.get(3)));
        principal.setCharacters(getList(entries.get(4)));
        principal.setId(principal.getTconst() + "_" + principal.getOrdering() + "_" + principal.getNconst());
        // LOG.info("processing principal {}", principal);
        return principal;
    }

    @RouterOperations({
            @RouterOperation(path = "v2/api/principal",
                    method = POST,
                    operation = @Operation(operationId = "addPrincipalToSolr",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = TsvLine.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = String.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrPrincipalRoutes() {
        return route()
                .nest(RequestPredicates.path("/v2/api/"),
                        builder -> builder
                                .POST("principal", this::addPrincipal)
                                .build())
                .build();
    }
}
