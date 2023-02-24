package at.bumzack.solrsearch.solr;

import com.fasterxml.jackson.annotation.JsonProperty;

//
//    {
//            "responseHeader":{
//            "status":0,
//            "QTime":0,
//            "params":{
//            "q":"code_string:973243",
//            "fl":"code_string",
//            "fq":["isVisible_boolean:true",
//            "sellable_boolean:true"],
//            "rows":"10",
//            "_":"1652274198070"}},
//            "response":{"numFound":0,"start":0,"docs":[]
//            }}

public class SolrResponse {

    @JsonProperty
    private SolrResponseHeader responseHeader;

    @JsonProperty
    private SolrResponseResponse response;

    public SolrResponse() {
    }

    public SolrResponseHeader getResponseHeader() {
        return responseHeader;
    }

    public void setResponseHeader(final SolrResponseHeader responseHeader) {
        this.responseHeader = responseHeader;
    }

    public SolrResponseResponse getResponse() {
        return response;
    }

    public void setResponse(final SolrResponseResponse response) {
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
