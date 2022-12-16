#!/usr/bin/env bash

set -exu

TMP=$(mktemp)
TMPDIR=$(mktemp -d)

docker build --iidfile="$TMP" -f tests/Dockerfile .

docker run -v "$PWD":/home/docker/pkg:Z $(cat "$TMP") /home/docker/pkg/tests/test.sh
