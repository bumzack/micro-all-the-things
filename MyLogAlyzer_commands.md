# MyLogAlyzer commands

## Categorien von Hybris -> Solr
time curl http://localhost:8090/solr/categories


## Produkte von Hybris -> Solr
time curl http://localhost:8091/solr/products/0/1000/50000


## image assets von Hybris -> Solr
time curl http://localhost:8092/solr/image/0/1000/50000


## Suchen im Solr

### Category

Suche nach COde
time curl   -X POST http://localhost:8200/solr/search/category/code   -H  "Content-Type: application/json" -d "{  \"code\": \"Category_001959\",  \"text\": \"Hammer\",  \"start\": 0,  \"pageSize\": 100}"   | jq 

alle:  
time curl    http://localhost:8200/solr/search/category   | jq


Root categories

time curl    http://localhost:8200/solr/search/category/root   | jq

time curl    http://localhost:8200/solr/search/category/children/Category_000000   | jq


### Product

nach Code
time curl   -X POST http://localhost:8201/solr/search/product/code   -H  "Content-Type: application/json" -d "{  \"code\": \"152496\",   \"start\": 0,  \"pageSize\": 100}"   | jq 

time curl   -X POST http://localhost:8201/solr/search/product/text   -H  "Content-Type: application/json" -d "{   \"text\": \"MISCH\",  \"start\": 0,  \"pageSize\": 100}"   | jq 

time curl   -X POST http://localhost:8201/solr/search/product/text   -H  "Content-Type: application/json" -d "{   \"text\": \"MISCH\",  \"start\": 0,  \"pageSize\": 100}"   | jq

### Xinet

nach mediaContainerQualifier        XinetAsset_10011434

time curl   http://localhost:8202/solr/search/xinet/XinetAsset_10011434     | jq 


### Artikel = Produkt + Bilder

nach mediaContainerQualifier        XinetAsset_10011434

time curl   -X POST http://localhost:8201/solr/search/product/code   -H  "Content-Type: application/json" -d "{  \"code\": \"152496\",   \"start\": 0,  \"pageSize\": 100}"   | jq 


time curl  -X POST http://localhost:8300/solr/search/article/code     -H  "Content-Type: application/json" -d "{  \"code\": \"152496\",   \"start\": 0,  \"pageSize\": 100}"   | jq 
time curl  -X POST http://localhost:8300/solr/search/article/text    -H  "Content-Type: application/json" -d "{   \"text\": \"Blech\",  \"start\": 0,  \"pageSize\": 100}"   | jq 





## trigger Datenimport von PIM -> Solr

time curl http://localhost:8080/solr/findArticles/0/50000/100000000

time curl http://localhost:8080/solr/findArticles/0/5/12

// https://localhost:8002/rwaportalwebservices/public/products/readpaginated?start=0&pagesize=10


PIM direkt 
curl -X GET --header 'Accept: application/json' --header 'Authorization: Bearer 9c4ab3dc-ac1e-41bc-9e17-b37e9c69e44a' 'https://localhost:8002/rwaportalwebservices/public/products/readpaginated?start=0&pagesize=10'

## Suche nach Code in Solr


time curl -vvvvv  -X POST http://localhost:8082/solr/search/searchByArticleCode    -H  "Content-Type: application/json" -d "{  \"code\": \"$123\",  \"text\": \"Hammer\",  \"start\": 0,  \"pageSize\": 100}"   | jq 





time curl http://localhost:8090/solr/categories/0/5/100  
