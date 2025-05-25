#!/bin/sh

set -eu

while ! ./scripts/ssh.sh hostname; do
    sleep 1
done
