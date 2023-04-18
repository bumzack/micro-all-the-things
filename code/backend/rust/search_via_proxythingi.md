# Search Stuff via ProxyThingi

## Login via ProxyThingi

```
curl  -vv -X POST   http://proxy.bumzack.at/rust/login    -H 'Content-Type: application/json' -d '{ "email" : "bumzack@bumzack.at", "password" : "123" }' | jq
```

### search SolR

```
curl  -vv -X POST   http://proxy.bumzack.at/rust/solr/search    -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9" }   }' | jq
```

### search meili

```
curl  -vv -X POST   http://proxy.bumzack.at/rust/meili/search   -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9 " }   }' | jq
```

## LOCAL

```
curl  -vv -X POST   http://localhost:18982/api/v1/authentication/login    -H 'Content-Type: application/json' -d '{ "email" : "bumzack@bumzack.at", "password" : "123" }' | jq
```

### search SolR

```
curl  -vv -X POST   http://localhost:18600/api/v1/solr/article    -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9" }   }' | jq
```

### search meili

```
curl  -vv -X POST   http://localhost:18600/api/v1/meili/article    -H 'Content-Type: application/json' -d '{ "q" : "Terminator", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9 " }   }' | jq
```




