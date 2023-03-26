package at.bumzack.common.solr;

import com.fasterxml.jackson.annotation.JsonProperty;


public class SolrResponse<DOC_TYPE> {

    @JsonProperty
    private SolrResponseHeader responseHeader;

    @JsonProperty
    private SolrResponseResponse<DOC_TYPE> response;

    public SolrResponse() {
    }

    public SolrResponseHeader getResponseHeader() {
        return responseHeader;
    }

    public void setResponseHeader(final SolrResponseHeader responseHeader) {
        this.responseHeader = responseHeader;
    }

    public SolrResponseResponse<DOC_TYPE> getResponse() {
        return response;
    }

    public void setResponse(final SolrResponseResponse<DOC_TYPE> response) {
        this.response = response;
    }

    @Override
    public String toString() {
        return "SolrResponse{" +
                "responseHeader=" + responseHeader +
                ", response=" + response +
                '}';
    }
}
