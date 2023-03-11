#!/bin/zsh


nodemon productsapp/serve_product_app.js       & 
nodemon cartapp/serve_cart_app.js              & 
nodemon searchapp/serve_search_app.js             & 
nodemon searchresultapp/serve_search_result_app.js      &
nodemon mainapp/serve_main.js