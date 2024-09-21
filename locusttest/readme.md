# Locust

```shell

pip install iota-sdk==1.1.4


locust -f locustfile.py --headless -u 50 -r 5 --run-time 5m

locust -f main.py -u 1 -r 1 --run-time 1m

-u 50: Simulate 50 users.
-r 5: Spawn rate (users per second).
--run-time 5m: Run the test for 5 minutes.
Omit --headless to use the web UI at http://localhost:8089.

```
