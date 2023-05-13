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
const services_auth_1 = require("./services_auth");
const services_index_1 = require("./services_index");
const services_prices_1 = require("./services_prices");
const services_customerprices_1 = require("./services_customerprices");
const router = new koa_router_1.default();
const app = new koa_1.default();
app.use((0, koa_logger_1.default)());
app.use((0, koa_body_1.default)());
app.use(router.routes());
app.use(router.allowedMethods());
dotenv.config();
const port = process.env.PORT || 5000;
router.post('/api/v1/solr/article', async (ctx, next) => {
    const req = ctx.request.body;
    console.log(`req ${JSON.stringify(req, null, 4)}`);
    const res = [];
    if (req.customer !== null && req.customer?.customerId != null) {
        console.log(`got a customer and an id   ${req.customer?.customerId}`);
        const auth = await (0, services_auth_1.is_authenticated)(req.customer.customerId);
        console.log(`auth    ${JSON.stringify(auth)}`);
        const logged_in = auth.jwt != null;
        console.log(`logged_in    ${logged_in}`);
        let customer_prices = [];
        if (logged_in) {
            customer_prices = await (0, services_customerprices_1.find_customer_prices)(req.customer.customerId);
        }
        console.log(`customer_prices    ${JSON.stringify(customer_prices)}`);
        let movies = await (0, services_index_1.search_movies)(req.q, req.offset, req.limit);
        if (movies.movies != undefined) {
            let tconsts = movies.movies.map(m => m.tconst);
            let prices_tmp = await (0, services_prices_1.find_prices)(tconsts);
            console.log(`prices_tmp    ${JSON.stringify(prices_tmp)}`);
            let prices = new Map();
            prices_tmp.forEach(p => prices.set(p.movieTconst, p));
            movies.movies.forEach(m => {
                const p = prices.get(m.tconst);
                let cp = undefined;
                if (logged_in) {
                    const customer_price = customer_prices.find(p => {
                        return p.startYear <= m.year && m.year <= p.endYear;
                    });
                    if (customer_price !== undefined) {
                        cp = (100.0 - customer_price.discount) * p.amount / 100;
                    }
                }
                const entry = {
                    article: m,
                    price: p.amount,
                    customerPrice: cp,
                };
                res.push(entry);
            });
        }
    }
    ctx.body = JSON.stringify(res);
    await next;
});
app.listen(port, () => {
    console.log(`started. listening on port ${port}`);
});
//# sourceMappingURL=index.js.map