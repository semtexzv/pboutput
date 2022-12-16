#!/usr/bin/env bash

set -exu

HERE=$(dirname -- "$(readlink -f -- "$0")")
VER=15

export HERE
export VER

"$HERE"/make-db.sh
"$HERE"/install-ext.sh

for DIR in ./tests/*/; do
  DIR=$(realpath $DIR)

  psql -c "
DO \$$ DECLARE
    r RECORD;
BEGIN
    FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = current_schema()) LOOP
        EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
    END LOOP;
END \$$;
"
  psql -a -f ${DIR}/run.sql
  rm -f ${DIR}/last.textproto

  psql -o tmp.txt -t -c "SELECT encode(data, 'hex') FROM pg_logical_slot_peek_binary_changes('slot', NULL, NULL)"
  cat tmp.txt | while read -r line; do
    echo $line | xxd -r -p >/tmp/x.bin

    (
      echo 'msg {'
      protoc --decode pboutput.Message proto/pboutput.proto </tmp/x.bin
      echo '}'
    ) >>${DIR}/last.textproto
  done
done
