const Koa = require('koa')
const serve = require('koa-static')
const path = require('path')
const cors = require('@koa/cors');

const app = new Koa()
app.use(cors());
app.use(serve(path.join(__dirname, './')))

console.log("path " +  path.join(__dirname, './'))
app.listen(3013)
