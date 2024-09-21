from locust import User, TaskSet, task, between, events
from iota_sdk  import Client
import time
import threading

class IotaTaskSet(TaskSet):
    def on_start(self):
        # Initialize the IOTA client
        self.client = Client(nodes=['http://localhost:14265'])
        self.message_count = 0
        self.total_bytes_sent = 0

    @task
    def send_transaction(self):
        start_time = time.time()
        index = f"TestIndex-{self.user.environment.runner.start_time}"
        data = f"TestData-{self.message_count}".encode('utf-8')
        self.message_count += 1

        try:
            # Send a message to the Tangle
            message = self.client.message(index=index, data=data)
            message_id = message['message_id']

            # Calculate latency
            latency = (time.time() - start_time) * 1000  # in milliseconds

            # Fire success event for latency
            self.user.environment.events.request_success.fire(
                request_type="Transaction",
                name="send_transaction",
                response_time=latency,
                response_length=len(data),
            )

            # Update total bytes sent
            self.total_bytes_sent += len(data)

            # Start a thread to track confirmation
            threading.Thread(target=self.track_confirmation, args=(message_id,)).start()

        except Exception as e:
            latency = (time.time() - start_time) * 1000
            self.user.environment.events.request_failure.fire(
                request_type="Transaction",
                name="send_transaction",
                response_time=latency,
                response_length=0,
                exception=e,
            )

    def track_confirmation(self, message_id):
        start_time = time.time()
        confirmed = False
        while not confirmed:
            time.sleep(1)  # Poll every second
            try:
                # Get message metadata
                metadata = self.client.get_message().metadata(message_id)
                if metadata['referenced_by_milestone_index'] is not None:
                    confirmed = True
                    confirmation_time = (time.time() - start_time) * 1000  # in milliseconds
                    # Fire success event for confirmation
                    self.user.environment.events.request_success.fire(
                        request_type="Confirmation",
                        name="message_confirmation",
                        response_time=confirmation_time,
                        response_length=0,
                    )
            except Exception as e:
                pass  # Handle exceptions if necessary

class IotaUser(User):
    tasks = [IotaTaskSet]
    wait_time = between(0.1, 0.5)  # Adjust as needed
