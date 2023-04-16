# Search Stuff

## Solr

http://solr01.bumzack.at/solr/movie/select?fl=originalTitle%2CtitleType&indent=true&q.op=OR&q=originalTitle%3ATerminator%20AND%20titleType%3Amovie&rows=100&useParams=

## meilisearch

terminator movie Action
tt0088247

## API

https://askubuntu.com/questions/162229/how-do-i-increase-the-open-files-limit-for-a-non-root-user

in /etc/security eintragen und pam dingsdi auch

## get tasks

```
 curl  -vv   http://meilisearch01.bumzack.at/tasks    -H 'Authorization: Bearer 1234567890123456'    | jq
```

```
 curl  -vv   http://meilisearch01.bumzack.at/tasks/1131251    -H 'Authorization: Bearer 1234567890123456'    | jq
```

## read

```
 curl  -vv   http://meilisearch01.bumzack.at/indexes/movie/settings/filterable-attributes    -H 'Authorization: Bearer 1234567890123456'   | jq
```

```
  curl  -vv   http://meilisearch01.bumzack.at/indexes/person/settings/filterable-attributes    -H 'Authorization: Bearer 1234567890123456' | jq
```

## read settings

```
  curl  -vv   http://meilisearch01.bumzack.at/indexes/movie/settings/    -H 'Authorization: Bearer 1234567890123456' | jq
```

## set filterable attributes

```

  curl  -X PUT -d '[ "tconst" ]'                    http://meilisearch01.bumzack.at/indexes/movie/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
  curl  -X PUT -d '[ "titleId" ]'                   http://meilisearch01.bumzack.at/indexes/movieaka/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
  curl  -X PUT -d '[ "nconst" ]'                    http://meilisearch01.bumzack.at/indexes/person/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
  curl  -X PUT -d '[ "parentTconst", "tconst" ]'    http://meilisearch01.bumzack.at/indexes/episode/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
  curl  -X PUT -d '[  "tconst"  ]'                  http://meilisearch01.bumzack.at/indexes/crew/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
  curl  -X PUT -d '[ "tconst", "nconst" ]'          http://meilisearch01.bumzack.at/indexes/principal/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
  curl  -X PUT -d '[ "tconst"]'                     http://meilisearch01.bumzack.at/indexes/rating/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
  curl  -X PUT -d '[ "titles", "actors", "directors", "writers", "runtime_minutes", "adult", "genres", "characters", "title_type", "year"]'                     http://meilisearch01.bumzack.at/indexes/searchindex/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
```

## set sortable attributes

```
curl  -X PUT -d '[ "tconst"]'                    http://meilisearch01.bumzack.at/indexes/movie/settings/sortable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
```

curl -X PUT
-d '[ "tconst"]'                    http://meilisearch01.bumzack.at/indexes/searchindex/settings/sortable-attributes
-H 'Content-Type: application/json' -H 'Authorization: Bearer 1234567890123456' | jq

### movie all

```
 curl  -vv   http://localhost:18200/api/solr/movie/Terminator        | jq
```

```
 curl  -vv   http://localhost:18200/api/meili/movie/Terminator        | jq
```

```
curl  -X POST   http://localhost:18200/api/solr/movie    -H 'Content-Type: application/json'  -d '{ "q" : "*", "offset" : 0, "limit" : 3, "page" : 0 , "sort" : [ "tconst:asc" ] }'  | jq
```

```
curl  -X POST   http://localhost:18200/api/meili/movie    -H 'Content-Type: application/json'  -d '{ "q" : "*", "offset" : 0, "limit" : 2, "page" : 0 , "sort" : [ "tconst:asc" ] }'  | jq
```

### person by nconst POST a list

```
curl  -vv -X POST   http://localhost:18203/api/solr/person/filter      -d '{"nconsts":["nm0374658", "nm1588970", "nm0005690"]}'    -H 'Content-Type: application/json'   |  jq  
```

```
curl  -vv -X POST   http://localhost:18203/api/meili/person/filter      -d '{"nconsts":["nm0374658", "nm1588970", "nm0005690"]}'    -H 'Content-Type: application/json'   |  jq  
```

### Principal by Person Name (nconst)

```
 curl  -vv   http://localhost:18204/api/solr/principal/filter/name/nm1122026      |  jq  
```

```
 curl  -vv   http://localhost:18204/api/meili/principal/name/nm1122026      |  jq  
```

### Principal by Movie ID (tconst)

