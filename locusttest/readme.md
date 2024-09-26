# Locust

### Prerequisites

Install [Python](https://www.python.org/downloads/) then install the packages below.

```shell
# Install packages
pip install iota-sdk==1.1.4
pip install locust==2.31.6
```

### Run test

> Make sure the private tangle is running before starting the test.

```shell
# Run test_alias_resolve test
locust -f test_alias_resolve.py
```

- Web UI: http://localhost:8089

## Information

```shell
locust -f locustfile.py --headless -u 50 -r 5 --run-time 5m

# Parameters
-u 50          # Simulate 50 users.
-r 5           # Spawn rate (users per second).
--run-time 5m  # Run the test for 5 minutes.
--headless     # to run without the web UI
```

If you run with too many users and the hornet node can't recover restart both docker and WSL

```shell
# 1. Stop the private tangle
# 2. Stop the docker process
# 3. Stop WSL (wsl --shutdown)
# 4. Restart docker
# 5. Bootstrap the tangle again
# 5. Start the tangle
```
