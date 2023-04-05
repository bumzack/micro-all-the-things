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
  
```

## set sortable attributes

```
curl  -X PUT -d '[ "tconst"]'                    http://meilisearch01.bumzack.at/indexes/movie/settings/sortable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
```

### movie all

```
 curl  -vv   http://localhost:18200/api/movie/Terminator        
```

```
curl  -X POST   http://localhost:18200/api/movie    -H 'Content-Type: application/json'  -d '{ "q" : "*", "offset" : 500000, "limit" : 2, "page" : 0 , "sort" : [ "tconst:asc" ] }'  | jq
     
```

### person all attributes

```
 curl  -vv   http://localhost:18203/api/person/name/Schwarzeneger           |  jq  
```

### person by nconst POST a list

```
 curl  -vv -X POST   http://localhost:18203/api/person/nconst      -d '{"nconsts":["nm0374658","nm1588970","nm0005690"]}'    -H 'Content-Type: application/json'   |  jq  
```

```
 curl  -vv   http://localhost:18203/api/person/nconst/nm0000216        |  jq     
```

### Principal by Person Name (nconst)

```
 curl  -vv   http://localhost:18204/api/principal/name/nm1122026      |  jq  
```

### Principal by Movie ID (tconst)

```
 curl  -vv   http://localhost:18204/api/principal/title/tt0666268         |  jq    
```

### Rating by Movie (tconst)

```
 curl  -vv   http://localhost:18205/api/rating/tt0666268         |  jq    
```

### Crew for Movie (tconst)

```
 curl  -vv   http://localhost:18205/api/rating/tt0666268         |  jq    
```

## Build Index

```
 curl  -vv   http://localhost:18300/api/searchindex/build         |  jq    
```

## Movie max Hits

curl -X PATCH 'http://meilisearch01.bumzack.at/indexes/movie/settings/pagination' -H 'Content-Type: application/json'
--data-binary '{ "maxTotalHits": 3000 }' -H 'Authorization: Bearer 1234567890123456' | jq

curl -X POST   http://localhost:18200/api/movie    -H 'Content-Type: application/json' -d '{ "q" : "*", "offset" :
351000, "limit" : 500, "page" : 0 , "sort" : [ "tconst:asc" ] }' | jq

curl   'http://meilisearch01.bumzack.at/indexes/movie/settings'     -H 'Authorization: Bearer 1234567890123456' | jq
curl   'http://meilisearch01.bumzack.at/indexes/person/settings'     -H 'Authorization: Bearer 1234567890123456' | jq
curl   'http://meilisearch01.bumzack.at/indexes/principal/settings'     -H 'Authorization: Bearer 1234567890123456' | jq

curl -X PATCH 'http://meilisearch01.bumzack.at/indexes/movie/settings/pagination' -H 'Content-Type: application/json'
--data-binary '{ "maxTotalHits": 100000000 }' -H 'Authorization: Bearer 1234567890123456' | jq

## Read documents sorted & paginated

curl   'http://meilisearch01.bumzack.at/indexes/movie/documents?limit=3&offset=1000000'     -H 'Authorization: Bearer
1234567890123456' | jq
