import * as Koa from 'koa'
import  routes from './routes'

const app = new Koa();


app.use(routes.routes())

app.listen(3000);
