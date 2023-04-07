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

"proxy_busy_buffers_size" must be equal to or greater than the maximum of the value of "proxy_buffer_size" and one of
the "proxy_buffers" in /etc/nginx/nginx.conf:61