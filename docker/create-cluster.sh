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

  # Check that the configuration directory for the cluster exists.
  if [ ! -f "${name}/teleport.yaml" ]; then
    die "Could not read teleport.yaml from the \`${name}' configuration\n"
    exit 1
  fi

  # Check that the compose file for the cluster exists.
  if [ -f "${name}/docker-compose.yaml" ]; then
    compose_path="${name}/docker-compose.yaml"
  elif [ -f "${name}/compose.yaml" ]; then
    compose_path="${name}/compose.yaml"
  else
    die "Could not locate docker-compose.yaml from the \`${name}' configuration\n"
    exit 1
  fi

  # Source overridable variables.
  if [ -f "${name}/env" ]; then
    source "${name}/env"
  fi

  local proxy_addr="${TELEPORT_EXPOSED_PROXY_ADDR:-localhost}"
  local proxy_port="${TELEPORT_EXPOSED_PROXY_PORT:-3080}"

  # Bring up the containers.
  info "Spinning up containers for the \`${name}' configuration\n"
  $DOCKER_COMPOSE -f "${compose_path}" up --wait

  # Wait for the auth service to be ready.
  info "Waiting for ${proxy_addr}:${proxy_port} to be up and ready"

  for i in {1..60}; do
    if ! curl -s -k "https://${proxy_addr}:${proxy_port}/webapi/ping" > /dev/null; then
      echo -n "."

      if [ "${i}" -eq "60" ]; then
        echo
        die "Could not connect to Teleport proxy\n"
        exit 1
      fi

      sleep 1
    else
      echo
      break
    fi
  done

  local teleport_admins="${TELEPORT_ADMIN_USERS:-foo}"
  local teleport_users="${TELEPORT_REGULAR_USERS:-baz}"

  for user in ${teleport_admins}; do
    info "Creating Teleport admin user \`${user}'\n"

    $DOCKER_COMPOSE -f "${compose_path}" exec "${TELEPORT_AUTH_SERVICE_NAME}" tctl users add --roles=editor,access,auditor "${user}" || true
  done

  for user in ${teleport_users}; do
    info "Creating Teleport user \`${user}'\n"

    $DOCKER_COMPOSE -f "${compose_path}" exec "${TELEPORT_AUTH_SERVICE_NAME}" tctl users add --roles=access "${user}" || true
  done
}

main "$@"
