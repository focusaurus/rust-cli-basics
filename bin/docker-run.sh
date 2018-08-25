#!/usr/bin/env bash

# Please Use Google Shell Style: https://google.github.io/styleguide/shell.xml

# ---- Start unofficial bash strict mode boilerplate
# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -o errexit  # always exit on error
set -o errtrace # trap errors in functions as well
set -o pipefail # don't ignore exit codes when piping output
set -o posix    # more strict failures in subshells
# set -x          # enable debugging

IFS="$(printf "\n\t")"
# ---- End unofficial bash strict mode boilerplate

cd "$(dirname "$0")/.."
exec docker run --rm --interactive --tty \
  --attach stdin --attach stdout --attach stderr \
  --volume "${PWD}:/opt" \
  --volume $SSH_AUTH_SOCK:/ssh-agent \
  --env SSH_AUTH_SOCK=/ssh-agent \
  --user "${USER}" \
  divide "${1-/bin/bash}"
