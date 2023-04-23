# Customer Stuff

## Insert dummy data

```
curl http://localhost:18980/api/v1/customer/insertdummydata/0/1000/13000000            
``` 

## insert customer

``` 
curl -vvv -X POST   http://localhost:18980/api/v1/customer    -H 'Content-Type: application/json' -d '{ "first_name" : "23bum", "last_name" : "23zack", "email" : "bumzack23@bumzack23.at", "password" : "123"  }' | jq
``` 

## read customer by email

``` 
curl   -vvv   http://localhost:18980/api/v1/customer/bumzack@bumzack.at   | jq
``` 

## read customers paginated

``` 
curl   -vvv   http://localhost:18980/api/v1/customer/paginated/10/10   | jq
``` 

## read price for movie

``` 
curl  -vv  http://localhost:18800/api/v1/price/tt5370708   | jq
``` 

## insert dummy data prices for all movies

``` 
curl  -vv  http://localhost:18800/api/v1/price/insertdummydata/0/100000/1100000
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




tt12087758

