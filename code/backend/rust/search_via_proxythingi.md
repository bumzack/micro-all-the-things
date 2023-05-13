# Search Stuff via ProxyThingi

## Login via ProxyThingi

```
curl  -vv -X POST   http://proxy.proxythingi.at/rust/login    -H 'Content-Type: application/json' -d '{ "email" : "bumzack@bumzack.at", "password" : "123" }' | jq
```

### search SolR

```
curl  -vv -X POST   http://proxy.proxythingi.at/rust/solr/search    -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJidW16YWNrQGJ1bXphY2suYXQiLCJleHAiOjE3MTI2NTQ0MTEsImlhdCI6MTY4NDAxNDQxMX0.3nd_Fe-QPgfqDdS7ibxuCEMvp43dSvUSNKe4frzlotg" }   }' | jq
```

### search meili

```
curl  -vv -X POST   http://proxy.proxythingi.at/rust/meili/search   -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJidW16YWNrQGJ1bXphY2suYXQiLCJleHAiOjE3MTI2NTQ0MTEsImlhdCI6MTY4NDAxNDQxMX0.3nd_Fe-QPgfqDdS7ibxuCEMvp43dSvUSNKe4frzlotg " }   }' | jq
```

## LOCAL

```
curl  -vv -X POST   http://localhost:18982/api/v1/authentication/login    -H 'Content-Type: application/json' -d '{ "email" : "bumzack@bumzack.at", "password" : "123" }' | jq
```

### search SolR

```
curl  -vv -X POST   http://localhost:18600/api/v1/solr/article    -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJidW16YWNrQGJ1bXphY2suYXQiLCJleHAiOjE3MTI2NTQ0MTEsImlhdCI6MTY4NDAxNDQxMX0.3nd_Fe-QPgfqDdS7ibxuCEMvp43dSvUSNKe4frzlotg" }   }' | jq
```

### search meili

```
curl  -vv -X POST   http://localhost:18600/api/v1/meili/article    -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJidW16YWNrQGJ1bXphY2suYXQiLCJleHAiOjE3MTI2NTQ0MTEsImlhdCI6MTY4NDAxNDQxMX0.3nd_Fe-QPgfqDdS7ibxuCEMvp43dSvUSNKe4frzlotg " }   }' | jq
```

## WebFlux Java Solr

```
curl -vv -X POST   http://localhost:8600/api/v1/solr/article    -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customerId": 1, "jwt" : "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJidW16YWNrQGJ1bXphY2suYXQiLCJleHAiOjE3MTI2NTQ0MTEsImlhdCI6MTY4NDAxNDQxMX0.3nd_Fe-QPgfqDdS7ibxuCEMvp43dSvUSNKe4frzlotg" } }' | jq
```

```
curl  -vv -X POST   http://proxy.proxythingi.at/java8/solr/search   -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customerId": 1, "jwt" : "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJidW16YWNrQGJ1bXphY2suYXQiLCJleHAiOjE3MTI2NTQ0MTEsImlhdCI6MTY4NDAxNDQxMX0.3nd_Fe-QPgfqDdS7ibxuCEMvp43dSvUSNKe4frzlotg " }   }' | jq
curl  -vv -X POST   http://proxy.proxythingi.at/rust/solr/search   -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customerId": 1, "jwt" : "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJidW16YWNrQGJ1bXphY2suYXQiLCJleHAiOjE3MTI2NTQ0MTEsImlhdCI6MTY4NDAxNDQxMX0.3nd_Fe-QPgfqDdS7ibxuCEMvp43dSvUSNKe4frzlotg " }   }' | jq
curl  -vv -X POST   http://proxy.proxythingi.at/rust/meili/search   -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customerId": 1, "jwt" : "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJidW16YWNrQGJ1bXphY2suYXQiLCJleHAiOjE3MTI2NTQ0MTEsImlhdCI6MTY4NDAxNDQxMX0.3nd_Fe-QPgfqDdS7ibxuCEMvp43dSvUSNKe4frzlotg " }   }' | jq
curl  -vv -X POST   http://proxy.proxythingi.at/webflux/solr/search   -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customerId": 1, "jwt" : "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJidW16YWNrQGJ1bXphY2suYXQiLCJleHAiOjE3MTI2NTQ0MTEsImlhdCI6MTY4NDAxNDQxMX0.3nd_Fe-QPgfqDdS7ibxuCEMvp43dSvUSNKe4frzlotg " }   }' | jq
curl  -vv -X POST   http://proxy.proxythingi.at/koa/solr/search   -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customerId": 1, "jwt" : "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJidW16YWNrQGJ1bXphY2suYXQiLCJleHAiOjE3MTI2NTQ0MTEsImlhdCI6MTY4NDAxNDQxMX0.3nd_Fe-QPgfqDdS7ibxuCEMvp43dSvUSNKe4frzlotg " }   }' | jq



curl  -vv -X POST   http://proxy.proxythingi.at/rust/login    -H 'Content-Type: application/json' -d '{ "email" : "bumzack@bumzack.at", "password" : "123" }' | jq

```
