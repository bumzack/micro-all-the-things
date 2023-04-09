#!/bin/zsh

# curl  -vvvv  -X POST -H 'Content-Type: application/json' 'http://localhost:8984/solr/products/update?commitWithin=100&overwrite=true&wt=json' --data-binary '[ { "id": "1",     "title": "Doc 1" },   { "id": "2",  "title": "Doc 2"   } ]'

# http://localhost:8984/solr/admin/cores?action=CREATE&name=products&numShards=2&replicationFactor=2

#  curl 	http://localhost:8984/solr/admin/cores?_=1677079640717&action=CREATE&name=blupp&wt=json

DIR=$PWD
echo "creating schema SolR on port 8984"
cd /Users/gsc/stoff/micro-all-the-things/solr-10.0.0-SNAPSHOT/bin

./solr delete -c movie -p 8984
./solr delete -c movieaka -p 8984
./solr delete -c person -p 8984
./solr delete -c rating -p 8984
./solr delete -c principal -p 8984
./solr delete -c crew -p 8984
./solr delete -c episode -p 8984
./solr delete -c searchindex -p 8984

./solr create_core -c movie -p 8984
./solr config -c movie -p 8984 -action set-user-property -property update.autoCreateFields -value false

./solr create_core -c movieaka -p 8984
./solr config -c movieaka -p 8984 -action set-user-property -property update.autoCreateFields -value false

./solr create_core -c person -p 8984
./solr config -c person -p 8984 -action set-user-property -property update.autoCreateFields -value false

./solr create_core -c rating -p 8984
./solr config -c rating -p 8984 -action set-user-property -property update.autoCreateFields -value false

./solr create_core -c principal -p 8984
./solr config -c principal -p 8984 -action set-user-property -property update.autoCreateFields -value false

./solr create_core -c crew -p 8984
./solr config -c crew -p 8984 -action set-user-property -property update.autoCreateFields -value false

./solr create_core -c episode -p 8984
./solr config -c episode -p 8984 -action set-user-property -property update.autoCreateFields -value false

./solr create_core -c searchindex -p 8984
./solr config -c searchindex -p 8984 -action set-user-property -property update.autoCreateFields -value false

cd $DIR

curl -X POST -H 'Content-type:application/json' --data-binary '{
  "add-field":{       "name":"tconst",              "type":"string",             "stored":true,     indexed:"true",    "multiValued":false     },
  "add-field":{       "name":"titleType",           "type":"string",             "stored":true,     indexed:"true",    "multiValued":false     },
  "add-field":{       "name":"primaryTitle",        "type":"text_general",       "stored":true,     indexed:"true",    "multiValued":false     },
  "add-field":{       "name":"originalTitle",       "type":"text_general",       "stored":true,     indexed:"true",    "multiValued":false     },
  "add-field":{       "name":"adult",               "type":"boolean",            "stored":true,     indexed:"true",    "multiValued":false     },
  "add-field":{       "name":"startYear",           "type":"pint",               "stored":true,     indexed:"true",    "multiValued":false     },
  "add-field":{       "name":"endYear",             "type":"pint",               "stored":true,     indexed:"true",    "multiValued":false     },
  "add-field":{       "name":"runtimeMinutes",      "type":"pint",               "stored":true,     indexed:"true",    "multiValued":false     },
  "add-field":{       "name":"genres",              "type":"text_general",       "stored":true,     indexed:"true",    "multiValued":true      }
}' http://localhost:8984/solr/movie/schema

curl -X POST -H 'Content-type:application/json' --data-binary '{
  "add-field":{       "name":"titleId",             "type":"string",            "stored":true,    indexed:"true",     "multiValued":false   },
  "add-field":{       "name":"ordering",            "type":"pint",              "stored":true,    indexed:"true",     "multiValued":false   },
  "add-field":{       "name":"title",               "type":"text_general",      "stored":true,    indexed:"true",     "multiValued":false   },
  "add-field":{       "name":"region",              "type":"text_general",      "stored":true,    indexed:"true",     "multiValued":false   },
  "add-field":{       "name":"language",            "type":"text_general",      "stored":true,    indexed:"true",     "multiValued":false   },
  "add-field":{       "name":"types",               "type":"text_general",      "stored":true,    indexed:"true",     "multiValued":true    },
  "add-field":{       "name":"attributes",          "type":"text_general",      "stored":true,    indexed:"true",     "multiValued":true    },
  "add-field":{       "name":"originalTitle",       "type":"boolean",           "stored":true,    indexed:"true",     "multiValued":false   }
}' http://localhost:8984/solr/movieaka/schema

