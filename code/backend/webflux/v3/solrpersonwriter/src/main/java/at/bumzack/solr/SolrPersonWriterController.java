package at.bumzack.solr;


import at.bumzack.common.dto.Person;
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

import static at.bumzack.common.tsv.TsvUtils.getList;
import static at.bumzack.common.tsv.TsvUtils.getNullableValue;
import static java.util.Objects.isNull;
import static org.springframework.web.bind.annotation.RequestMethod.POST;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Controller("SolrPersonWriterController")
@CrossOrigin
public class SolrPersonWriterController {

    private static final String SCHEMA = "http";
    private static final String COMMAND = "/update?commitWithin=100&overwrite=true&wt=json";
    private static final Logger LOG = Loggers.getLogger(SolrPersonWriterController.class);

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
    public Mono<ServerResponse> addPerson(final ServerRequest request) throws WebClientResponseException {
        final var tsvLine = request
                .bodyToMono(TsvLine.class)
//                .doOnSuccess(tsv -> {
//                    LOG.info("processing person {}", tsv);
//                })
                .doOnError(e -> {
                    LOG.error("error    {}", e);
                });
        final var webClient = WebClientFactory.getClient();

        return tsvLine
                .map(this::mapToPerson)
                .flatMap(a -> execSolrPost(webClient, a));
    }

    private Person mapToPerson(final TsvLine tsvLine) {
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

    private Mono<ServerResponse> execSolrPost(final WebClient webClient, final Person person) {
        final var url = getSolrUrl();

        return webClient.post()
                .uri(url)
                .contentType(MediaType.APPLICATION_JSON)
                .accept()
                .body(BodyInserters.fromValue(List.of(person)))
                .retrieve()
                .bodyToMono(String.class)
                //  .doOnNext(e -> LOG.info("movie solr response {}", e))
                .doOnError(e -> LOG.error("person  error from Solr '{}'", e.getMessage()))
//                .doOnSuccess(s -> {
//                    // LOG.info("movie solr success {}", s);
//                })
                .flatMap(e -> ServerResponse.ok().body(BodyInserters.fromValue("person  SolrWriter says: all good")));
    }

    @RouterOperations({
            @RouterOperation(path = "/api/person",
                    method = POST,
                    operation = @Operation(operationId = "addPersonToSolr",
                            requestBody = @RequestBody(content = @Content(schema = @Schema(implementation = TsvLine.class))),
                            responses = {@ApiResponse(responseCode = "200", content = @Content(schema = @Schema(implementation = String.class))),
                            })
            )
    })

    @Bean
    public RouterFunction<ServerResponse> solrPersonRoutes() {
        return route()
                .nest(RequestPredicates.path("/api/"),
                        builder -> builder
                                .POST("person", this::addPerson)
                                .build())
                .build();
    }
}
