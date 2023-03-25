package at.bumzack.solr;


import org.apache.commons.lang3.StringUtils;
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
    private final List<String> responseFields = new ArrayList<>();
    private final List<String> queryField = new ArrayList<>();
    private String host = "http://127.0.0.1:8984/solr";
    private String port = "8984";
    private String solrPath = "solr";
    private String command = "select";
    private String core;

    private String query = null;
    private Integer rowCount = 25;

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
        this.port = port;
        return this;
    }

    public SolrRequestBuilder setCommand(final String command) {
        this.command = command;
        return this;
    }

    public SolrRequestBuilder setRowCount(final Integer rowCount) {
        this.rowCount = rowCount;
        return this;
    }

    public SolrRequestBuilder setHost(final String host) {
        this.host = host;
        return this;
    }

    public String build() {
        final var url = Stream.of(host, core, command)
                .filter(Objects::nonNull)
                .collect(Collectors.joining(SLASH));

        final var q = Optional.of(query).map(q1 -> StringUtils.join("q=", q1)).orElse(null);
        final var fq = queryField.stream()
                .map(fieldQuery -> StringUtils.join("fq", "=", fieldQuery))
                .collect(toList());
        final var fl = StringUtils.join("fl", "=", String.join(",", responseFields));
        final var rows = StringUtils.join("rows", "=", String.valueOf(rowCount));

        LOG.info("\nurl         \n{}\n", url);
        LOG.info("\nq           {}", q);
        LOG.info("\nfq          {}", String.join(" // ", fq));
        LOG.info("\nfl          {}", fl);

        final var urlParams = Stream.concat(Stream.of(q, fl, rows), fq.stream())
                .map(StringUtils::trimToNull)
                .filter(Objects::nonNull)
                .collect(Collectors.joining(AMPERSAND));

        return StringUtils.joinWith("?", url, urlParams);
    }
}
