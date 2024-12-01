# Example ToDo App 

## Tech Stack
- Postgres
- Rust (Backend)
- Docker

## REST
    [X] Welcome bunner 
    [x] Swagger UI
    [x] Logger
    

## Database
    [x] starts with docker compose
    [x] has simple  PG UI ( http://localhost:8088/ )
    [x] initial table structure - see ./pgInit/init.sql

```shell
  docker compose -f .\compose.yaml up
```


## ToDo:
- Paginator
- Auth/Session/Users
- Benchmark