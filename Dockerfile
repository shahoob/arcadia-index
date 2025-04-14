FROM rust:1.86-slim-bullseye
WORKDIR /app
COPY . .

RUN if [ ! -f ".env.local" ]; then \
    echo "No custom .env.local file found, using default env"; \
    cp .env .env.local; \
fi

RUN apt-get update && apt-get install -y postgresql postgresql-client libssl-dev openssl curl
RUN mkdir -p /var/lib/postgresql/data
RUN chown -R postgres:postgres /var/lib/postgresql/data
USER postgres

# RUN initdb -D /var/lib/postgresql/data --no-locale --encoding=UTF8
RUN find /usr/lib/postgresql/*/bin/initdb -executable -type f -print -quit | xargs -I {} sh -c "{} -D /var/lib/postgresql/data --no-locale --encoding=UTF8"
RUN cat migrations/create_db.sql migrations/20250312215600_initdb.sql migrations/fixtures.sql > /var/lib/postgresql/data/init.sql
RUN nohup sh -c "psql -U arcadia -d arcadia -f /init.sql &" && sleep 4 

USER root
RUN cargo build --release

# COPY docker-entrypoint.sh /usr/local/bin/
# RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
CMD ["/app/target/release/arcadia"]