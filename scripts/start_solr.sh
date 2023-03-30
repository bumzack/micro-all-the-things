#!/bin/zsh

DIR=$PWD
echo "starting SolR on port 8984"
cd /Users/gsc/stoff/micro-all-the-things/solr-10.0.0-SNAPSHOT/bin
./solr start -p 8984   -m 96g

cd $DIR



./solr stop