curl -X POST -H 'Content-type:application/json' --data-binary '{
  "add-field":{       "name":"nconst",              "type":"string",        "stored":true,  indexed:"true",     "multiValued":false     },
  "add-field":{       "name":"primaryName",         "type":"text_general",  "stored":true,  indexed:"true",     "multiValued":false     },
  "add-field":{       "name":"birthYear",           "type":"pint",          "stored":true,  indexed:"true",     "multiValued":false     },
  "add-field":{       "name":"deathYear",           "type":"pint",          "stored":true,  indexed:"true",     "multiValued":false     },
  "add-field":{       "name":"primaryProfession",   "type":"string",        "stored":true,  indexed:"true",     "multiValued":true      },
  "add-field":{       "name":"knownForTitles",      "type":"string",        "stored":true,  indexed:"true",     "multiValued":true      }
}' http://localhost:8984/solr/person/schema

curl -X POST -H 'Content-type:application/json' --data-binary '{
  "add-field":{       "name":"tconst",             "type":"string",    "stored":true,     indexed:"true",     "multiValued":false   },
  "add-field":{       "name":"averageRating",      "type":"pfloat",    "stored":true,     indexed:"true",     "multiValued":false   },
  "add-field":{       "name":"numVotes",           "type":"pint",      "stored":true,     indexed:"true",     "multiValued":false   }
}' http://localhost:8984/solr/rating/schema

curl -X POST -H 'Content-type:application/json' --data-binary '{
  "add-field":{       "name":"tconst",          "type":"string",        "stored":true,    indexed:"true",     "multiValued":false      },
  "add-field":{       "name":"ordering",        "type":"pint",          "stored":true,    indexed:"true",     "multiValued":false      },
  "add-field":{       "name":"nconst",          "type":"string",        "stored":true,    indexed:"true",     "multiValued":false      },
  "add-field":{       "name":"category",        "type":"text_general",  "stored":true,    indexed:"true",     "multiValued":false      },
  "add-field":{       "name":"job",             "type":"text_general",  "stored":true,    indexed:"true",     "multiValued":false      },
  "add-field":{       "name":"characters",      "type":"text_general",  "stored":true,    indexed:"true",     "multiValued":true       }
}' http://localhost:8984/solr/principal/schema

curl -X POST -H 'Content-type:application/json' --data-binary '{
  "add-field":{       "name":"tconst",       "type":"string",         "stored":true,       indexed:"true",       "multiValued":false   },
  "add-field":{       "name":"directors",    "type":"string",         "stored":true,       indexed:"true",       "multiValued":true    },
  "add-field":{       "name":"writers",      "type":"string",         "stored":true,       indexed:"true",       "multiValued":true    }
}' http://localhost:8984/solr/crew/schema

curl -X POST -H 'Content-type:application/json' --data-binary '{
  "add-field":{   "name":"tconst",              "type":"string",         "stored":true,  indexed:"true",     "multiValued":false     },
  "add-field":{   "name":"parentTconst",        "type":"string",         "stored":true,  indexed:"true",     "multiValued":false     },
  "add-field":{   "name":"seasonNumber",        "type":"pint",           "stored":true,  indexed:"true",     "multiValued":false     },
  "add-field":{   "name":"episodeNumber",       "type":"pint",           "stored":true,  indexed:"true",     "multiValued":false     }
}' http://localhost:8984/solr/episode/schema

curl -X POST -H 'Content-type:application/json' --data-binary '{
  "add-field":{   "name":"tconst",              "type":"string",        "stored":true,      indexed:"true",     "multiValued":false     },
  "add-field":{   "name":"originalTitle",       "type":"string",        "stored":true,      indexed:"true",     "multiValued":false     },
  "add-field":{   "name":"primaryTitle",        "type":"string",        "stored":true,      indexed:"true",     "multiValued":false     },
  "add-field":{   "name":"titles",              "type":"string",        "stored":true,      indexed:"true",     "multiValued":true      },
  "add-field":{   "name":"actors",              "type":"string",        "stored":true,      indexed:"true",     "multiValued":true      },
  "add-field":{   "name":"directors",           "type":"string",        "stored":true,      indexed:"true",     "multiValued":true      },
  "add-field":{   "name":"writers",             "type":"string",        "stored":true,      indexed:"true",     "multiValued":true      },
  "add-field":{   "name":"genres",              "type":"string",        "stored":true,      indexed:"true",     "multiValued":true      },
  "add-field":{   "name":"characters",          "type":"string",        "stored":true,      indexed:"true",     "multiValued":true      },
  "add-field":{   "name":"adult",               "type":"boolean",       "stored":true,      indexed:"true",     "multiValued":false     },
  "add-field":{   "name":"titleType",           "type":"string",        "stored":true,      indexed:"true",     "multiValued":false     },
  "add-field":{   "name":"runtimeMinutes",      "type":"pint",          "stored":true,      indexed:"true",     "multiValued":false     }

}' http://localhost:8984/solr/searchindex/schema
