package at.bumzack.solrsearch;

import at.bumzack.dto.Category;
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

    public Mono<SolrResponse<Category>> searchByCode(final String searchText, final int pageSize, final ParameterizedTypeReference<SolrResponse<Category>> typeRef) {
        final var fieldNames = SolrReflectionThingi.getFieldNames(Category.class);

        final var url = new SolrRequestBuilder()
                .setCoreName("categories")
                .setQuery("code:" + searchText)
                .setRowCount(pageSize)
                .addResponseFields(fieldNames)
                .build();
        return executeSolrGet(typeRef, url);
    }

    public Mono<SolrResponse<Category>> allCategories(final ParameterizedTypeReference<SolrResponse<Category>> typeRef) {
        final var fieldNames = SolrReflectionThingi.getFieldNames(Category.class);
        final var url = new SolrRequestBuilder()
                .setCoreName("categories")
                .setQuery("*:*")
                .setRowCount(1000000)
                .addResponseFields(fieldNames)
                .build();
        return executeSolrGet(typeRef, url);
    }

    public Mono<SolrResponse<Category>> rootCategories(final ParameterizedTypeReference<SolrResponse<Category>> typeRef) {
        final var fieldNames = SolrReflectionThingi.getFieldNames(Category.class);
        final var url = new SolrRequestBuilder()
                .setCoreName("categories")
                .setQuery("-superCategories:*")
                .setRowCount(1000000)
                .addResponseFields(fieldNames)
                .build();

        return executeSolrGet(typeRef, url);
    }

    public Mono<SolrResponse<Category>> children(final String code, final ParameterizedTypeReference<SolrResponse<Category>> typeRef) {
        final var fieldNames = SolrReflectionThingi.getFieldNames(Category.class);
        final var url = new SolrRequestBuilder()
                .setCoreName("categories")
                .setQuery("superCategories:" + code)
                .setRowCount(1000000)
                .addResponseFields(fieldNames)
                .build();

        return executeSolrGet(typeRef, url);
    }


}
