# Readme

# Running using Docker desktop on Windows using WSL 2 as backend

```shell
# Bootstrap
sudo ./bootstrap

# Bootstrap network (create hornet database, create genesis milestone, create coo state)
docker compose run bootstrap-network

# Restart spammer when env file is updated
docker compose up --force-recreate -d inx-spammer
```

- grafana: http://localhost:3000
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
