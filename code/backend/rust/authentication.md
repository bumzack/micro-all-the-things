# Customer Stuff

## Is user authenticated ?

```
curl  -vv  http://localhost:18982/api/v1/authenticated/1  | jq             
``` 

## login

``` 
curl  -vv -X POST   http://localhost:18982/api/v1/authentication/login    -H 'Content-Type: application/json' -d '{ "email" : "bumzack@bumzack.at", "password" : "123" }' | jq
``` 

## logout

``` 
curl  -vv   -X POST   http://localhost:18982/api/v1/authentication/logout    -H 'Content-Type: application/json' -d '{ "customer_id" : 1203   }' | jq
``` 



``` 
curl  -vv -X POST   http://localhost:58982/api/v1/authentication/login    -H 'Content-Type: application/json' -d '{ "email" : "bumzack@bumzack.at", "password" : "123" }' | jq

curl  -vv   -X POST   http://localhost:58982/api/v1/authentication/logout    -H 'Content-Type: application/json' -d '{ "customerId" : 1203   }' | jq

curl  -vv  http://localhost:58982/api/v1/authenticated/1203  | jq             

curl  -vv -X POST   http://localhost:28982/api/v1/authentication/login    -H 'Content-Type: application/json' -d '{ "email" : "bumzack@bumzack.at", "password" : "123" }' | jq

curl  -vv -X POST   http://localhost:28982/api/v1/authentication/logout    -H 'Content-Type: application/json' -d '{ "customerId" : 1   }' | jq

curl  -vv  http://localhost:28982/api/v1/authenticated/1 | jq             
``` 

58982