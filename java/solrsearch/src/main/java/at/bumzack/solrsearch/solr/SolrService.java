package at.bumzack.solrsearch.solr;

import at.bumzack.solrsearch.SolrController;
import org.springframework.stereotype.Component;
import org.springframework.web.reactive.function.client.WebClient;
import reactor.core.publisher.Mono;
import reactor.util.Logger;


@Component
public class SolrService {
    private static final Logger LOG = reactor.util.Loggers.getLogger(SolrController.class);

    public Mono<SolrResponse> searchByCode(final String searchText, final int pageSize) {
        final var url = new SolrRequestBuilder()
                .setCoreName("products")
                .setQuery("*:*")
                .setRowCount(pageSize)
                .addResponseField("code")
                .addResponseField("articleName")
                .addResponseField("articleDescription")
                .addResponseField("visible")
                .addResponseField("orderable")
                .addResponseField("article")
                .addResponseField("articleUnit")
                .addResponseField("sourcing")
                .addResponseField("division")
                .addResponseField("material")
                .addResponseField("codeWhg")
                .addResponseField("supplierName")
                .addResponseField("defaultSupplier")
                .addResponseField("otns")
                .addResponseField("eans")
                .addResponseField("predecessorCodes")
                .addResponseField("predecessorEans")
                .addResponseField("predecessorOtns")
                .addResponseField("module")
                .addResponseField("moduleGroup")
                .addResponseField("ownBrand")
                .build();

        final var webClient = WebClient.create();

        final var responseSpec = webClient.get()
                .uri(url)
                .retrieve();
        LOG.info("searchByCode solr request URL   \n{}\n", url);

        return responseSpec.bodyToMono(SolrResponse.class);
    }

    public Mono<SolrResponse> searchTexts(final String searchText, final int rowCount) {
        final var url = new SolrRequestBuilder()
                .setCoreName("products")
                // TODO make query an array and join with ORs
                // articleName:*Scheibe* OR (material:*Sohn* AND material:*Schmitt*)
                .setQuery("articleName:*" + searchText + "* OR " + "articleDescription:*" + searchText + "*")
                .setRowCount(rowCount)
                .addQueryField("articleName", searchText)
                .addQueryField("articleDescription", searchText)
                .addResponseField("code")
                .addResponseField("articleName")
                .addResponseField("articleDescription")
                .addResponseField("visible")
                .addResponseField("orderable")
                .addResponseField("article")
                .addResponseField("articleUnit")
                .addResponseField("sourcing")
                .addResponseField("division")
                .addResponseField("material")
                .addResponseField("codeWhg")
                .addResponseField("supplierName")
                .addResponseField("defaultSupplier")
                .addResponseField("otns")
                .addResponseField("eans")
                .addResponseField("predecessorCodes")
                .addResponseField("predecessorEans")
                .addResponseField("predecessorOtns")
                .addResponseField("module")
                .addResponseField("moduleGroup")
                .addResponseField("ownBrand")
                .build();

        final var webClient = WebClient.create();

        final var responseSpec = webClient.get()
                .uri(url)
                .retrieve();
        LOG.info("searchByCode solr request URL   \n{}\n", url);

        return responseSpec.bodyToMono(SolrResponse.class);
    }
}
