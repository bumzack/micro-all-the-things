export interface SolrParams {
    q: string,
    fl: string,
    fq: Array<string>;
    rows: number,
    timeStamp: string,
}
