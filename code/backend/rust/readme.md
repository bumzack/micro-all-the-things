# Testing the microthingi

```
curl  -vvvv  -X POST -H 'Content-Type: application/json' 'http://localhost:18104/api/principal' --data-binary ' { "original": "tt0000001\t1\tnm1588970\tself\t\\N\t[\"Self\"]", "entries" : ["tt0000001", "1" , "nm1588970" , "self" ,"\\N",  "[\"Self\"]" ]}'
```

  
```

curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIEAKA",  "start": 0,  "end": 2, "pageSize": 3}'      

```


```

curl  -vvvv  -X POST -H 'Authorization: Bearer 1234567890123456'  -H 'Content-Type: application/json' 'http://meilisearch01.bumzack.at/indexes/principal/documents?primaryKey=id' --data-binary '{"category":"director","characters":["\\N"],"id":"tt0000001_2_nm0005690","nconst":"nm0005690","ordering":2,"tconst":"tt0000001"}'  | jq
```


curl  -X GET 'http://meilisearch01.bumzack.at/tasks/0'      -H 'Authorization: Bearer 1234567890123456' 
curl  -X GET 'http://meilisearch01.bumzack.at/tasks/1'    -H 'Authorization: Bearer 1234567890123456' 
curl  -X GET 'http://meilisearch01.bumzack.at/tasks/8'    -H 'Authorization: Bearer 1234567890123456' | jq 


```

curl  -vvvv  -X POST -H 'Authorization: Bearer 1234567890123456'  -H 'Content-Type: application/json' 'http://meilisearch01.bumzack.at/indexes/principal/documents?primaryKey=id' --data-binary '{"category":"director","characters":["\\N"],"id":"tt0000001_2_nm0005690","nconst":"nm0005690","ordering":2,"tconst":"tt0000001"}'  | jq
```


## TSV REader POST request

``` 
 curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "CREW",  "start": 1,  "end": 9928301, "pageSize": 50000}'          && 
 curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "EPISODE",  "start": 1,  "end": 7579300, "pageSize": 50000}'           && 
 curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIE",  "start": 1,  "end": 9928301, "pageSize": 40000}'         && 
 curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIEAKA",  "start": 1,  "end": 35461900, "pageSize": 50000}'         && 
 curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PERSON",  "start": 1,  "end": 12508003, "pageSize": 50000}'           && 
 curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "RATING",  "start": 1,  "end": 1494595, "pageSize": 50000}'            && 
 curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PRINCIPAL",  "start": 1,  "end": 60000000, "pageSize": 50000}'

 ```

curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "CREW",  "start": 1,  "end": 2001, "pageSize": 500}'    && 
curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "EPISODE",  "start": 1,  "end": 2001, "pageSize": 500}'    && 
curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIE",  "start": 1,  "end": 2001, "pageSize": 500}'    && 
curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIEAKA",  "start": 1,  "end": 2001, "pageSize": 500}'    && 
curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PERSON",  "start": 1,  "end": 2001, "pageSize": 500}'    && 
curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "RATING",  "start": 1,  "end": 2001, "pageSize": 500}'    && 
curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PRINCIPAL",  "start": 1,  "end": 2001, "pageSize": 500}'



## List indices


curl  -X GET   -H 'Authorization: Bearer 1234567890123456'  'http://meilisearch01.bumzack.at/indexes'


## Delete index

curl  -X DELETE   -H 'Authorization: Bearer 1234567890123456'  'http://meilisearch01.bumzack.at/indexes/principal'
curl  -X DELETE   -H 'Authorization: Bearer 1234567890123456'  'http://meilisearch01.bumzack.at/indexes/movie'
curl  -X DELETE   -H 'Authorization: Bearer 1234567890123456'  'http://meilisearch01.bumzack.at/indexes/movieaka'
curl  -X DELETE   -H 'Authorization: Bearer 1234567890123456'  'http://meilisearch01.bumzack.at/indexes/person'
curl  -X DELETE   -H 'Authorization: Bearer 1234567890123456'  'http://meilisearch01.bumzack.at/indexes/rating'
curl  -X DELETE   -H 'Authorization: Bearer 1234567890123456'  'http://meilisearch01.bumzack.at/indexes/episode'
curl  -X DELETE   -H 'Authorization: Bearer 1234567890123456'  'http://meilisearch01.bumzack.at/indexes/crew'


 

 