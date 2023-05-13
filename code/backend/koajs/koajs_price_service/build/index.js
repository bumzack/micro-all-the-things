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
const db = __importStar(require("./db"));
const dotenv = __importStar(require("dotenv"));
const router = new koa_router_1.default();
const app = new koa_1.default();
app.use((0, koa_logger_1.default)());
app.use((0, koa_body_1.default)());
app.use(router.routes());
app.use(router.allowedMethods());
dotenv.config();
const port = process.env.PORT || 5000;
router.get("/api/v1/price/:tconst", async (ctx, next) => {
    const client = await db.pool.connect();
    //  const loggedin = ctx.request.body as LoggedinRequest;
    const tconst = ctx.params.tconst;
    console.log(`tconst ${tconst} `);
    try {
        const result = await client.query('SELECT * FROM price WHERE movie_tconst = $1::text ', [tconst]);
        const prices = result.rows;
        console.log(`got price entries: ${JSON.stringify(prices, null, 4)}`);
        if (prices.length == 1) {
            ctx.body = JSON.stringify(prices[0]);
        }
        else {
            ctx.status = 404;
        }
    }
    finally {
        client.release();
    }
    await next;
});
router.post('/api/v2/prices', async (ctx, next) => {
    console.log(`ctx.request.body: ${JSON.stringify(ctx.request.body, null, 4)}`);
    const search_prices_request = ctx.request.body;
    console.log(`got a search_prices_request: ${JSON.stringify(search_prices_request, null, 4)}`);
    if (search_prices_request === undefined) {
        ctx.status = 404;
    }
    else {
        const client = await db.pool.connect();
        try {
            const ids = new Set();
            search_prices_request.movieTconst?.forEach(tconst => {
                ids.add(tconst);
            });
            const ids_list = [];
            ids.forEach(i => ids_list.push(i));
            const update_return = await client.query("SELECT * FROM price WHERE movie_tconst = ANY ($1) ", [ids_list]);
            console.log(`response from UPDATE query  ${JSON.stringify(update_return, null, 4)}`);
            const prices = update_return.rows;
            ctx.body = JSON.stringify(prices);
        }
        finally {
            client.release();
        }
    }
    await next;
});
app.listen(port, () => {
    console.log(`started. listening on port ${port}`);
});
//# sourceMappingURL=index.js.map