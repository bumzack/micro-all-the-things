#!/bin/zsh

START=$(date +"%d-%m-%Y %T")
echo "started  building of searchindex at  ${START}"


echo "start, pagesize, max-movies, tasks (unused)"
curl -vvv http://localhost:18300/api/v3/solr/searchindex/build/0/500/13000000/1

END=$(date +"%d-%m-%Y %T")

echo ""
echo "started  building of searchindex @ ${START}. ended @ ${END}"
