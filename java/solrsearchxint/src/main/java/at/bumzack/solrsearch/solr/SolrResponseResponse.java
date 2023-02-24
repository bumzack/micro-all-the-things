package at.bumzack.solrsearch.solr;

import at.bumzack.solrsearch.dto.XinetData;
import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.List;

public class SolrResponseResponse {

    @JsonProperty
    private Integer numFound;

    @JsonProperty
    private Integer start;

    private List<XinetData> docs;


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

    public List<XinetData> getDocs() {
        return docs;
    }

    public void setDocs(final List<XinetData> docs) {
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
