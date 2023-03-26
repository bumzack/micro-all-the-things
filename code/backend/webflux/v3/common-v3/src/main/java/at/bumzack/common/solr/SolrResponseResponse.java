package at.bumzack.common.solr;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.List;

public class SolrResponseResponse<DOC_TYPE> {

    @JsonProperty
    private Integer numFound;

    @JsonProperty
    private Integer start;

    private List<DOC_TYPE> docs;


    public SolrResponseResponse() {
    }

    public Integer getNumFound() {
        return numFound;
    }

    public void setNumFound(final Integer numFound) {
        this.numFound = numFound;
    }

    public Integer getStart() {
        return start;
    }

    public void setStart(final Integer start) {
        this.start = start;
    }

    public List<DOC_TYPE> getDocs() {
        return docs;
    }

    public void setDocs(final List<DOC_TYPE> docs) {
        this.docs = docs;
    }

    @Override
    public String toString() {
        return "SolrResponseResponse{" +
                "numFound=" + numFound +
                ", start=" + start +
                ", docs=" + docs +
                '}';
    }
}
