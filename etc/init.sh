#!/bin/bash

set -e

GIT_HOOK_DIR=$(git rev-parse --show-toplevel)/.git/hooks

ETC_DIR=$(realpath $(dirname ${BASH_SOURCE[0]}))

if [ ! -f $GIT_HOOK_DIR/pre-commit ]; then
  echo "Adding Git pre-commit hook"
  ln -s $ETC_DIR/hooks/pre-commit.sh $GIT_HOOK_DIR/pre-commit
fi
