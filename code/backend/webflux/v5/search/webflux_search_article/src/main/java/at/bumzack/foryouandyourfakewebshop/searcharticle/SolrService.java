package at.bumzack.foryouandyourfakewebshop.searcharticle;

import at.bumzack.common.solr.SolrReflectionThingi;
import at.bumzack.common.solr.SolrRequestBuilder;
import at.bumzack.common.solr.SolrResponse;
import at.bumzack.foryouandyourfakewebshop.ecom.model.SearchDoc;
import org.springframework.core.ParameterizedTypeReference;
import org.springframework.stereotype.Component;
import reactor.core.publisher.Mono;

import static at.bumzack.common.solr.SolrExecutor.executeSolrGet;


@Component
public class SolrService {


    public Mono<SolrResponse<SearchDoc>> searchByCode(final String searchText, final int pageSize, final ParameterizedTypeReference<SolrResponse<SearchDoc>> typeRef) {
        final var fieldNames = SolrReflectionThingi.getFieldNames(SearchDoc.class);

        final var url = new SolrRequestBuilder()
                .setCoreName("categories")
                .setQuery("code:" + searchText)
                .setRowCount(pageSize)
                .addResponseFields(fieldNames)
                .build();
        return executeSolrGet(typeRef, url);
    }

}
