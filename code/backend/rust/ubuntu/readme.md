# Ubuntu services

copy to

```
/lib/systemd/system
```

run

```
systemctl daemon-reload
```

## enable

```
sudo systemctl enable rust_create_search_index.service
sudo systemctl enable rust_search_movie.service
sudo systemctl enable rust_search_person.service
sudo systemctl enable rust_search_principal.service
sudo systemctl enable rust_logservice.service

```

## start

```
sudo systemctl start  rust_create_search_index.service
sudo systemctl start  rust_search_movie.service
sudo systemctl start  rust_search_person.service
sudo systemctl start  rust_search_principal.service
sudo systemctl start  rust_logservice.service
```

## restart

```
sudo systemctl restart  rust_create_search_index.service
sudo systemctl restart  rust_search_movie.service
sudo systemctl restart  rust_search_person.service
sudo systemctl restart  rust_search_principal.service
sudo systemctl restart  rust_logservice.service
```

Apr 7 17:28:26 bumzack-rust rust_personsearch[731]: [2023-04-07T17:28:26Z ERROR common::meili_search] remote address
Some(195.201.240.124:80)
Apr 7 17:28:26 bumzack-rust
rust_personsearch[731]: [2023-04-07T17:28:26Z ERROR common::meili_filter::meili_filter_person] meili_filter_person
request != OK. status 408,
Apr 7 17:28:26 bumzack-rust
rust_personsearch[731]: [2023-04-07T17:28:26Z ERROR common::meili_filter::meili_filter_person] meili_filter_person
request != OK. headers {"server": "nginx/1.18.0 (Ubuntu)", "date": "Fri, 07 Apr 2023 17:28:26 GMT", "content-length": "
0", "connection": "keep-alive"},
Apr 7 17:28:26 bumzack-rust rust_personsearch[731]: [2023-04-07T17:28:26Z ERROR common::meili_search] request != OK.
status 408, url http://meilisearch01.bumzack.at/indexes/person/search
Apr 7 17:28:26 bumzack-rust rust_personsearch[731]: [2023-04-07T17:28:26Z ERROR common::meili_search] request != OK.
headers {"server": "nginx/1.18.0 (Ubuntu)", "date": "Fri, 07 Apr 2023 17:28:26 GMT", "content-length": "0", "
connection": "keep-alive"}, url http://meilisearch01.bumzack.at/indexes/person/search
Apr 7 17:28:26 bumzack-rust rust_personsearch[731]: [2023-04-07T17:28:26Z ERROR common::meili_search] remote address
Some(195.201.240.124:80)
Apr 7 17:28:26 bumzack-rust
rust_personsearch[731]: [2023-04-07T17:28:26Z ERROR common::meili_filter::meili_filter_person] meili_filter_person
request != OK. status 408,
Apr 7 17:28:26 bumzack-rust rust_personsearch[731]: [2023-04-07T17:28:26Z ERROR common::meili_filter::
meili_filter_persorust_create_search_index.service

## Data Supply

```
sudo systemctl enable rust_crewwriter.service
sudo systemctl enable rust_episodewriter.service
sudo systemctl enable rust_movieakaswriter.service
sudo systemctl enable rust_moviewriter.service
sudo systemctl enable rust_personwriter.service
sudo systemctl enable rust_principalwriter.service
sudo systemctl enable rust_ratingwriter.service
sudo systemctl enable rust_tsvfilereader.service

```

```
sudo systemctl start rust_crewwriter.service
sudo systemctl start rust_episodewriter.service
sudo systemctl start rust_logservice.service
sudo systemctl start rust_movieakaswriter.service
sudo systemctl start rust_moviewriter.service
sudo systemctl start rust_personwriter.service
sudo systemctl start rust_principalwriter.service
sudo systemctl start rust_ratingwriter.service
sudo systemctl start rust_tsvfilereader.service

```

```
sudo systemctl restart rust_crewwriter.service
sudo systemctl restart rust_episodewriter.service
sudo systemctl restart rust_logservice.service
sudo systemctl restart rust_movieakaswriter.service
sudo systemctl restart rust_moviewriter.service
sudo systemctl restart rust_personwriter.service
sudo systemctl restart rust_principalwriter.service
sudo systemctl restart rust_ratingwriter.service
sudo systemctl restart rust_tsvfilereader.service
```













