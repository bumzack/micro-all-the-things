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
router.get("/api/v1/customerprices/:customerId", async (ctx, next) => {
    const client = await db.pool.connect();
    const customerId = ctx.params.customerId;
    console.log(`customerId ${customerId} `);
    try {
        const result = await client.query('SELECT * FROM customer_price WHERE customer_id = $1::int ', [customerId]);
        const customer_prices = result.rows;
        console.log(`got customer_prices entries: ${JSON.stringify(customer_prices, null, 4)}`);
        ctx.body = JSON.stringify(customer_prices);
    }
    finally {
        client.release();
    }
    await next;
});
app.listen(port, () => {
    console.log(`started. listening on port ${port}`);
});
//# sourceMappingURL=index.js.map