#!/usr/bin/env bash

set -exuo pipefail

sudo chmod ugo+rw /usr/share/postgresql/${VER}/extension
sudo chmod ugo+rw /usr/lib/postgresql/${VER}/lib/

cargo pgx install
