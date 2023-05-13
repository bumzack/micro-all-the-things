"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const koa_router_1 = __importDefault(require("koa-router"));
const koa_body_1 = __importDefault(require("koa-body"));
const koa_1 = __importDefault(require("koa"));
const koa_logger_1 = __importDefault(require("koa-logger"));
const dotenv = __importStar(require("dotenv"));
const router = new koa_router_1.default();
const app = new koa_1.default();
app.use((0, koa_logger_1.default)());
app.use((0, koa_body_1.default)());
app.use(router.routes());
app.use(router.allowedMethods());
dotenv.config();
const port = process.env.PORT || 5000;
const solr_host = process.env.SOLRHOST || "solr01.bumzack.at";
const solr_port = process.env.SOLRPORT || "80";
const solr_core = process.env.SOLRCORE || "searchindex";
router.post('/api/v1/solr/searchindex/search', async (ctx, next) => {
    const search_movie_request = ctx.request.body;
    const fields = [];
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
    const urlParams = [];
    if (search_movie_request.limit !== null) {
        urlParams.push(`limit=${search_movie_request.limit}`);
    }
    if (search_movie_request.offset !== null) {
        urlParams.push(`offset=${search_movie_request.offset}`);
    }
    urlParams.push(`q=${query}`);
    let params = urlParams.join("&");
    let full_url = `${url}?${params}`;
    console.log(`full_url = ${full_url}`);
    const response = await fetch(full_url);
    const body = await response.text();
    // console.log(`body   ${body}`);
    const solr_response = JSON.parse(body);
    console.log(`response from solr  ${JSON.stringify(solr_response)}`);
    const docs = solr_response.response.docs;
    const res = {
        movies: docs,
    };
    ctx.body = JSON.stringify(res);
    await next;
});
app.listen(port, () => {
    console.log(`started. listening on port ${port}`);
});
//# sourceMappingURL=index.js.map