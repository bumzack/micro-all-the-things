package at.bumzack.solrsearch.solr;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.List;


public class SolrParams {

    @JsonProperty
    private String q;

    @JsonProperty
    private String fl;

    @JsonProperty
    private List<String> fq;

    @JsonProperty
    private Integer rows;

    @JsonProperty("_")
    private Integer timeStamp;

    public SolrParams() {
    }

    public String getQ() {
        return q;
    }

    public void setQ(final String q) {
        this.q = q;
    }

    public String getFl() {
        return fl;
    }

    public void setFl(final String fl) {
        this.fl = fl;
    }

    public Object getFq() {
        return fq;
    }

    public void setFq(final List<String> fq) {
        this.fq = fq;
    }

    public Integer getRows() {
        return rows;
    }

    public void setRows(final Integer rows) {
        this.rows = rows;
    }

    public Integer getTimeStamp() {
        return timeStamp;
    }

    public void setTimeStamp(final Integer timeStamp) {
        this.timeStamp = timeStamp;
    }

    @Override
    public String toString() {
        return "SolrParams{" +
                "q='" + q + '\'' +
                ", fl='" + fl + '\'' +
                ", fq=" + fq +
                ", rows=" + rows +
                ", timeStamp=" + timeStamp +
                '}';
    }
}
