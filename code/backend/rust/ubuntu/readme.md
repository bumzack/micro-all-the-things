# Ubuntu services

copy to

sudo apt install pkg-config libssl-dev

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

sudo systemctl enable  rust_create_search_article.service
sudo systemctl enable  rust_authenticationservice
sudo systemctl enable  rust_search_search_index
```

## start

```
sudo systemctl start  rust_create_search_index.service
sudo systemctl start  rust_search_movie.service
sudo systemctl start  rust_search_person.service
sudo systemctl start  rust_search_principal.service
sudo systemctl start  rust_logservice.service
sudo systemctl start  rust_create_search_article.service
sudo systemctl start  rust_authenticationservice
sudo systemctl start  rust_search_search_index

```

## restart

```
sudo systemctl restart  rust_create_search_index.service
sudo systemctl restart  rust_search_movie.service
sudo systemctl restart  rust_search_person.service
sudo systemctl restart  rust_search_principal.service
sudo systemctl restart  rust_logservice.service
sudo systemctl restart  rust_create_search_article.service
sudo systemctl restart  rust_authenticationservice
sudo systemctl restart  rust_search_search_index
```

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

## Price and Customer

```
sudo systemctl enable rust_customerpriceservice.service
sudo systemctl enable rust_customerservice.service
sudo systemctl enable rust_priceservice.service
```

```
sudo systemctl start rust_customerpriceservice.service
sudo systemctl start rust_customerservice.service
sudo systemctl start rust_priceservice.service
```

```
sudo systemctl restart rust_customerpriceservice.service
sudo systemctl restart rust_customerservice.service
sudo systemctl restart rust_priceservice.service
```

## Price and Customer

```
sudo systemctl enable rust_customerpriceservice.service
sudo systemctl enable rust_customerservice.service
sudo systemctl enable rust_priceservice.service
```

```
sudo systemctl start rust_customerpriceservice.service
sudo systemctl start rust_customerservice.service
sudo systemctl start rust_priceservice.service
```

```
sudo systemctl restart rust_customerpriceservice.service
sudo systemctl restart rust_customerservice.service
sudo systemctl restart rust_priceservice.service
```


