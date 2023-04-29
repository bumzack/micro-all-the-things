package at.bumzack.common.entitywriter;


import at.bumzack.common.solr.SolrUtils;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.http.MediaType;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.client.WebClient;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.Loggers;

import java.util.List;

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
