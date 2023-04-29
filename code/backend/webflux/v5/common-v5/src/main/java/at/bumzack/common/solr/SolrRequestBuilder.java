package at.bumzack.common.solr;


import org.apache.commons.lang3.StringUtils;
import org.springframework.beans.factory.annotation.Value;
import reactor.util.Logger;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import static java.util.stream.Collectors.toList;

/**
 * BE CAREFUL: always provide at least 2 queryFields -> otherwise the deserialization crashes, as the DTO expects a List<String>
 * if only one queryField is provided, then Solr returns a simple String, not a List<String> and deserialization goes booom
 */
public class SolrRequestBuilder {
    private static final Logger LOG = reactor.util.Loggers.getLogger(SolrRequestBuilder.class);

    private static final String AMPERSAND = "&";
    private static final String SLASH = "/";
    private static final String SOLR = "solr";
    private final List<String> responseFields = new ArrayList<>();
    private final List<String> queryField = new ArrayList<>();
    private String solrHost = "localhost";

    @Value("${solr.port}")
    private Integer solrPort;

    @Value("${solr.core}")
    private String core;

    // private String host = "http://solr01.bumzack.at/solr";
    // private String port = "8984";
    // private String solrPath = "solr";
    private String command = "select";

    private String query = null;
    private Integer offset = 25;
    private Integer limit = 25;

    public SolrRequestBuilder setQuery(final String query) {
        this.query = query;
        return this;
    }

    public SolrRequestBuilder addResponseField(final String fieldName) {
        responseFields.add(fieldName);
        return this;
    }

    public SolrRequestBuilder addResponseFields(final List<String> fieldNames) {
        responseFields.addAll(fieldNames);
        return this;
    }

    public SolrRequestBuilder addQueryField(final String name, final String value) {
        queryField.add(StringUtils.join(name, ":", value));
        return this;
    }

    public SolrRequestBuilder setCoreName(final String core) {
        this.core = core;
        return this;
    }

    public SolrRequestBuilder setPort(final String port) {
        this.solrPort = Integer.parseInt(port);
        return this;
    }

    public SolrRequestBuilder setCommand(final String command) {
        this.command = command;
        return this;
    }


    public SolrRequestBuilder setOffset(final Integer offset) {
        this.offset = offset;
        return this;
    }

    public SolrRequestBuilder setLimit(final Integer limit) {
        this.limit = limit;
        return this;
    }

    public SolrRequestBuilder setHost(final String host) {
        this.solrHost = host;
        return this;
    }

    public String build() {
        final var host = StringUtils.join("http://", solrHost, ":", solrPort, "/");
        LOG.info("solrHost       {}\n", solrHost);
        LOG.info("solrPort       {}\n", solrPort);
        LOG.info("host            {}\n", host);

        final var url = Stream.of(host, SOLR, "/", core, command)
                .filter(Objects::nonNull)
                .collect(Collectors.joining(SLASH));

        final var q = Optional.of(query).map(q1 -> StringUtils.join("q=", q1)).orElse(null);
        final var fq = queryField.stream()
                .map(fieldQuery -> StringUtils.join("fq", "=", fieldQuery))
                .collect(toList());
        final var fl = StringUtils.join("fl", "=", String.join(",", responseFields));
        final var limit = StringUtils.join("limit", "=", String.valueOf(this.limit));
        final var offset = StringUtils.join("offset", "=", String.valueOf(this.offset));

        LOG.info("\nurl         \n{}\n", url);
        LOG.info("\nq           {}", q);
        LOG.info("\nfq          {}", String.join(" // ", fq));
        LOG.info("\nfl          {}", fl);

        final var urlParams = Stream.concat(Stream.of(q, fl, limit, offset), fq.stream())
                .map(StringUtils::trimToNull)
                .filter(Objects::nonNull)
                .collect(Collectors.joining(AMPERSAND));

        return StringUtils.joinWith("?", url, urlParams);
    }
}
