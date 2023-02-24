package at.bumzack.solrsearch.solr;

import at.bumzack.solrsearch.dto.B2BArticleChannelInfoData;
import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.Collection;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

public class SolrResponseResponse {

    @JsonProperty
    private Integer numFound;

    @JsonProperty
    private Integer start;

    private List<B2BArticleChannelInfoData> docs;


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

    public List<B2BArticleChannelInfoData> getDocs() {
        return docs;
    }

    public void setDocs(final List<B2BArticleChannelInfoData> docs) {
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
