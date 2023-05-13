import Router from 'koa-router';
import koaBody from 'koa-body';
import Koa from 'koa';
import logger from 'koa-logger';


import * as db from './db';
import {Customer} from "./generated-clients";
import * as dotenv from "dotenv";

const router = new Router();
const app = new Koa();

app.use(logger())
app.use(koaBody());
app.use(router.routes());
app.use(router.allowedMethods());

dotenv.config()

const port = process.env.PORT || 5000


router.get('/api/v1/customer/:email', async (ctx: Koa.Context, next: Koa.Next) => {
    const client = await db.pool.connect();
    const email = ctx.params.email;
    console.log(`got an email?:   ${email}`);
    try {
        const result = await client.query('SELECT * FROM customer WHERE email = $1::text', [email]);
        const customers = result.rows as Array<Customer>;
        console.log(`got customers: ${JSON.stringify(customers, null, 4)}`);
        if (customers.length == 1) {
            ctx.body = JSON.stringify(customers[0]);
        } else {
            ctx.status = 404
        }
    } finally {
        client.release()
    }
    await next;
});

app.listen(port, () => {
    console.log(`started. listening on port ${port}`);
});