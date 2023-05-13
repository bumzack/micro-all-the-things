import Router from 'koa-router';
import koaBody from 'koa-body';
import Koa from 'koa';
import logger from 'koa-logger';
import * as db from './db';
import * as dotenv from "dotenv";
import {PriceEntry, SearchPricesRequest} from "./generated-clients";


const router = new Router();
const app = new Koa();

app.use(logger())
app.use(koaBody());
app.use(router.routes());
app.use(router.allowedMethods());


dotenv.config()

const port = process.env.PORT || 5000


const secret = process.env.JWTSECRET || "fallbacksecret"

router.get("/api/v1/price/:tconst", async (ctx: Koa.Context, next: Koa.Next) => {
    const client = await db.pool.connect();
    //  const loggedin = ctx.request.body as LoggedinRequest;
    const tconst = ctx.params.tconst as string;
    console.log(`tconst ${tconst} `);
    try {
        const result = await client.query('SELECT * FROM price WHERE movie_tconst = $1::text ', [tconst]);
        const prices = result.rows as Array<PriceEntry>;
        console.log(`got price entries: ${JSON.stringify(prices, null, 4)}`);
        if (prices.length == 1) {
            ctx.body = JSON.stringify(prices[0]);
        } else {
            ctx.status = 404
        }
    } finally {
        client.release()
    }
    await next;
});

router.post('/api/v2/prices', async (ctx: Koa.Context, next: Koa.Next) => {
    console.log(`ctx.request.body: ${JSON.stringify(ctx.request.body, null, 4)}`);
    const search_prices_request = ctx.request.body as SearchPricesRequest;
    console.log(`got a search_prices_request: ${JSON.stringify(search_prices_request, null, 4)}`);
    if (search_prices_request === undefined) {
        ctx.status = 404;
    } else {
        const client = await db.pool.connect();
        try {
            const ids: Set<string> = new Set<string>();
            search_prices_request.movieTconst?.forEach(tconst => {
                ids.add(tconst);
            })

            const ids_list: Array<string> = [];
            ids.forEach(i => ids_list.push(i));

            const update_return = await client.query("SELECT * FROM price WHERE movie_tconst = ANY ($1) ",
                [ids_list]);
            console.log(`response from UPDATE query  ${JSON.stringify(update_return, null, 4)}`);
            const prices = update_return.rows as Array<PriceEntry>;
            ctx.body = JSON.stringify(prices);
        } finally {
            client.release()
        }
    }
    await next;
});

app.listen(port, () => {
    console.log(`started. listening on port ${port}`);
});
