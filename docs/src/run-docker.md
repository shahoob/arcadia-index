# Docker Setup

<div class="warning">

If running `docker compose` doesn't work, you may have an older version of the docker cli installed and may need to use `docker-compose` instead.
```bash
docker-compose up db -d
```

Also don't forget to use `sudo` if you aren't in the `docker` group!
</div>

## Database

The recommended method for spawning an instance of PostgreSQL is using Docker Compose:

```bash
docker compose up db -d
```

Arcadia will automatically run migrations on launch. Otherwise, initialization of the database can be done with:

```bash
cargo sqlx database setup
# If cargo doesn't know the command, install it with this command
cargo install sqlx-cli
```

### Adding Test Data

You can optionally add "fake" data to the database by running the `fixtures.sql` file in the migrations/fixtures directory. This allows to quickly have data to work with.

Here is how to insert them if you are using docker:

```bash
docker exec -i arcadia_db psql -U arcadia -d arcadia < backend/migrations/fixtures/fixtures.sql
```

The default user defined in the test data is `picolo` with password `test`.

### Exporting Test Data

If you wish to create your own additional test data, you can then dump it to a file with this command (if you want to share it in the repo):

```bash
docker exec -i arcadia_db pg_dump -U arcadia -d arcadia --data-only --inserts > backend/migrations/fixtures/fixtures.sql
```

## Backend

Launch the server:

```bash
docker compose up backend -d
```

It may take a while for the image to build as everything is done in Docker. Yes, Docker Compose can handle that!

### Auto-rebuild with Compose Watch

What Docker Compose can also handle is auto rebuilding images when source code changes as made possible by [Compose Watch](https://docs.docker.com/compose/how-tos/file-watch/).

To take advantage of that, just run this command instead:

```bash
docker compose up backend --watch
```

Or when running attached (as in not just turning it on and leaving it with the `--detach` / `-d` option), just press <kbd>W</kbd>.

Now when you make changes to the backend, compose will automatically rebuild the image and restart the container with the new source code, making *somewhat* quicker to iterate while also testing in Docker.

## Frontend

Launch the server:

```bash
docker compose up frontend -d
```
