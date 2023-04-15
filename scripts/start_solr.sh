#!/bin/zsh

## git clone https://github.com/apache/solr.git
# https://cwiki.apache.org/confluence/display/SOLR/Building+Solr+with+Gradle
# ./gradlew  assemble

DIR=$PWD
echo "starting SolR on port 8984"
cd /Users/gsc/stoff/micro-all-the-things/solr-10.0.0-SNAPSHOT/bin
./solr start -p 8984   -m 64g

cd $DIR



./solr stop
