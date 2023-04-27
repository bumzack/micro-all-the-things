package at.bumzack.common.solr;

import org.apache.commons.lang3.StringUtils;

public class SolrUtils {
    public static String getSolrUrl(final String solrHost, final String solrPort, final String solrCore, final String command, final String schema) {
        return StringUtils.join(schema, "://", solrHost, ":", solrPort, "/solr/", solrCore, command);
    }
}
