# Rust Things


## meilisearch

Start command: 

```
../target/release/meilisearch --no-analytics --http-addr 'localhost:18984' --db-path ../../data/meilifile```

### deactivate telemetrics 

```
meilisearch --no-analytics
```

or via environment variable 
```
export MEILI_NO_ANALYTICS=true
meilisearch
```


###  Configure DB file

```
--db-path ./meilifile
```

###  Configure http stuff
```
--http-addr 'localhost:18984'
```