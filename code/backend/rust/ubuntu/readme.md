# Ubuntu services

copy to 

```
/lib/systemd/system
```


run 
```
systemctl daemon-reload
```

run 
```
sudo systemctl enable rust_create_search_index.service
sudo systemctl enable rust_search_movie.service
sudo systemctl enable rust_search_person.service
sudo systemctl enable rust_search_principal.service
```