# arcadia-index
Arcadia's backend

Arcadia is composed of two components:
- The postgres database
- The server

## Developper Setup

### Required tools
- (sqlx-cli)[https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md]
- (cargo)[https://doc.rust-lang.org/cargo/getting-started/installation.html]
- (docker (recommended))[https://docs.docker.com/desktop/setup/install]

### Env
Copy `.env` to `.env.local`

### Database

The recommended method is using docker:
```
docker-compose -f docker/postgres.yml up -d
sqlx migrate run
```

### Launch server
```
cargo run
```

## Database

### Creating a migration file
TODO

## Notes

- The current auth mechanism relies on an actix extractor, which is the user provider. Everytime a handler accesses the current user, the authentication takes place. This might be replaced by a middleware in the future.
