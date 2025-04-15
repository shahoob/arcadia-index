# syntax=docker/dockerfile:1
FROM rust:1.86-slim-bullseye AS prebuild
RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

WORKDIR /app
COPY . .


RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
--mount=type=cache,target=/var/lib/apt,sharing=locked \
apt-get update && apt-get install -y postgresql postgresql-client libssl-dev openssl curl pkg-config wait-for-it
RUN --mount=type=cache,target=/root/.cargo,sharing=locked \
cargo install sqlx-cli --no-default-features --features native-tls,postgres
ENV PGDATA="/var/lib/postgresql/data"
RUN mkdir -p ${PGDATA}
RUN chown -R postgres:postgres ${PGDATA}
ENV DATABASE_URL="postgresql://arcadia:arcadia@127.0.0.1:5432/arcadia"
USER postgres

RUN find /usr/lib/postgresql/*/bin/initdb -executable -type f -print -quit | xargs -I {} nohup sh -c "{} -D ${PGDATA} --no-locale --encoding=UTF8"
RUN ls ./
RUN cat /app/migrations/fixtures/create_db.sql /app/migrations/20250312215600_initdb.sql /app/migrations/fixtures/fixtures.sql > /var/lib/postgresql/data/init.sql
# the db needs to be running at build time for sqlx compile-time checks
#RUN nohup postgres -D /var/lib/postgresql/data >logfile 2>&1 &
RUN find /usr/lib/postgresql/*/bin/psql -executable -type f -print -quit | xargs -I {} nohup sh -c "{} -U arcadia -d arcadia -f /init.sql &" && sleep 4

#RUN find /usr/lib/postgresql/*/bin/pg_ctl -executable -type f -print -quit | xargs -I {} nohup sh -c "{} start -D ${PGDATA}" 

USER root
RUN sleep 5
RUN wait-for-it localhost:5432 -- echo databaseReady

RUN find /usr/lib/postgresql/*/bin/pg_isready -executable -type f -print -quit | xargs -I {} sh -c "while true; do {} -h 127.0.0.1 -p 5432 && exit 0 || sleep 4; done"

RUN cargo sqlx database setup

RUN cargo sqlx prepare

# COPY docker-entrypoint.sh /usr/local/bin/
# RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
# CMD ["/app/target/release/arcadia"]

FROM rust:1.86-slim-bullseye AS build
RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

WORKDIR /app

USER arcadia

COPY src/ migrations/ Cargo.toml Cargo.lock ./
COPY --from=prebuild .sqlx/ .

USER root

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && apt-get install -y --no-install-recommends libssl-dev openssl curl
RUN --mount=type=cache,target=/root/.cargo,sharing=locked \
    cargo build
