#!/bin/zsh

DIR=$PWD
echo "starting SolR on port 8984"
cd /Users/gsc/stoff/mylogalyzer/solr-10.0.0-SNAPSHOT/bin
./solr start -p 8984

cd $DIR
