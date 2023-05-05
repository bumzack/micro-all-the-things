#!/bin/bash

JSON=webflux_customer_service.json

curl localhost:3030/api/backend/webflux_customer_service | jq >service.json

curl localhost:3030/api/backend/apiclientprefix/webflux_customer_service >apiclientprefix
curl localhost:3030/api/backend/apiclientpackage/webflux_customer_service >apiclientpackage

curl localhost:3030/api/backend/openapiclient/webflux_customer_service | jq >  $JSON
 