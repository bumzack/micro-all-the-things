# Customer Stuff

## Insert dummy data

```
curl http://localhost:18980/api/v1/customer/insertdummydata/0/1000/13000000            
``` 

## insert customer

``` 
curl -X POST   http://localhost:18980/api/v1/customer    -H 'Content-Type: application/json' -d '{ "first_name" : "bum", "last_name" : "zack", "email" : "bumzack@bumzack.at", "password" : "123"  }' | jq
``` 

## read customer by email

``` 
curl    http://localhost:18980/api/v1/customer/bumzack@bumzack.at   | jq
``` 

## read customers paginated

``` 
curl    http://localhost:18980/api/v1/customer/paginated/10/10   | jq
``` 

## read price for movie

``` 
curl  -vv  http://localhost:18800/api/v1/price/tt0000001   | jq
``` 

## insert dummy data prices for all movies

``` 
curl  -vv  http://localhost:18800/api/v1/price/insertdummydata/0/200/1100000
``` 

## Customer Price

### get discount for customer

``` 
curl  -vv  http://localhost:18981/api/v1/customerprice/1203/1990   | jq
``` 

### add  discount for customer

``` 
curl  -vv  -X POST  http://localhost:18981/api/v1/customerprice/   -H 'Content-Type: application/json' -d '{ "customer_id" : 1203, "start_year" : 1990, "end_year" : 1999, "discount" : 12.3  }' | jq
``` 

### add  dummy data for customer prices

``` 
curl  -vv    http://localhost:18981/api/v1/customerprice/insertdummydata/0/10/100   
``` 
    




