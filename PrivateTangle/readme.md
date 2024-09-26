# Setup Private tangle

Based on [IOTA Hornet](https://github.com/iotaledger/hornet) and [IOTA Sandbox](https://github.com/iotaledger/iota-sandbox)

### Prerequisites

- Install [Docker](https://docs.docker.com/engine/install/)

#### Configure the tangle

- Remote PoW can be set in [.env](.env) and modify ENABLE_REMOTE_POW
  > NB: Changing the environments require a restart of the private tangle.
- minPoWScore can be edited in [protocol_parameters.json](./protocol_parameters.json)
  > NB: Changing the protocol_parameters.json require the network the be bootstraped again.

## Start Private Tangle

Make sure docker is running, then run the appropriate commands for your operating system.

#### Windows Powershell

```shell
# Bootstrap
./bootstrap.ps1

# Start the containers
docker compose up -d
```

#### Linux

```shell
# Bootstrap
sudo ./bootstrap

# Start the containers
sudo docker compose up -d
```

#### Useful commands

```shell
# Stop the containers
docker compose down

# Restart spammer when env file is updated
docker compose up --force-recreate -d inx-spammer
```

## Test API Requests

[api_requests.rest](api_requests.rest) is an easy way of testing API requests.

# Useful links

- grafana: http://localhost:3000 (username: admin, password: admin)
- prometheus: http://localhost:9090
- explorer: http://localhost:8085
- inx-faucet:
  - Faucet: http://localhost:8091
  - pprof: http://localhost:6024/debug/pprof
- hornet:
  - API: http://localhost:14265
  - External Peering: 15611/tcp
  - Dashboard: http://localhost:8011 (username: admin, password: admin)
  - Prometheus: http://localhost:9311/metrics
  - pprof: http://localhost:6011/debug/pprof
  - inx: localhost:9011
- Hornet-2:
  - API: http://localhost:14266
  - External Peering: 15612/tcp
  - Dashboard: http://localhost:8012 (username: admin, password: admin)
  - Prometheus: http://localhost:9312/metrics
  - pprof: http://localhost:6012/debug/pprof
  - inx: localhost:9012
- Hornet-3:
  - API: http://localhost:14267
  - External Peering: 15613/tcp
  - Dashboard: http://localhost:8013 (username: admin, password: admin)
  - Prometheus: http://localhost:9313/metrics
  - pprof: http://localhost:6013/debug/pprof
  - inx: localhost:9013
- Hornet-4:
  - API: http://localhost:14268
  - External Peering: 15614/tcp
  - Dashboard: http://localhost:8014 (username: admin, password: admin)
  - Prometheus: http://localhost:9314/metrics
  - pprof: http://localhost:6014/debug/pprof
  - inx: localhost:9014
- inx-coordinator:
  - pprof: http://localhost:6021/debug/pprof
- inx-indexer:
  - pprof: http://localhost:6022/debug/pprof
  - Prometheus: http://localhost:9322/metrics
- inx-mqtt:
  - pprof: http://localhost:6023/debug/pprof
  - Prometheus: http://localhost:9323/metrics
- inx-participation:
  - pprof: http://localhost:6025/debug/pprof
- inx-spammer:
  - pprof: http://localhost:6026/debug/pprof
  - Prometheus: http://localhost:9326/metrics
- inx-poi:
  - pprof: http://localhost:6027/debug/pprof
- inx-dashboard-1:
  - pprof: http://localhost:6031/debug/pprof
  - Prometheus: http://localhost:9331/metrics
- inx-dashboard-2:
  - pprof: http://localhost:6032/debug/pprof
  - Prometheus: http://localhost:9332/metrics
- inx-dashboard-3:
  - pprof: http://localhost:6033/debug/pprof
  - Prometheus: http://localhost:9333/metrics
- inx-dashboard-4:
  - pprof: http://localhost:6034/debug/pprof
  - Prometheus: http://localhost:9334/metrics
