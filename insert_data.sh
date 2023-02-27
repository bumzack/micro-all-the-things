#!/bin/zsh

time curl http://localhost:8090/solr/categories & 
time curl http://localhost:8091/solr/products/0/50000/5500000 & 
time curl http://localhost:8092/solr/images/0/10000/400000 & 




{\"code":null,"text":"BLECH","start":0,"pageSize":50}
 wi


 curl -X POST  http://localhost:8300/solr/search/article/text    -H 'Content-type:application/json' -d  -H  "Content-Type: application/json" -d "{  \"code\": null,  \"text\": \"BLECH\",  \"start\": 0, \"BLECH\",  \"pageSize\": 50 }" 


# curl -k -X POST $URL -H  "accept: */*" -H  "X-ATS-Cooperative-ID: $COOPID" -H  "X-ATS-User-ID: $UUID" -H  "authorization: Basic YXRzX2NsaWVudDpzZWNyZXQ=" -H  "Content-Type: application/json" -d "{  \"businessArea\": \"$BUSINESSAREA\",  \"code\": \"$ARTICLE_CODE\",  \"quantity\": 2}"    |  jq


 curl -X POST http://localhost:8300/solr/search/article/text  -H 'Content-type:application/json' -d  "{   \"code\": \"1\",   \"text\": \"B\",  \"start\": 0,  \"pageSize\": 50 }"


  { \"id\":\"8798524604466_70_66\",     \"code\":\"8798524604466_70_66\", \"mediaContainerQualifier\":\"XinetAsset_10074039\", \"url\":\"/mam/20/6f/P-W-8798524604466-0.jpg\", \"width\":\"70\", \"height\":\"66\", \"valid\":true}




  curl -X POST  http://localhost:8400/image/desktop/  -H 'Content-type:application/json' -d  "[   { \"id\":\"8798524604466_70_66\",     \"code\":\"8798524604466_70_66\", \"mediaContainerQualifier\":\"XinetAsset_10074039\", \"url\":\"/mam/20/6f/P-W-8798524604466-0.jpg\", \"width\":\"70\", \"height\":\"66\", \"valid\":true}  ]"
