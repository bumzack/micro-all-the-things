package at.bumzack.solr;

public class SolrWriter {

    private String host;
    private String port;
    private String url;

    public SolrWriter(final String host, final String port, final String url) {
        this.host = host;
        this.port = port;
        this.url = url;
    }

    public String getHost() {
        return host;
    }

    public void setHost(final String host) {
        this.host = host;
    }

    public String getPort() {
        return port;
    }

    public void setPort(final String port) {
        this.port = port;
    }

    public String getUrl() {
        return url;
    }

    public void setUrl(final String url) {
        this.url = url;
    }

    public String getRequestUrl() {
        return "http://" + host + ":" + port + "/" + url;
    }

    @Override
    public String toString() {
        return "SolrWriter{" +
                "host='" + host + '\'' +
                ", port='" + port + '\'' +
                ", url='" + url + '\'' +
                '}';
    }
}
