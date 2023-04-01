# Search Stuff

## Solr

http://solr01.bumzack.at/solr/movie/select?fl=originalTitle%2CtitleType&indent=true&q.op=OR&q=originalTitle%3ATerminator%20AND%20titleType%3Amovie&rows=100&useParams=


## meilisearch

terminator movie Action
tt0088247


## API

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

## set filterable attributes
```
  curl  -vv -X PUT -d '[ "tconst", "titleType" ]'  http://meilisearch01.bumzack.at/indexes/movie/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
```

```
  curl  -vv -X PUT -d '[ "titleId", "types" ]'  http://meilisearch01.bumzack.at/indexes/movieaka/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
```

```
  curl  -vv -X PUT -d '[ "nconst", "knownForTitles", "primaryProfession" ]'  http://meilisearch01.bumzack.at/indexes/person/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
```

```
  curl  -vv -X PUT -d '[ "parentTconst", "tconst" ]'  http://meilisearch01.bumzack.at/indexes/episode/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
```

```
  curl  -vv -X PUT -d '[ "directors", "tconst", "writers" ]'  http://meilisearch01.bumzack.at/indexes/crew/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
```

```
  curl  -vv -X PUT -d '[ "tconst", "nconst", "category", "characters"]'  http://meilisearch01.bumzack.at/indexes/principal/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
```

```
  curl  -vv -X PUT -d '[ "tconst"]'  http://meilisearch01.bumzack.at/indexes/rating/settings/filterable-attributes    -H 'Content-Type: application/json'     -H 'Authorization: Bearer 1234567890123456'    | jq
```

### movie all 
```
 curl  -vv   http://localhost:18200/api/movie/Terminator        
```


### person all 

```
 curl  -vv   http://localhost:18203/api/person/Schwarzeneger        
```

### person by nconst

bynconst
```
 curl  -vv   http://localhost:18203/api/personbynconst/nm0000216        
```
