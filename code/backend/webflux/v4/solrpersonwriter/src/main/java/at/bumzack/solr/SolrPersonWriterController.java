package at.bumzack.solr;


import at.bumzack.common.dto.Person;
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
import static at.bumzack.common.tsv.TsvUtils.getNullableValue;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrPersonWriterController")
@CrossOrigin
public class SolrPersonWriterController extends SolrEntityWriter<Person> {

    private static final Logger LOG = Loggers.getLogger(SolrPersonWriterController.class);

    @NonNull
    public Mono<ServerResponse> addPerson(final ServerRequest request) throws WebClientResponseException {
        LOG.info("got a request");
        return processTsvLinesRequest(request, "Person");
    }

    @Override
    public Person mapToEntity(final TsvLine tsvLine) {
        final Person person = new Person();
        final List<String> entries = tsvLine.getEntries();
        person.setNconst(entries.get(0));
        person.setPrimaryName(getNullableValue(entries.get(1)));
        person.setBirthYear(getNullableValue(entries.get(2)));
        person.setDeathYear(getNullableValue(entries.get(3)));
        person.setPrimaryProfession(getList(entries.get(4)));
        person.setKnownForTitles(getList(entries.get(5)));
        person.setId(person.getNconst());

        return person;
    }


    @RouterOperations({
            @RouterOperation(path = "/v2/api/person",
                    method = POST,
                    operation = @Operation(operationId = "addPersonToSolr",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = TsvLines.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = String.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrPersonRoutes() {
        return route()
                .nest(RequestPredicates.path("/v2/api/"),
                        builder -> builder
                                .POST("person", this::addPerson)
                                .build())
                .build();
    }
}
