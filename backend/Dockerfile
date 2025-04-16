FROM rust:1.86-slim-bullseye AS prebuild
# Don't delete any downloaded packages, we'd be using BuildKit cache mounts later!
RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

WORKDIR /app
COPY . .
COPY .sqlx/ ./sqlx/

# This doesn't mount anything from your system, it's all in the build cache
# Helps speed up repetitive docker builds by A LOT
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && apt-get install -y libssl-dev openssl curl pkg-config


FROM prebuild AS build-prod

RUN SQLX_OFFLINE=true cargo build --release

FROM prebuild AS build-debug

RUN SQLX_OFFLINE=true cargo build

# use --target debug when you want to build the debug variant
FROM debian:bullseye-slim AS debug
WORKDIR /app

COPY --chmod=777 --from=build-debug /app/target/debug/arcadia-index .
CMD ["arcadia-index"]

FROM debian:bullseye-slim
WORKDIR /app

COPY --chmod=777 --from=build-prod /app/target/release/arcadia-index .

# COPY docker-entrypoint.sh /usr/local/bin/
# RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
CMD ["arcadia-index"]
