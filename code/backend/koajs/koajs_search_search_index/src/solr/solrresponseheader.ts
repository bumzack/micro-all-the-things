import {SolrParams} from "./solrparams";

export interface SolrResponseHeader {
    satus: number,
    qtime: number,
    params: SolrParams,
}
