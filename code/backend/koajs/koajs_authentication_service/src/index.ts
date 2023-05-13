import Router from 'koa-router';
import koaBody from 'koa-body';
import Koa from 'koa';
import logger from 'koa-logger';
import * as db from './db';
import {AuthenticationEntry, LogInRequest, LogOutRequest} from "./generated-clients";
import * as dotenv from "dotenv";
import {Configuration, ConfigurationParameters, DefaultApi, GetCustomerRequest} from "./generated-clients-customer";

const jwt = require('jsonwebtoken');

const router = new Router();
const app = new Koa();

app.use(logger())
app.use(koaBody());
app.use(router.routes());
app.use(router.allowedMethods());

dotenv.config()

const port = process.env.PORT || 5000

const secret = process.env.JWTSECRET || "fallbacksecret"

router.get('/api/v1/authenticated/:id', async (ctx: Koa.Context, next: Koa.Next) => {
    const client = await db.pool.connect();
    //  const loggedin = ctx.request.body as LoggedinRequest;
    const id = ctx.params.id;
    console.log(`authenticated?  id      ${id}`)

    try {
        const result = await client.query('SELECT * FROM authentication WHERE customer_id = $1::int AND  jwt IS NOT NULL', [id]);
        const authentication_entries = result.rows as Array<AuthenticationEntry>;
        console.log(`got an authentication_entries: ${JSON.stringify(authentication_entries, null, 4)}`);
        if (authentication_entries.length == 1) {
            ctx.body = JSON.stringify(authentication_entries[0]);
        } else {
            ctx.status = 404
        }
    } finally {
        client.release()
    }
    await next;
});

router.post('/api/v1/authentication/login', async (ctx: Koa.Context, next: Koa.Next) => {
    console.log(`ctx.request.body: ${JSON.stringify(ctx.request.body, null, 4)}`);
    const login_req = ctx.request.body as LogInRequest;
    console.log(`got an login_req: ${JSON.stringify(login_req, null, 4)}`);
    if (login_req === undefined) {
        ctx.status = 404;
    } else {
        const client = await db.pool.connect();
        try {
            if (login_req.email === null || login_req.email === undefined) {
                ctx.status = 404;
            } else {
                const configParams: ConfigurationParameters = {
                    basePath: 'http://localhost:58980',
                };
                const apiConfig = new Configuration(configParams);
                const api = new DefaultApi(apiConfig);
                const r: GetCustomerRequest = {
                    email: login_req.email
                };

                const customer = await api.getCustomer(r);
                console.log(`got a customer: ${JSON.stringify(customer, null, 4)}`);
                const auth_entries = await client.query('SELECT * FROM authentication WHERE customer_id = $1::int', [customer.id]);
                const authentication_entries = auth_entries.rows as Array<AuthenticationEntry>;

                console.log(`got an authentication_entries: ${JSON.stringify(authentication_entries, null, 4)}`);
                if (authentication_entries.length == 1 && authentication_entries[0].jwt != null) {
                    // user already logged in
                    ctx.body = JSON.stringify(authentication_entries[0]);
                } else if (authentication_entries.length == 1 && authentication_entries[0].jwt == null) {
                    // log user in by updating existing record
                    if (customer.password === login_req.password && customer.email === login_req.email) {
                        // log user in
                        const id = authentication_entries[0].id as number;
                        const json_web_token = jwt.sign(customer, secret);
                        const update_return = await client.query("UPDATE  authentication " +
                            "SET jwt =  $1::text, logged_in =NOW()  WHERE id = $2::int    RETURNING *",
                            [json_web_token, id]);
                        console.log(`response from UPDATE query  ${JSON.stringify(update_return, null, 4)}`);
                        // TODO not working
                        ctx.body = JSON.stringify(update_return.rows[0]);
                        // ctx.body = JSON.stringify(authentication_entries[0]);
                    } else {
                        ctx.status = 404;
                    }
                } else if (authentication_entries.length == 0) {
                    // log user in by creating new  record
                    if (customer.password === login_req.password && customer.email === login_req.email) {
                        // log user in
                        const json_web_token = jwt.sign(customer, secret);
                        const update_return = await client.query("INSERT INTO authentication(customer_id, jwt, logged_in) " +
                            "VALUES($1::int, $2::text, NOW()) RETURNING *",
                            [customer.id, json_web_token]);
                        console.log(`response from INSERT query  ${JSON.stringify(update_return, null, 4)}`);
                        ctx.body = JSON.stringify(update_return.rows[0]);
                    } else {
                        ctx.status = 404;
                    }
                } else {
                    ctx.body = `more than 1 entry found for customer ${customer.id}-> that's not good`;
                    ctx.status = 500;
                }
            }
        } finally {
            client.release()
        }
    }
    await next;
});

router.post('/api/v1/authentication/logout', async (ctx: Koa.Context, next: Koa.Next) => {
    console.log(`ctx.request.body: ${JSON.stringify(ctx.request.body, null, 4)}`);
    const logout_req = ctx.request.body as LogOutRequest;
    console.log(`got an logout_req: ${JSON.stringify(logout_req, null, 4)}`);
    if (logout_req === undefined) {
        ctx.status = 404;
    } else {
        const client = await db.pool.connect();
        try {
            console.log(`trying to find an auth_entry for customer_id: ${JSON.stringify(logout_req.customerId, null, 4)}`);
            const auth_entries = await client.query('SELECT * FROM authentication WHERE customer_id = $1::int AND NOT jwt IS NULL ', [logout_req.customerId]);
            if (auth_entries === null || auth_entries.rows.length != 1) {
                ctx.status = 404
            } else {
                const update_return = await client.query('UPDATE  authentication  SET jwt =  NULL,   logged_out = NOW()', []);
                console.log(`logout: update statement return : ${JSON.stringify(update_return, null, 4)}`);
                // TDOO this is the old one, we want the actual one, where the loggedout timestamp is set and the JWT == NULL
                ctx.body = JSON.stringify(auth_entries[0]);
            }
        } finally {
            client.release()
        }
    }
    await next;
});


app.listen(port, () => {
    console.log(`started. listening on port ${port}`);
});