from locust import User, task, between, events
from iota_sdk import Client, utf8_to_hex
import time
import threading

class IotaUser(User):
    wait_time = between(1, 2)  # Adjust the wait time as needed

    def on_start(self):
        # Initialize the IOTA client with the Testnet node
        self.client = Client(nodes=['http://localhost:14265'])

    @task
    def send_transaction(self):
        # block = self.client.build_and_post_block(tag=utf8_to_hex('hello'), data=utf8_to_hex('hello'))
        start_time = time.time()

        node_info = self.client.get_info()

        total_time = int((time.time() - start_time) * 1000)

        self.environment.events.request.fire(
                request_type="IOTA",
                name="send_transaction",
                response_time=total_time,
                response_length=0,
                exception=None,
                context={},  # Optional context
            )

        # print(f'{node_info}')
        



