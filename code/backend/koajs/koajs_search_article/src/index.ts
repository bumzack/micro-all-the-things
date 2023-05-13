import Router from 'koa-router';
import koaBody from 'koa-body';
import Koa from 'koa';
import logger from 'koa-logger';
import * as dotenv from "dotenv";
import {ArticleSearchResult, SearchArticleRequest} from "./generated-clients/searcharticleservice";
import {is_authenticated} from "./services_auth";
import {search_movies} from "./services_index";
import {find_prices} from "./services_prices";
import {PriceEntry} from "./generated-clients/priceservice";
import {find_customer_prices} from "./services_customerprices";
import {CustomerPriceEntry} from "./generated-clients/customerpriceservice";

const router = new Router();
const app = new Koa();

app.use(logger())
app.use(koaBody());
app.use(router.routes());
app.use(router.allowedMethods());


dotenv.config()

const port = process.env.PORT || 5000

router.post('/api/v1/solr/article', async (ctx: Koa.Context, next: Koa.Next) => {
    const req = ctx.request.body as SearchArticleRequest;
    console.log(`req ${JSON.stringify(req, null, 4)}`);
    const res: Array<ArticleSearchResult> = [];

    if (req.customer !== null && req.customer?.customerId != null) {
        console.log(`got a customer and an id   ${req.customer?.customerId}`)
        const auth = await is_authenticated(req.customer.customerId);
        console.log(`auth    ${JSON.stringify(auth)}`)
        const logged_in = auth.jwt != null;
        console.log(`logged_in    ${logged_in}`)
        let customer_prices: Array<CustomerPriceEntry> = [];
        if (logged_in) {
            customer_prices = await find_customer_prices(req.customer.customerId);
        }
        console.log(`customer_prices    ${JSON.stringify(customer_prices)}`)

        let movies = await search_movies(req.q as string, req.offset as number, req.limit as number);
        if (movies.movies != undefined) {
            let tconsts: Array<string> = movies.movies.map(m => m.tconst as string);
            let prices_tmp = await find_prices(tconsts);
            console.log(`prices_tmp    ${JSON.stringify(prices_tmp)}`)
            let prices = new Map<string, PriceEntry>();
            prices_tmp.forEach(p => prices.set(p.movieTconst as string, p));

            movies.movies.forEach(m => {
                const p = prices.get(m.tconst as string) as PriceEntry;
                let cp = undefined;
                if (logged_in) {
                    const customer_price = customer_prices.find(p => {
                        return (p.startYear as number) <= (m.year as number) && (m.year as number) <= (p.endYear as number);
                    });
                    if (customer_price !== undefined) {
                        cp = (100.0 - (customer_price.discount as number)) * (p.amount as number) / 100;
                    }
                }

                const entry: ArticleSearchResult = {
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
