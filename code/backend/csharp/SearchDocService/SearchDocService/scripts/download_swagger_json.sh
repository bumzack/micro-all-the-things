#!/bin/bash

JSON=webflux_authentication_service.json
curl localhost:3030/api/backend/openapiclient/webflux_authentication_service | jq >$JSON

JSON=webflux_search_search_index.json
curl localhost:3030/api/backend/openapiclient/webflux_search_search_index | jq >$JSON

JSON=webflux_customerprice_service.json
curl localhost:3030/api/backend/openapiclient/webflux_customerprice_service | jq >$JSON

JSON=webflux_customer_service.json
curl localhost:3030/api/backend/openapiclient/webflux_customer_service | jq >$JSON

JSON=webflux_price_service.json
curl localhost:3030/api/backend/openapiclient/webflux_price_service | jq >$JSON

