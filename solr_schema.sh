#/!/bin/zsh

# curl  -vvvv  -X POST -H 'Content-Type: application/json' 'http://localhost:8984/solr/products/update?commitWithin=100&overwrite=true&wt=json' --data-binary '[ { "id": "1",     "title": "Doc 1" },   { "id": "2",  "title": "Doc 2"   } ]'

# http://localhost:8984/solr/admin/cores?action=CREATE&name=products&numShards=2&replicationFactor=2

#  curl 	http://localhost:8984/solr/admin/cores?_=1677079640717&action=CREATE&name=blupp&wt=json


DIR=$PWD
echo "creating schema SolR on port 8984"
cd /Users/gsc/stoff/mylogalyzer/solr-10.0.0-SNAPSHOT/bin
./solr delete -c products  -p 8984 
./solr delete -c categories  -p 8984 
./solr delete -c xinet  -p 8984 


./solr create_core -c products  -p 8984 
./solr config -c products -p 8984 -action set-user-property -property update.autoCreateFields -value false

./solr create_core -c categories  -p 8984 
./solr config -c categories -p 8984 -action set-user-property -property update.autoCreateFields -value false

./solr create_core -c xinet  -p 8984 
./solr config -c xinet -p 8984 -action set-user-property -property update.autoCreateFields -value false




cd $DIR

curl -X POST -H 'Content-type:application/json' --data-binary '{
  "add-field":{       "name":"article",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"articleUnit",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"articleName",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"articleDescription",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"code",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"visible",              "type":"boolean",             "stored":true, indexed:"true" },
  "add-field":{       "name":"orderable",              "type":"boolean",             "stored":true, indexed:"true" },
  "add-field":{       "name":"sourcing",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"division",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"material",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"codeWhg",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"supplierName",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"defaultSupplier",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"module",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"moduleGroup",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"ownBrand",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"otns",              "type":"string",             "stored":true,"multiValued":true, indexed:"true" },
  "add-field":{       "name":"eans",              "type":"string",             "stored":true,"multiValued":true, indexed:"true" },
  "add-field":{       "name":"predecessorCodes",              "type":"string",             "stored":true,"multiValued":true, indexed:"true" },
  "add-field":{       "name":"predecessorEans",              "type":"string",             "stored":true,"multiValued":true, indexed:"true" },
  "add-field":{       "name":"predecessorOtns",              "type":"string",             "stored":true,"multiValued":true, indexed:"true" },
  "add-field":{       "name":"superCategories",              "type":"string",             "stored":true,"multiValued":true, indexed:"true" },
  "add-field":{       "name":"allSuperCategories",              "type":"string",             "stored":true,"multiValued":true, indexed:"true" },
  "add-field":{       "name":"imagesContainerQualifiers",              "type":"string",             "stored":true,"multiValued":true, indexed:"true" },
  "add-field":{       "name":"mainImageContainerQualifier",              "type":"string",             "stored":true,"multiValued":true, indexed:"true" }  
}' http://localhost:8984/solr/products/schema



curl -X POST -H 'Content-type:application/json' --data-binary '{
  "add-field":{       "name":"code",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"name",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"superCategories",              "type":"string",             "stored":true,"multiValued":true, indexed:"true" },
  "add-field":{       "name":"allSuperCategories",              "type":"string",             "stored":true,"multiValued":true, indexed:"true" }
  
}' http://localhost:8984/solr/categories/schema

 



curl -X POST -H 'Content-type:application/json' --data-binary '{
  "add-field":{       "name":"code",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"mediaContainerQualifier",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"url",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"width",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"height",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"channel",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"valid",              "type":"boolean",             "stored":true, indexed:"true" },
  "add-field":{       "name":"mediaFormat",              "type":"string",             "stored":true, indexed:"true" },
  "add-field":{       "name":"mime",              "type":"string",             "stored":true, indexed:"true" }
  
}' http://localhost:8984/solr/xinet/schema

 
 
 
 
 
 
 