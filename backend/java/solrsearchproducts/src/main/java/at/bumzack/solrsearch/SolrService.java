package at.bumzack.solrsearch;

import at.bumzack.dto.Image;
import at.bumzack.dto.Product;
import at.bumzack.solr.SolrReflectionThingi;
import at.bumzack.solr.SolrRequestBuilder;
import at.bumzack.solr.SolrResponse;
import org.springframework.core.ParameterizedTypeReference;
import org.springframework.stereotype.Component;
import reactor.core.publisher.Mono;
import reactor.util.Logger;
import reactor.util.Loggers;

import static at.bumzack.solr.SolrExecutor.executeSolrGet;


@Component
public class SolrService {
    private static final Logger LOG = Loggers.getLogger(SolrService.class);

    public Mono<SolrResponse<Product>> searchByCode(final String searchText, final int pageSize, final ParameterizedTypeReference<SolrResponse<Product>> typeRef) {
        final var fieldNames = SolrReflectionThingi.getFieldNames(Image.class);
        final var url = new SolrRequestBuilder()
                .setCoreName("products")
                .setQuery("code:" + searchText)
                .setRowCount(pageSize)
                .addResponseFields(fieldNames)
                .build();

        return executeSolrGet(typeRef, url);
    }

    public Mono<SolrResponse<Product>> searchTexts(final String searchText, final int rowCount, final ParameterizedTypeReference<SolrResponse<Product>> typeRef) {
        final var fieldNames = SolrReflectionThingi.getFieldNames(Image.class);
        final var url = new SolrRequestBuilder()
                .setCoreName("products")
                // TODO make query an array and join with ORs
                // articleName:*Scheibe* OR (material:*Sohn* AND material:*Schmitt*)
                .setQuery("articleName:*" + searchText + "* OR " + "articleDescription:*" + searchText + "*")
                .setRowCount(rowCount)
                .addResponseFields(fieldNames)
                .build();
        return executeSolrGet(typeRef, url);
    }
}

