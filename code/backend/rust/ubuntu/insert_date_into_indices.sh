#!/bin/zsh

START_ALL=$(date +"%d-%m-%Y %T")

START_CREW=$(date +"%d-%m-%Y %T")
echo "started  CREW at  ${START_CREW}"
curl -X POST http://localhost:18900/tsv/read -H "Content-Type: application/json" -d '{  "tsvType": "CREW",  "start": 1,  "end": 10928301, "pageSize": 50000}'
END_CREW=$(date +"%d-%m-%Y %T")
echo "ended CREW  at  ${END_CREW}"

START_EPISODE=$(date +"%d-%m-%Y %T")
echo "started  EPISODE at  ${START_EPISODE}"
curl -X POST http://localhost:18900/tsv/read -H "Content-Type: application/json" -d '{  "tsvType": "EPISODE",  "start": 1,  "end": 9979300, "pageSize": 50000}'
END_EPISODE=$(date +"%d-%m-%Y %T")
echo ""
echo "ended EPISODE  at  ${END_EPISODE}"

START_MOVIE=$(date +"%d-%m-%Y %T")
echo "started  MOVIE at  ${START_MOVIE}"
curl -X POST http://localhost:18900/tsv/read -H "Content-Type: application/json" -d '{  "tsvType": "MOVIE",  "start": 1,  "end": 15000000, "pageSize": 40000}'
END_MOVIE=$(date +"%d-%m-%Y %T")
echo ""
echo "ended MOVIE  at  ${END_MOVIE}"

START_MOVIEAKA=$(date +"%d-%m-%Y %T")
echo "started  MOVIEAKA at  ${START_MOVIEAKA}"
curl -X POST http://localhost:18900/tsv/read -H "Content-Type: application/json" -d '{  "tsvType": "MOVIEAKA",  "start": 1,  "end": 45461900, "pageSize": 50000}'
END_MOVIEAKA=$(date +"%d-%m-%Y %T")
echo ""
echo "ended MOVIEAKA  at  ${END_MOVIEAKA}"

START_PERSON=$(date +"%d-%m-%Y %T")
echo "started  PERSON at  ${START_PERSON}"
curl -X POST http://localhost:18900/tsv/read -H "Content-Type: application/json" -d '{  "tsvType": "PERSON",  "start": 1,  "end": 15508003, "pageSize": 50000}'
END_PERSON=$(date +"%d-%m-%Y %T")
echo ""
echo "ended PERSON  at  ${END_PERSON}"

START_RATING=$(date +"%d-%m-%Y %T")
echo "started  RATING at  ${START_RATING}"
curl -X POST http://localhost:18900/tsv/read -H "Content-Type: application/json" -d '{  "tsvType": "RATING",  "start": 1,  "end": 1994595, "pageSize": 50000}'
END_RATING=$(date +"%d-%m-%Y %T")
echo ""
echo "ended RATING  at  ${END_CRATING}"

START_PRINCIPAL=$(date +"%d-%m-%Y %T")
echo "started  PRINCIPAL at  ${START_PRINCIPAL}"
curl -X POST http://localhost:18900/tsv/read -H "Content-Type: application/json" -d '{  "tsvType": "PRINCIPAL",  "start": 1,  "end": 60000000, "pageSize": 50000}'
END_PRINCIPAL=$(date +"%d-%m-%Y %T")
echo ""
echo "ended PRINCIPAL  at  ${END_PRINCIPAL}"

echo ""
END_ALL=$(date +"%d-%m-%Y %T")
echo "full index started @ ${START_ALL} and ended @ ${END_ALL}"
