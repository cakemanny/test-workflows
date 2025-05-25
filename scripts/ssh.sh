#!/bin/sh

exec ssh -F ./configs/ssh_config -i keys/id_ed25519 flatcar "$@"
