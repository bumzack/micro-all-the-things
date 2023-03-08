#!/bin/zsh

time curl http://localhost:8090/pim/categories & 
time curl http://localhost:8091/pim/products/0/50000/5500000 & 
time curl http://localhost:8092/pim/images/0/10000/400000 & 
time curl http://localhost:8092/pim/prices/0/10000/400000 & 




# {\"code":null,"text":"BLECH","start":0,"pageSize":50}
#  wi


#  curl -X POST  http://localhost:8300/solr/search/article/text    -H 'Content-type:application/json' -d  -H  "Content-Type: application/json" -d "{  \"code\": null,  \"text\": \"BLECH\",  \"start\": 0, \"BLECH\",  \"pageSize\": 50 }" 


# # curl -k -X POST $URL -H  "accept: */*" -H  "X-ATS-Cooperative-ID: $COOPID" -H  "X-ATS-User-ID: $UUID" -H  "authorization: Basic YXRzX2NsaWVudDpzZWNyZXQ=" -H  "Content-Type: application/json" -d "{  \"businessArea\": \"$BUSINESSAREA\",  \"code\": \"$ARTICLE_CODE\",  \"quantity\": 2}"    |  jq


#  curl -X POST http://localhost:8300/solr/search/article/text  -H 'Content-type:application/json' -d  "{   \"code\": \"1\",   \"text\": \"B\",  \"start\": 0,  \"pageSize\": 2 }"


#   { \"id\":\"8798524604466_70_66\",     \"code\":\"8798524604466_70_66\", \"mediaContainerQualifier\":\"XinetAsset_10074039\", \"url\":\"/mam/20/6f/P-W-8798524604466-0.jpg\", \"width\":\"70\", \"height\":\"66\", \"valid\":true}



2023-02-27T21:04:22.902+01:00  INFO 72591 --- [ctor-http-nio-3] a.bumzack.imageselector.CustomWebFilter  : success: Body request [{"id":"8798524604466_70_66","code":"8798524604466_70_66","mediaContainerQualifier":"XinetAsset_10074039","url":"https://qstatic.rwa-test.at/mam/20/6f/P-W-8798524604466-0.jpg","width":"70","height":"66","channel":null,"valid":true,"mediaFormat":null,"mime":null},{"id":"8798524604466_270_253","code":"8798524604466_270_253","mediaContainerQualifier":"XinetAsset_10074039","url":"https://qstatic.rwa-test.at/mam/20/6f/P-W-8798524604466-1.jpg","width":"270","height":"253","channel":null,"valid":true,"mediaFormat":null,"mime":null},{"id":"8798524604466_470_440","code":"8798524604466_470_440","mediaContainerQualifier":"XinetAsset_10074039","url":"https://qstatic.rwa-test.at/mam/20/6f/P-W-8798524604466-2.jpg","width":"470","height":"440","channel":null,"valid":true,"mediaFormat":null,"mime":null},{"id":"8798524604466_1500_1406","code":"8798524604466_1500_1406","mediaContainerQualifier":"XinetAsset_10074039","url":"https://qstatic.rwa-test.at/mam/20/6f/P-W-8798524604466-3.jpg","width":"1500","height":"1406","channel":null,"valid":true,"mediaFormat":null,"mime":null}]



#   curl -X POST  http://localhost:8400/image/desktop/  -H 'Content-type:application/json' -d  "[   { \"id\":\"8798524604466_70_66\",     \"code\":\"8798524604466_70_66\", \"mediaContainerQualifier\":\"XinetAsset_10074039\", \"url\":\"/mam/20/6f/P-W-8798524604466-0.jpg\", \"width\":\"70\", \"height\":\"66\", \"valid\":true}  ]"
