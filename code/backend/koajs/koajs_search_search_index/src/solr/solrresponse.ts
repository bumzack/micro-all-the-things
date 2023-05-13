import {SolrResponseResponse} from "./solrresponseresponse";
import {SolrResponseHeader} from "./solrresponseheader";

export interface SolrResponse<DOC_TYPE> {
    responseHeader: SolrResponseHeader,
    response: SolrResponseResponse<DOC_TYPE>,
}
