#!/bin/bash

#
# iota-sandbox __VERSION__
# https://github.com/iotaledger/iota-sandbox
#

if [[ "$OSTYPE" != "darwin"* && "$EUID" -ne 0 ]]; then
  echo "Please run as root or with sudo"
  exit
fi

# Cleanup if necessary
if [ -d "data" ]; then
  docker compose down --remove-orphans
  rm -Rf data
fi

# Prepare db directory
mkdir -p data/sandboxdb/grafana
mkdir -p data/sandboxdb/prometheus
mkdir -p data/sandboxdb/dashboard-1
mkdir -p data/sandboxdb/dashboard-2
mkdir -p data/sandboxdb/dashboard-3
mkdir -p data/sandboxdb/dashboard-4
mkdir -p data/sandboxdb/database_legacy
mkdir -p data/sandboxdb/database_chrysalis
mkdir -p data/sandboxdb/wasp
mkdir -p data/sandboxdb/hornet
mkdir -p data/sandboxdb/hornet-2
mkdir -p data/sandboxdb/hornet-3
mkdir -p data/sandboxdb/hornet-4
mkdir -p data/sandboxdb/state
mkdir -p data/sandboxdb/indexer
mkdir -p data/sandboxdb/participation
mkdir -p data/snapshots/hornet
mkdir -p data/snapshots/hornet-2
mkdir -p data/snapshots/hornet-3
mkdir -p data/snapshots/hornet-4
mkdir -p data/sandboxdb/evm-toolkit
mkdir -p data/sandboxdb/wasp-cli

if [ ! -f data/sandboxdb/wasp/users.json ]; then
  echo "{}" >> data/sandboxdb/wasp/users.json
fi

cp assets/wasp-cli/config.json data/sandboxdb/wasp-cli/config.json

if [[ "$OSTYPE" != "darwin"* ]]; then
  chown -R 65532:65532 data
fi

# Create snapshot
docker compose run create-snapshots

# Bootstrap network (create hornet database, create genesis milestone, create coo state)
docker compose run bootstrap-network

# Duplicate snapshot
cp -R data/snapshots/hornet/* data/snapshots/hornet-2
cp -R data/snapshots/hornet/* data/snapshots/hornet-3
cp -R data/snapshots/hornet/* data/snapshots/hornet-4

if [[ "$OSTYPE" != "darwin"* ]]; then
  chown -R 65532:65532 data
fi
