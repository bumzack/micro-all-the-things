#!/bin/bash

JSON=webflux_authentication_service.json

curl localhost:3030/api/backend/webflux_authentication_service | jq > service.json
curl localhost:3030/api/backend/apiclientprefix/webflux_authentication_service > apiclientprefix
curl localhost:3030/api/backend/apiclientpackage/webflux_authentication_service > apiclientpackage
curl localhost:3030/api/backend/openapiclient/webflux_authentication_service | jq > $JSON

#!/bin/bash

REPO_PATH=${PWD}

echo "REPO_PATH   ${REPO_PATH}"

OPENAPI_CLI=${REPO_PATH}/openapi-cli.jar

if [[ ! -f $OPENAPI_CLI ]]; then
  wget https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/6.3.0/openapi-generator-cli-6.3.0.jar -O $OPENAPI_CLI
fi

OUTPUT_PATH=${REPO_PATH}/src/generated-clients/authenticationservice

echo "================================================================================"
echo "validating json"
echo "================================================================================"

java -jar $OPENAPI_CLI validate -i $JSON

echo "================================================================================"
echo "building stuff"
echo "================================================================================"
java -jar $OPENAPI_CLI generate \
  -i $JSON \
  -g typescript-fetch \
  -o $OUTPUT_PATH \
  -additional-properties=supportsES6=true,withNodeImports=true