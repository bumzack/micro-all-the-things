#!/bin/bash

JSON=webflux_price_service.json

curl localhost:3030/api/backend/webflux_price_service | jq > service.json
curl localhost:3030/api/backend/apiclientprefix/webflux_price_service > apiclientprefix
curl localhost:3030/api/backend/apiclientpackage/webflux_price_service > apiclientpackage
curl localhost:3030/api/backend/openapiclient/webflux_price_service | jq > $JSON

#!/bin/bash

REPO_PATH=${PWD}

echo "REPO_PATH   ${REPO_PATH}"

OPENAPI_CLI=${REPO_PATH}/openapi-cli.jar

if [[ ! -f $OPENAPI_CLI ]]; then
  wget https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/6.3.0/openapi-generator-cli-6.3.0.jar -O $OPENAPI_CLI
fi

OUTPUT_PATH=${REPO_PATH}/../generated-clients/priceservice

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
  --library webclient \
  -o $OUTPUT_PATH \
  --additional-properties=invokerPackage=at.bumzack.priceservice,apiPackage=at.bumzack.priceservice.api,modelPackage=at.bumzack.priceservice.model,useSpringBoot3=true,dateLibrary=java8-localdatetime,artifactId=webflux_price_service_client,artifactVersion=0.0.1-SNAPSHOT,groupId=foryouandyourfakewebshop

rm -rf  ../src/main/java/at/bumzack/priceservice

mv ../generated-clients/priceservice/src/main/java/at/bumzack/priceservice ../src/main/java/at/bumzack
