# Log service testing

## read entries

```
curl  -vvvv  -X POST -H 'Content-Type: application/json' 'http://localhost:18700/api/log/entries' --data-binary ' { "last_n": 50  }'
```

## add entry

```
curl -X POST  http://localhost:18700/api/log/entry  -H  "Content-Type: application/json" -d '  {  "service_id": "rust_searchmovie",  "log_type": "INFO",  "message": "my ultra important log message", "logtime": "1970-01-01 00:01:01 UTC" }   '      
```

sudo -u bumzack psql bumzack

ALTER USER bumzack WITH PASSWORD  'bumzack';

