#!/bin/bash

JSON=java8_search_search_index.json

curl localhost:3030/api/backend/webflux_search_search_index | jq >service.json
curl localhost:3030/api/backend/apiclientprefix/webflux_search_search_index >apiclientprefix
curl localhost:3030/api/backend/apiclientpackage/webflux_search_search_index >apiclientpackage
curl localhost:3030/api/backend/openapiclient/webflux_search_search_index | jq > $JSON

#!/bin/bash

REPO_PATH=${PWD}

echo "REPO_PATH   ${REPO_PATH}"

OPENAPI_CLI=${REPO_PATH}/openapi-cli.jar

if [[ ! -f $OPENAPI_CLI ]]; then
  wget https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/6.3.0/openapi-generator-cli-6.3.0.jar -O $OPENAPI_CLI
fi

OUTPUT_PATH=${REPO_PATH}/../generated-clients/searchindexservice

echo "================================================================================"
echo "validating json"
echo "================================================================================"

java -jar $OPENAPI_CLI validate -i $JSON

echo "================================================================================"
echo "building stuff"
echo "================================================================================"
java -jar $OPENAPI_CLI generate \
  -i $JSON \
  -g java \
  --library native \
  -o $OUTPUT_PATH \
  --additional-properties=invokerPackage=at.bumzack.searchindexservice,apiPackage=at.bumzack.searchindexservice.api,modelPackage=at.bumzack.searchindexservice.model,useSpringBoot3=true,dateLibrary=java8-localdatetime,artifactId=java8_search_search_index_client,artifactVersion=0.0.1-SNAPSHOT,groupId=foryouandyourfakewebshop

rm -rf  ../src/main/java/at/bumzack/searchindexservice


mv ../generated-clients/searchindexservice/src/main/java/at/bumzack/searchindexservice    ../src/main/java/at/bumzack

