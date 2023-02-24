package at.bumzack.solrsearch.solr;

import com.fasterxml.jackson.annotation.JsonProperty;

public class SolrResponseHeader {

    @JsonProperty
    private Integer status;

    @JsonProperty
    private Integer QTime;

    @JsonProperty
    private SolrParams params;

    public SolrResponseHeader() {
    }

    public Integer getStatus() {
        return status;
    }

    public void setStatus(final Integer status) {
        this.status = status;
    }

    public Integer getQTime() {
        return QTime;
    }

    public void setQTime(final Integer QTime) {
        this.QTime = QTime;
    }

    public SolrParams getParams() {
        return params;
    }

    public void setParams(final SolrParams params) {
        this.params = params;
    }

    @Override
    public String toString() {
        return "ResponseHeader{" +
                "status=" + status +
                ", QTime=" + QTime +
                ", params=" + params +
                '}';
    }
}
