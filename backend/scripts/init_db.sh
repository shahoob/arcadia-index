#!/bin/bash

set -e

if [ -z ${DATABASE_URL+x} ]; then
  echo "required envvar DATABASE_URL is not set";
  exit 1;
fi

./sqlx database setup --source ./migrations
