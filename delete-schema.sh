#/!/bin/zsh


curl -X POST -H 'Content-type:application/json' --data-binary '{
  "delete-field":{       "name":"article"  },
 
}' http://localhost:8984/solr/products/schema



