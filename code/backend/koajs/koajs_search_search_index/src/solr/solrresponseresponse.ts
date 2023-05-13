export interface SolrResponseResponse<DOC_TYPE> {
    num_found: number,
    start: number,
    docs: Array<DOC_TYPE>,
}
