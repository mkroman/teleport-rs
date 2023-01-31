#!/usr/bin/env bash

# Copyright 2023 Mikkel Kroman <mk@maero.dk>
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
# or https://opensource.org/licenses/MIT>, at your option. This file may not be
# copied, modified, or distributed except according to those terms.

set -o errexit
set -o nounset
set -o pipefail

SCRIPT_ABSOLUTE_PATH="$(readlink -f -- "${BASH_SOURCE[0]}")"
SCRIPT_ABSOLUTE_DIR="$(dirname -- "${SCRIPT_ABSOLUTE_PATH}")"
DOCKER_COMPOSE="${DOCKER_COMPOSE:-docker compose}"

show_help() {
  cat <<EOF
Usage: $(basename "$0") [cluster name]
EOF
  exit 1
}

log() {
  local level="${1}"
  shift
  printf "%s %-5s %b" "$(date --rfc-3339=ns)" "${level}" "$@"
}

die() {
  log "FATAL" "$@"
  exit 1
}

info() {
  log "INFO" "$@"
}

main() {
  local name="${1:-single-node}"
  local compose_path=

  # Enter the docker directory.
  cd "${SCRIPT_ABSOLUTE_DIR}"

  # Check that the compose file for the cluster exists.
  if [ -f "${name}/docker-compose.yaml" ]; then
    compose_path="${name}/docker-compose.yaml"
  elif [ -f "${name}/compose.yaml" ]; then
    compose_path="${name}/compose.yaml"
  else
    die "Could not locate docker-compose.yaml from the \`${name}' configuration\n"
    exit 1
  fi

  # Check if containers exist for the services.
  container_ids="$(${DOCKER_COMPOSE} -f "${compose_path}" ps -a -q)"

  # If there were any container ids, take down the composition.
  if [ -n "${container_ids}" ]; then
    info "Destroying containers for the \`${name}' configuration\n"

    $DOCKER_COMPOSE -f "${compose_path}" down
  fi
}

main "$@"
