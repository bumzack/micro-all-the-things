package at.bumzack.solrsearch;


import at.bumzack.dto.Image;
import at.bumzack.solr.SolrReflectionThingi;
import at.bumzack.solr.SolrRequestBuilder;
import at.bumzack.solr.SolrResponse;
import org.springframework.core.ParameterizedTypeReference;
import org.springframework.stereotype.Component;
import reactor.core.publisher.Mono;
import reactor.util.Logger;

import static at.bumzack.solr.SolrExecutor.executeSolrGet;
import static at.bumzack.webflux.WebClientFactory.getWebClient;


@Component
public class SolrService {
    private static final Logger LOG = reactor.util.Loggers.getLogger(SolrService.class);

    public Mono<SolrResponse<Image>> searchByMediaContainerQualifier(final String mediaContainerQualifier, final ParameterizedTypeReference<SolrResponse<Image>> typeRef) {
        final var fieldNames = SolrReflectionThingi.getFieldNames(Image.class);
        final var url = new SolrRequestBuilder()
                .setCoreName("images")
                .setQuery("mediaContainerQualifier:" + mediaContainerQualifier)
                .addResponseFields(fieldNames)
                .build();
        return executeSolrGet(typeRef, url);
    }
}
