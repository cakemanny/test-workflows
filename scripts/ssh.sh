#!/bin/sh

exec ssh -F ./configs/ssh_config flatcar "$@"
