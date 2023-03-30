# Testing the microthingi

```
curl  -vvvv  -X POST -H 'Content-Type: application/json' 'http://localhost:18104/api/principal' --data-binary ' { "original": "tt0000001\t1\tnm1588970\tself\t\\N\t[\"Self\"]", "entries" : ["tt0000001", "1" , "nm1588970" , "self" ,"\\N",  "[\"Self\"]" ]}'
```

  
```

    curl -X POST  http://localhost:18900/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIEAKA",  "start": 0,  "end": 10000, "pageSize": 100}'      

```


```

curl  -vvvv  -X POST -H 'Authorization: Bearer 1234567890123456'  -H 'Content-Type: application/json' 'http://meilisearch01.bumzack.at/indexes/principal/documents?primaryKey=id' --data-binary '{"category":"director","characters":["\\N"],"id":"tt0000001_2_nm0005690","nconst":"nm0005690","ordering":2,"tconst":"tt0000001"}'  | jq
```


curl  -X GET 'http://meilisearch01.bumzack.at/tasks/0'      -H 'Authorization: Bearer 1234567890123456' 
curl  -X GET 'http://meilisearch01.bumzack.at/tasks/1'    -H 'Authorization: Bearer 1234567890123456' 
curl  -X GET 'http://meilisearch01.bumzack.at/tasks/8'    -H 'Authorization: Bearer 1234567890123456' | jq 

