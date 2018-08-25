#!/usr/bin/env bash

# Please Use Google Shell Style: https://google.github.io/styleguide/shell.xml

# ---- Start unofficial bash strict mode boilerplate
# http://redsymbol.net/articles/unofficial-bash-strict-mode/
set -o errexit  # always exit on error
set -o errtrace # trap errors in functions as well
set -o pipefail # don't ignore exit codes when piping output
set -o posix    # more strict failures in subshells
set -x          # enable debugging

IFS="$(printf "\n\t")"
# ---- End unofficial bash strict mode boilerplate

cd "$(dirname "$0")/.."

# The backslash escaped variables below are so bash doesn't immediately
# replace them with their environment variable values before passing to docker
# FROM rust:1.28.0
dockerfile=$(
  cat <<EOF
FROM nrdmn/clippy:0.0.197
ARG USER
ARG USER_ID=1000
ARG GROUP_ID=1000
RUN addgroup --gid \${GROUP_ID} \${USER} || true
RUN adduser --disabled-password --gid \${GROUP_ID} --uid \${USER_ID} --gecos \${USER} \${USER} || true
WORKDIR /opt
ENV PATH=/usr/local/bin:/usr/local/cargo/bin:/usr/sbin:/usr/bin:/sbin:/bin:/opt/target/debug
ENV USER=\${USER}
CMD ["bash"]
EOF
)
echo "${dockerfile}" | docker build \
  --tag divide \
  --build-arg "USER=${USER}" \
  --build-arg "USER_ID=$(id -u)" \
  --build-arg "GROUP_ID=$(id -g)" \
  -