```
 curl  -vv   http://localhost:18204/api/solr/principal/filter/title/tt0666268         |  jq    
```

```
 curl  -vv   http://localhost:18204/api/meili/principal/filter/title/tt0666268         |  jq    
```

### Rating by Movie (tconst)

```
 curl  -vv   http://localhost:18202/api/solr/rating/filter/tt0666268         |  jq    
```

### Crew for Movie (tconst)

```
 curl  -vv   http://localhost:18205/api/solr/crew/filter/tt0666268         |  jq    
 
 curl  -vv   http://localhost:18205/api/meili/crew/filter/tt0666268         |  jq
```

### movieAka  for Movie (tconst)

```
 curl  -vv   http://localhost:18201/api/solr/movieaka/filter/tt0666268         |  jq    
 
 curl  -vv   http://localhost:18201/api/meili/movieaka/filter/tt0666268         |  jq
```

## Build Index

```
 curl  -vv   http://localhost:18300/api/v3/solr/searchindex/build         |  jq    
```

## /:engine/:start/:pagesize/:tasks

```
curl http://localhost:18300/api/v3/solr/searchindex/build/0/50000/6         |  jq    
```

## Movie max Hits

```
curl -X PATCH 'http://meilisearch01.bumzack.at/indexes/movie/settings/pagination' -H 'Content-Type: application/json' --data-binary '{ "maxTotalHits": 3000 }' -H 'Authorization: Bearer 1234567890123456' | jq

curl -X POST   http://localhost:18200/api/movie    -H 'Content-Type: application/json' -d '{ "q" : "*", "offset" : 351000, "limit" : 500, "page" : 0 , "sort" : [ "tconst:asc" ] }' | jq

curl   'http://meilisearch01.bumzack.at/indexes/movie/settings'     -H 'Authorization: Bearer 1234567890123456' | jq
curl   'http://meilisearch01.bumzack.at/indexes/person/settings'     -H 'Authorization: Bearer 1234567890123456' | jq
curl   'http://meilisearch01.bumzack.at/indexes/principal/settings'     -H 'Authorization: Bearer 1234567890123456' | jq

curl -X PATCH 'http://meilisearch01.bumzack.at/indexes/movie/settings/pagination' -H 'Content-Type: application/json'  --data-binary '{ "maxTotalHits": 100000000 }' -H 'Authorization: Bearer 1234567890123456' | jq
```

## Read documents sorted & paginated

```
curl   'http://meilisearch01.bumzack.at/indexes/movie/documents?limit=3&offset=1000000'     -H 'Authorization: Bearer 1234567890123456' | jq
```

## Search index

### Meili

```
curl  -vvvv    -X POST   http://localhost:18320/api/v1/meili/searchindex/search    -H 'Content-Type: application/json'  -d '{ "q" : "Terminator", "offset" : 0, "limit" : 2  }'  | jq
```

### Solr

```
curl  -vvvv    -X POST   http://localhost:18320/api/v1/solr/searchindex/search    -H 'Content-Type: application/json'  -d '{ "q" : "Terminator", "offset" : 0, "limit" : 2  }'  | jq
```

```
curl    -vvvv    -X POST -d '{"facets":["genres","actors","directors"],"hitsPerPage":2,"limit":2,"offset":0,"q":"Terminator","sort":null}'          http://meilisearch01.bumzack.at/indexes/searchindex/search     -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
```

## User login and search

```
curl  -vv -X POST   http://localhost:18982/api/v1/authentication/login    -H 'Content-Type: application/json' -d '{ "email" : "bumzack@bumzack.at", "password" : "123" }' | jq
```

### search SolR

```
curl  -vv -X POST   http://localhost:18600/api/v1/solr/article    -H 'Content-Type: application/json' -d '{ "q" : "Brianna", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1203, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9" }   }' | jq
```

### search meili

```
curl  -vv -X POST   http://localhost:18600/api/v1/meili/article    -H 'Content-Type: application/json' -d '{ "q" : "Brianna", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1203, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9 " }   }' | jq
```

### search SolR

```
curl  -vv -X POST   http://localhost:18600/api/v1/solr/article    -H 'Content-Type: application/json' -d '{ "q" : "Brianna", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1203, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9" }   }' | jq
```

### search meili

```
curl  -vv -X POST   http://localhost:18600/api/v1/meili/article    -H 'Content-Type: application/json' -d '{ "q" : "Brianna", "offset" : 0, "limit": 50, "customer" : {  "customer_id": 1203, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9 " }   }' | jq
```





