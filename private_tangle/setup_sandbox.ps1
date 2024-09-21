# iota-sandbox __VERSION__
# https://github.com/iotaledger/iota-sandbox

# Cleanup if necessary
if (Test-Path "data") {
    docker-compose down --remove-orphans
    Remove-Item -Recurse -Force "data"
}

# Prepare db directory
$dirs = @(
    "data\sandboxdb\grafana",
    "data\sandboxdb\prometheus",
    "data\sandboxdb\dashboard-1",
    "data\sandboxdb\dashboard-2",
    "data\sandboxdb\dashboard-3",
    "data\sandboxdb\dashboard-4",
    "data\sandboxdb\hornet",
    "data\sandboxdb\hornet-2",
    "data\sandboxdb\hornet-3",
    "data\sandboxdb\hornet-4",
    "data\sandboxdb\state",
    "data\sandboxdb\indexer",
    "data\sandboxdb\participation",
    "data\snapshots\hornet",
    "data\snapshots\hornet-2",
    "data\snapshots\hornet-3",
    "data\snapshots\hornet-4"
)

foreach ($dir in $dirs) {
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
}

# Create snapshot
docker-compose run create-snapshots

# Bootstrap network
docker-compose run bootstrap-network

# Duplicate snapshot
Copy-Item -Recurse -Force "data\snapshots\hornet\*" "data\snapshots\hornet-2\"
Copy-Item -Recurse -Force "data\snapshots\hornet\*" "data\snapshots\hornet-3\"
Copy-Item -Recurse -Force "data\snapshots\hornet\*" "data\snapshots\hornet-4\"
