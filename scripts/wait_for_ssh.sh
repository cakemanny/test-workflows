#!/bin/sh

set -eu

while ! ssh -F ./configs/ssh_config flatcar hostname; do
    sleep 1
done
