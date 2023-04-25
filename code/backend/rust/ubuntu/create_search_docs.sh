#!/bin/zsh

START=$(date +"%d-%m-%Y %T")
echo "started  building of searchindex at  ${START}"


echo "start, pagesize, tasks"
curl -vvv http://localhost:18300/api/v4/solr/searchindex/build/0/5000/12000000/1

END=$(date +"%d-%m-%Y %T")

echo ""
echo "started  building of searchindex @ ${START}. ended @ ${END}"
