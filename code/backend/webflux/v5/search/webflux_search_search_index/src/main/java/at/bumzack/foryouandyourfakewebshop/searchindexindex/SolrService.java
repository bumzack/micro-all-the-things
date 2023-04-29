package at.bumzack.foryouandyourfakewebshop.searchindexindex;

import at.bumzack.common.solr.SolrReflectionThingi;
import at.bumzack.common.solr.SolrRequestBuilder;
import at.bumzack.common.solr.SolrResponse;
import at.bumzack.foryouandyourfakewebshop.searchindexindex.model.SearchDoc;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.core.ParameterizedTypeReference;
import org.springframework.stereotype.Component;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.Loggers;

import java.util.stream.Collectors;

import static at.bumzack.common.solr.SolrExecutor.executeSolrGet;
import static java.util.Objects.nonNull;


@Component
public class SolrService {
    private static final Logger LOG = Loggers.getLogger(SolrService.class);

    @Value("${solr.host}")
    private String solrHost;

    @Value("${solr.port}")
    private Integer solrPort;

    public Mono<SolrResponse<SearchDoc>> searchByText(final String searchText, final int limit, final int offset, final ParameterizedTypeReference<SolrResponse<SearchDoc>> typeRef) {
        final var fieldNames = SolrReflectionThingi.getFieldNames(SearchDoc.class, String.class);

        final var q = fieldNames.stream()
                .map(name -> String.join(":", name, searchText))
                .collect(Collectors.joining(" OR "));

        LOG.info("Solr search query for String fields... i hope.  q = {}", q);

        final var builder = new SolrRequestBuilder()
                .setCoreName("searchindex")
                .setQuery(q)
                .setOffset(offset)
                .setLimit(limit);
        if (nonNull(solrHost)) {
            builder.setHost(solrHost);
        }

        if (nonNull(solrPort)) {
            builder.setPort(solrPort.toString());
        }

        final var url = builder
                .build();

        return executeSolrGet(typeRef, url);
    }
}
