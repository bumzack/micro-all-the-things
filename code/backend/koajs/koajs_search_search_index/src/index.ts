import Router from 'koa-router';
import koaBody from 'koa-body';
import Koa from 'koa';
import logger from 'koa-logger';
import {MovieSearchResult, SearchDoc, SearchMovieIndexRequest} from "./generated-clients";
import * as dotenv from "dotenv";
import {SolrResponse} from "./solr/solrresponse";

const router = new Router();
const app = new Koa();

app.use(logger())
app.use(koaBody());
app.use(router.routes());
app.use(router.allowedMethods());

dotenv.config()

const port = process.env.PORT || 5000
const solr_host = process.env.SOLRHOST || "solr01.bumzack.at"
const solr_port = process.env.SOLRPORT || "80"
const solr_core = process.env.SOLRCORE || "searchindex"

router.post('/api/v1/solr/searchindex/search', async (ctx: Koa.Context, next: Koa.Next) => {
    const search_movie_request = ctx.request.body as SearchMovieIndexRequest;

    const fields: Array<string> = [];
    fields.push("titles");
    fields.push("actors");
    fields.push("directors");
    fields.push("genres");
    fields.push("characters");

    const query = fields.map(f => {
        return `${f}:${search_movie_request.q}`;
    })
        .join(" OR ");

    // console.log(`q = ${query}`);

    let url = `http://${solr_host}:${solr_port}/solr/${solr_core}/select`;
    // console.log(`url = ${url}`);

    const urlParams: Array<string> = [];
    if (search_movie_request.limit !== null) {
        urlParams.push(`limit=${search_movie_request.limit}`)
    }
    if (search_movie_request.offset !== null) {
        urlParams.push(`offset=${search_movie_request.offset}`)
    }
    urlParams.push(`q=${query}`);
    let params = urlParams.join("&");
    let full_url = `${url}?${params}`;
    console.log(`full_url = ${full_url}`);

    const response = await fetch(full_url);
    const body = await response.text();
    // console.log(`body   ${body}`);
    const solr_response = JSON.parse(body) as SolrResponse<SearchDoc>;
    console.log(`response from solr  ${JSON.stringify(solr_response)}`);
    const docs = solr_response.response.docs;
    const res: MovieSearchResult = {
        movies: docs,
    }

    ctx.body = JSON.stringify(res);

    await next;
});

app.listen(port, () => {
    console.log(`started. listening on port ${port}`);
});