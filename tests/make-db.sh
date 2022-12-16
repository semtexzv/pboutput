#!/usr/bin/env bash
sudo pg_createcluster ${VER} docker --start -o wal_level=logical -p 5432 -u docker

psql "dbname=postgres" -c "CREATE DATABASE docker"
psql "dbname=postgres" -c "GRANT ALL PRIVILEGES ON DATABASE docker TO docker"
