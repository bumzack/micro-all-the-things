package at.bumzack.common.entitywriter;


import at.bumzack.common.dto.TsvLine;
import at.bumzack.common.dto.TsvLines;
import at.bumzack.common.solr.SolrUtils;
import at.bumzack.common.webflux.WebClientFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.http.MediaType;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClient;
import org.springframework.web.reactive.function.client.WebClientResponseException;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.Loggers;
import reactor.util.annotation.NonNull;

import java.util.List;
import java.util.stream.Collectors;

import static java.util.Objects.isNull;

public abstract class SolrEntityWriter<ENTITY> {
    public static final String SCHEMA = "http";
    public static final String COMMAND = "/update?commitWithin=100&overwrite=true&wt=json";
    private static final Logger LOG = Loggers.getLogger(SolrEntityWriter.class);
    @Value("${solr.host}")
    public String solrHost;

    @Value("${solr.port}")
    public String solrPort;

    @Value("${solr.core}")
    public String solrCore;

    public String solrUrl = null;

    public String getSolrUrl() {
        if (isNull(solrUrl)) {
            solrUrl = SolrUtils.getSolrUrl(solrHost, solrPort, solrCore, COMMAND, SCHEMA);
        }
        return solrUrl;
    }

    @NonNull
    public Mono<ServerResponse> processTsvLinesRequest(final ServerRequest request, final String entityName) throws WebClientResponseException {
        LOG.info("got a request");
        final var webClient = WebClientFactory.getClient();
        return request
                .bodyToMono(TsvLines.class)
//                .doOnSuccess(tsv -> {
//                    LOG.info("processing principal lines {}", tsv);
//                })
                .doOnError(e -> {
                    LOG.error("error    {}", e);
                })
                .map(this::mapToEntityList)
                .flatMap(a -> execSolrPost(webClient, a, entityName));
    }

    private List<ENTITY> mapToEntityList(final TsvLines tsvLines) {
        return tsvLines.getLines().stream()
                .map(this::mapToEntity)
                .collect(Collectors.toList());
    }

    public abstract ENTITY mapToEntity(final TsvLine tsvLine);

    private Mono<ServerResponse> execSolrPost(final WebClient webClient, final List<ENTITY> entities, final String entityName) {
        final var url = getSolrUrl();
        final String msg = "Entity: " + entityName + ": SolrWriter says: all good";

        return webClient.post()
                .uri(url)
                .contentType(MediaType.APPLICATION_JSON)
                .accept()
                .body(BodyInserters.fromValue(entities))
                .retrieve()
                .bodyToMono(String.class)
                .doOnNext(e -> LOG.info("{}} solr response {}", entityName, e))
                .doOnError(e -> LOG.error("{}  error from Solr '{}'", entityName, e.getMessage()))
                .doOnSuccess(s -> {
                    LOG.info("{} solr success {}", entityName, s);
                })
                .flatMap(e -> ServerResponse.ok().body(BodyInserters.fromValue(msg)));
    }
}
