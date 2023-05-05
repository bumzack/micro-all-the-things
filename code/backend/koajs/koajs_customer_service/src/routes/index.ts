import * as Router from 'koa-router'

const myroutes = new Router();

myroutes.get("/bum" ,async (ctx, next) => {
    ctx.body = 'Hello Bum';
    await next;
})

export default myroutes