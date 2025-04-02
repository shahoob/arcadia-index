First, thanks for considering contributing to Arcadia's devevelopment !

## Developper Setup

Arcadia-index is a [REST](https://en.wikipedia.org/wiki/REST) API written in rust with the [actix](https://github.com/actix/actix-web) framework and the [sqlx](https://github.com/launchbadge/sqlx) database driver.

It (will) also contain some scripts that are meant to be run on a regular basis (for example as cron jobs) when the site is on production.

### Recommended tools
- [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [docker](https://docs.docker.com/desktop/setup/install)

### Env
Copy `.env` to `.env.local`

### Database

Arcadia uses a postgresql database.

The recommended method is using docker:
```
docker-compose -f docker/postgres.yml up -d
```

The database schema is created in a migration file `migrations/*_initdb.sql`. It can be ran manually, or with sqlx-cli : `sqlx migrate run`. This command will establish a connection to the database with the details given in `.env.local` and run the SQL code in the migration file.

You can optionally add "fake" data to the databse by running the `fixtures.sql` file in the migrations directory. This allows to quickly have data to work with.

The default user is :

```
username: picolo
password: test
```

### Launch server

```
cargo run
```

This will start the development server in dev mode.

## Code structure

API calls are forwarded to `handlers`, database requests are done by `repositories`, objects are defined by `models`. Directories with those names contain the relevant code.

There is currently no swagger (see [this issue](https://github.com/Arcadia-Solutions/arcadia-index/issues/1)), the routes are defined in `src/routes.rs` and can, for now, easily be browsed.

## Contributing

Wheter you want to add a new feature or fix an existing issue, it needs to be done on your own branch :

- fork this repo
- clone it locally on your computer
- create a new branch `feature-name` or `bug-name-fix` (with the proper name)
- open a pull request when your contribution is done

If you are unsure about what/how to do something, don't hesitate to [open a discussion](https://github.com/Arcadia-Solutions/arcadia-index/discussions) or [an issue](https://github.com/Arcadia-Solutions/arcadia-index/issues) about the topic.

You can also hop on the [discord server](https://discord.gg/amYWVk7pS3) to chat with other devs and the community.

### Finding contributions to make

Arcadia-index has a [board](https://github.com/orgs/Arcadia-Solutions/projects/1) to track the existing issues and features that need to be worked on. Feel free to claim one that isn't claimed yet before starting to work on it.