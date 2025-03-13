# arcadia-index
Arcadia's backend

## Setup

- Copy `.env` to `.env.local` and replace the values with yours

- The `docker/` folder contains some containers to various services used by `actix-index`. Feel free to use these or substitutes

- Create a database with the name that is specified by the env variable `POSTGRESQL_DATABASE`

- install `sqlx-cli` and run the database migrations : `sqlx migrate run`


## Developing

```
cargo run
```