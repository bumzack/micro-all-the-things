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
 



