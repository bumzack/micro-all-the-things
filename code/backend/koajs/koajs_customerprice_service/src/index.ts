import Router from 'koa-router';
import koaBody from 'koa-body';
import Koa from 'koa';
import logger from 'koa-logger';
import * as db from './db';
import * as dotenv from "dotenv";
import {CustomerPriceEntry} from "./generated-clients";

const router = new Router();
const app = new Koa();

app.use(logger())
app.use(koaBody());
app.use(router.routes());
app.use(router.allowedMethods());

dotenv.config()

const port = process.env.PORT || 5000

router.get("/api/v1/customerprices/:customerId", async (ctx: Koa.Context, next: Koa.Next) => {
    const client = await db.pool.connect();
    const customerId = ctx.params.customerId as string;
    console.log(`customerId ${customerId} `);
    try {
        const result = await client.query('SELECT * FROM customer_price WHERE customer_id = $1::int ', [customerId]);
        const customer_prices = result.rows as Array<CustomerPriceEntry>;
        console.log(`got customer_prices entries: ${JSON.stringify(customer_prices, null, 4)}`);
        ctx.body = JSON.stringify(customer_prices);
    } finally {
        client.release()
    }
    await next;
});

app.listen(port, () => {
    console.log(`started. listening on port ${port}`);
});