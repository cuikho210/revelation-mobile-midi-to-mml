#!/usr/bin/bash

cd $(dirname "$0")

# Env
export $(cat "./.env" | xargs)

# Alias
alias cm=$PWD/commit.sh
