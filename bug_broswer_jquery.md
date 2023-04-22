sudo systemctl stop rust_proxythingi_v6.service
sudo systemctl stop rust_search_search_index.service
sudo systemctl stop rust_create_search_article.service
sudo systemctl stop rust_priceservice.service
sudo systemctl stop rust_customerpriceservice.service



cd micro-all-the-things/code/backend/rust/search/rust_search_search_index
cargo run --release

cd micro-all-the-things/code/backend/rust/search/rust_search_article
cargo run --release


cd /home/bumzack/micro-all-the-things/code/backend/rust/price/priceservice
cargo run --release


cd /home/bumzack/micro-all-the-things/code/backend/rust/customer/customerpriceservice
cargo run --release



cd proxythingis/warp-proxy-v6
cargo run --release


 cd /var/log



HEADERS  vvvvv
curl -vv -X POST http://proxy.proxythingi.at/rust/solr/search -H 'origin: http://localhost:58598'  -H 'Content-Type: application/json'  -H 'accept-language: de,en-US;q=0.7,en;q=0.3'   -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/111.0'  -H 'Content-Type: application/json'  -H  'accept: application/json, text/javascript, */*; q=0.01'  -d '{ "q" : "Edison Kinetoscopic Record of a Sneeze", "offset" : 0, "limit": 1, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9" }   }' | jq





Local vom Server direkt das Service 
curl -vv -X POST http://localhost:18600/api/v1/solr/article  -H 'Content-Type: application/json'  -d '{ "q" : "Edison Kinetoscopic Record of a Sneeze", "offset" : 0, "limit": 1, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9" }   }' | jq


Local vom Server: über den ProxyThingi

curl -vv -X POST http://proxy.proxythingi.at/rust/solr/search   -H 'Content-Type: application/json'        -d '{ "q" : "Edison Kinetoscopic Record of a Sneeze", "offset" : 0, "limit": 1, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9" }   }'  

 
REmote via eigener URL 
curl -vv -X POST http://search.rust.bumzack.at/api/v1/solr/article     -H 'Content-Type: application/json'    -d '{ "q" : "Edison Kinetoscopic Record of a Sneeze", "offset" : 0, "limit": 1, "customer" : {  "customer_id": 1, "jwt" : "eyJhbGciOiJIUzM4NCJ9.eyJjdXN0b21lcl9pZCI6IjEifQ.ygrMNXNsg00VwM6u0mk_WlUZvYKlVYDCgOi7trRnw3MrcEnwu-zIp-JbNCYqNlp9" }   }' 

 


curl -vv -X POST http://searchindex.rust.bumzack.at/api/v1/solr/searchindex/search     -H 'Content-Type: application/json'    -d '{ "q" : "Edison Kinetoscopic Record of a Sneeze", "offset" : 0, "limit": 1   }' 

curl -vv -X POST http://localhost:18320/api/v1/solr/searchindex/search     -H 'Content-Type: application/json'    -d '{ "q" : "Edison Kinetoscopic Record of a Sneeze", "offset" : 0, "limit": 1   }' 


http://proxy.proxythingi.at/rust/solr/search
