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
const generated_clients_customer_1 = require("./generated-clients-customer");
const jwt = require('jsonwebtoken');
const router = new koa_router_1.default();
const app = new koa_1.default();
app.use((0, koa_logger_1.default)());
app.use((0, koa_body_1.default)());
app.use(router.routes());
app.use(router.allowedMethods());
dotenv.config();
const port = process.env.PORT || 5000;
const secret = process.env.JWTSECRET || "fallbacksecret";
router.get('/api/v1/authenticated/:id', async (ctx, next) => {
    const client = await db.pool.connect();
    //  const loggedin = ctx.request.body as LoggedinRequest;
    const id = ctx.params.id;
    console.log(`authenticated?  id      ${id}`);
    try {
        const result = await client.query('SELECT * FROM authentication WHERE customer_id = $1::int AND  jwt IS NOT NULL', [id]);
        const authentication_entries = result.rows;
        console.log(`got an authentication_entries: ${JSON.stringify(authentication_entries, null, 4)}`);
        if (authentication_entries.length == 1) {
            ctx.body = JSON.stringify(authentication_entries[0]);
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
router.post('/api/v1/authentication/login', async (ctx, next) => {
    console.log(`ctx.request.body: ${JSON.stringify(ctx.request.body, null, 4)}`);
    const login_req = ctx.request.body;
    console.log(`got an login_req: ${JSON.stringify(login_req, null, 4)}`);
    if (login_req === undefined) {
        ctx.status = 404;
    }
    else {
        const client = await db.pool.connect();
        try {
            if (login_req.email === null || login_req.email === undefined) {
                ctx.status = 404;
            }
            else {
                const configParams = {
                    basePath: 'http://localhost:58980',
                };
                const apiConfig = new generated_clients_customer_1.Configuration(configParams);
                const api = new generated_clients_customer_1.DefaultApi(apiConfig);
                const r = {
                    email: login_req.email
                };
                const customer = await api.getCustomer(r);
                console.log(`got a customer: ${JSON.stringify(customer, null, 4)}`);
                const auth_entries = await client.query('SELECT * FROM authentication WHERE customer_id = $1::int', [customer.id]);
                const authentication_entries = auth_entries.rows;
                console.log(`got an authentication_entries: ${JSON.stringify(authentication_entries, null, 4)}`);
                if (authentication_entries.length == 1 && authentication_entries[0].jwt != null) {
                    // user already logged in
                    ctx.body = JSON.stringify(authentication_entries[0]);
                }
                else if (authentication_entries.length == 1 && authentication_entries[0].jwt == null) {
                    // log user in by updating existing record
                    if (customer.password === login_req.password && customer.email === login_req.email) {
                        // log user in
                        const id = authentication_entries[0].id;
                        const json_web_token = jwt.sign(customer, secret);
                        const update_return = await client.query("UPDATE  authentication " +
                            "SET jwt =  $1::text, logged_in =NOW()  WHERE id = $2::int    RETURNING *", [json_web_token, id]);
                        console.log(`response from UPDATE query  ${JSON.stringify(update_return, null, 4)}`);
                        // TODO not working
                        ctx.body = JSON.stringify(update_return.rows[0]);
                        // ctx.body = JSON.stringify(authentication_entries[0]);
                    }
                    else {
                        ctx.status = 404;
                    }
                }
                else if (authentication_entries.length == 0) {
                    // log user in by creating new  record
                    if (customer.password === login_req.password && customer.email === login_req.email) {
                        // log user in
                        const json_web_token = jwt.sign(customer, secret);
                        const update_return = await client.query("INSERT INTO authentication(customer_id, jwt, logged_in) " +
                            "VALUES($1::int, $2::text, NOW()) RETURNING *", [customer.id, json_web_token]);
                        console.log(`response from INSERT query  ${JSON.stringify(update_return, null, 4)}`);
                        ctx.body = JSON.stringify(update_return.rows[0]);
                    }
                    else {
                        ctx.status = 404;
                    }
                }
                else {
                    ctx.body = `more than 1 entry found for customer ${customer.id}-> that's not good`;
                    ctx.status = 500;
                }
            }
        }
        finally {
            client.release();
        }
    }
    await next;
});
router.post('/api/v1/authentication/logout', async (ctx, next) => {
    console.log(`ctx.request.body: ${JSON.stringify(ctx.request.body, null, 4)}`);
    const logout_req = ctx.request.body;
    console.log(`got an logout_req: ${JSON.stringify(logout_req, null, 4)}`);
    if (logout_req === undefined) {
        ctx.status = 404;
    }
    else {
        const client = await db.pool.connect();
        try {
            console.log(`trying to find an auth_entry for customer_id: ${JSON.stringify(logout_req.customerId, null, 4)}`);
            const auth_entries = await client.query('SELECT * FROM authentication WHERE customer_id = $1::int AND NOT jwt IS NULL ', [logout_req.customerId]);
            if (auth_entries === null || auth_entries.rows.length != 1) {
                ctx.status = 404;
            }
            else {
                const update_return = await client.query('UPDATE  authentication  SET jwt =  NULL,   logged_out = NOW()', []);
                console.log(`logout: update statement return : ${JSON.stringify(update_return, null, 4)}`);
                // TDOO this is the old one, we want the actual one, where the loggedout timestamp is set and the JWT == NULL
                ctx.body = JSON.stringify(auth_entries[0]);
            }
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