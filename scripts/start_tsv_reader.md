# Curl commands to trigger import

## MOVIE

```
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIE",  "start": 1,  "end": 10000, "pageSize": 5 }'
```

```
curl -X POST   http://localhost:8100/api/movie    -H  "Content-Type: application/json" -d   '{"entries":["tt0000001","short","Carmencita","Carmencita","0","1894","\\N","1","Documentary,Short"],"original":"tt0000001\tshort\tCarmencita\tCarmencita\t0\t1894\t\\N\t1\tDocumentary,Short"}'
```

```
curl -X POST   http://localhost:8984/solr/movie/update/json?commitWithin=100&overwrite=true&wt=json   -H  "Content-Type: application/json" -d  '{"id" : "tt0000001", "tconst" : "tt0000001", "titleType" : "titledffsfs typ", "primaryTitle" : "short", "originalTitle" : "Carmencita", "isAdult": false, "startYear" : "0", "endYear" : "1894", "runtimeMinutes":"34"}'
```

## MOVIEAKAS

```
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIEAKA",  "start": 1,  "end": 10000, "pageSize": 35403199}'
```

```
curl -X POST   http://localhost:8101/api/movieaka    -H  "Content-Type: application/json" -d   '{"entries":["tt0000001","short","Carmencita","Carmencita","0","1894","\\N","1","Documentary,Short"],"original":"tt0000001\tshort\tCarmencita\tCarmencita\t0\t1894\t\\N\t1\tDocumentary,Short"}'
```

```
curl -X POST   http://localhost:8984/solr/movieaka/update/json?commitWithin=100&overwrite=true&wt=json   -H  "Content-Type: application/json" -d '[{"id" : "tt0000001", "titleId" : "tt0000001" , "ordering":"5", "title" : "Карменсита", "region" : "RU", "language" : null, "types":["imdbDisplay"], "attributes":null, "originalTitle":false}]'
```


## CREW

```
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "CREW",  "start": 0,  "end": 10000, "pageSize": 2}'
```



## PERSON

```
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PERSON",  "start": 1,  "end": 10000, "pageSize": 2}'
```



## RATING

```
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "RATING",  "start": 1,  "end": 10000, "pageSize": 2}'
```

```
curl -X POST   http://localhost:8984/solr/rating/update/json?commitWithin=100&overwrite=true&wt=json   -H  "Content-Type: application/json" -d '[{"id" : "tt0000001", "tconst" : "tt0000001" , "averageRating":"5", "numVotes" : "23" }]'
```


## EPISODE

```
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "EPISODE",  "start": 1,  "end": 10000, "pageSize": 2}'
```


## PRINCIPAL

```
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PRINCIPAL",  "start": 1,  "end": 10000, "pageSize": 2}'
```



## Prod update cores    
```  
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "CREW",  "start": 1,  "end": 99999999, "pageSize": 50000 }'                     
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PRINCIPAL",  "start": 1,  "end": 99999999, "pageSize": 50000 }'       
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIEAKA",  "start": 1,  "end": 99999999, "pageSize": 50000 }'      
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "EPISODE",  "start": 1,  "end": 99999999, "pageSize": 50000 }'       
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PERSON",  "start": 1,  "end": 99999999, "pageSize": 50000 }' 
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIE",  "start": 1,  "end": 99999999, "pageSize": 50000 }'
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "RATING",  "start": 1,  "end": 99999999, "pageSize": 50000 }'       
```



## local test  update cores
```  

curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "CREW",  "start": 1,  "end": 20001, "pageSize": 500 }'                     
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PRINCIPAL",  "start": 1,  "end": 20001, "pageSize": 500 }'       
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIEAKA",  "start": 1,  "end": 20001, "pageSize": 500 }'      
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "EPISODE",  "start": 1,  "end": 20001, "pageSize": 500 }'       
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PERSON",  "start": 1,  "end": 20001, "pageSize": 500 }' 
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIE",  "start": 1,  "end": 20001, "pageSize": 500 }'
curl -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "RATING",  "start": 1,  "end": 20001, "pageSize": 500 }'       
```


```
CREATE USER 'bumzack'@'localhost' IDENTIFIED BY 'bumzack';

CREATE USER 'bumzack'@'localhost' IDENTIFIED BY 'bumzack';
CREATE USER 'bumzack'@'host' IDENTIFIED   BY 'bumzack';

GRANT PRIVILEGE ON *.* TO 'bumzack'@'host';
```


##  v4

```
 curl -vv -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PRINCIPAL",  "start": 1,  "end": 100, "pageSize": 10 }'
```

```
{ "lines" : [ {"entries" : ["tt0000026", "5", "nm0525910", "director", "N", "N" ], "original" :"tt0000026	5	nm0525910	director	\N	\N"}]}

```

```
curl -vv -X POST  http://localhost:8104/v2/api/principal   -H  "Content-Type: application/json" -d '{ "lines" : [ {"entries" : ["tt0000026", "5", "nm0525910", "grg", "N", "N" ], "original" :"tt0000026   5   nm0525910   grg    N   N"}]}'
```


##  v4


PRINCIPAL

```
 curl -vv -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "PRINCIPAL",  "start": 1,  "end": 100, "pageSize": 10 }'
```


MOVIEAKA

```
 curl -vv -X POST  http://localhost:8900/v2/api/tsv/read    -H  "Content-Type: application/json" -d '{  "tsvType": "MOVIEAKA",  "start": 1,  "end": 9999999, "pageSize": 50000 }'
```

Test tt4089182_3_nm6748260
tt4089182	3	nm6748260	actor	\N	["Legolas","Gimli","Radagast"]


tt0622100_10_nm0108613

tt0622100	10	nm0108613	writer	written by: Woody Allen's material	\N



```
{ "lines" : [ {"entries" : ["tt0000026", "5", "nm0525910", "director", "N", "N" ], "original" :"tt0000026	5	nm0525910	director	\N	\N"}]}

```

```
curl -vv -X POST  http://localhost:8104/v2/api/principal   -H  "Content-Type: application/json" -d '{ "lines" : [ {"entries" : ["tt0000026", "5", "nm0525910", "grg", "N", "N" ], "original" :"tt0000026   5   nm0525910   grg    N   N"}]}'
```
