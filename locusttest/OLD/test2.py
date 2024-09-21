from locust import User, task, between, events
from iota_sdk import Client, SecretManager, MnemonicSecretManager, Wallet
import time

class IotaUser(User):
    wait_time = between(1, 2)  # Adjust the wait time as needed

    def on_start(self):
        MNEMONIC="endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river"


        # Initialize the IOTA client with the Testnet node
        self.client = Client(nodes=['https://api.testnet.shimmer.network'])

        # Initialize the secret manager with your Testnet mnemonic
        # Replace 'YOUR_TESTNET_MNEMONIC' with your actual mnemonic
        self.secret_manager = SecretManager(MnemonicSecretManager(MNEMONIC))

        # Create or get a wallet instance
        self.wallet = Wallet(
            client_options={'nodes': ['https://api.testnet.shimmer.network']},
            secret_manager=self.secret_manager
        )

        # Get or create an account
        accounts = self.wallet.get_accounts()
        if accounts:
            self.account = accounts[0]
        else:
            self.account = self.wallet.create_account('test_account')

        # Sync the account with the Tangle
        self.account.sync()

    @task
    def send_transaction(self):
        start_time = time.time()
        try:
            # Generate a new address to send funds to
            address = self.account.generate_addresses(1)[0]['address']

            # Prepare and send a zero-value transaction
            transfer = {
                'address': address,
                'amount': '0',  # Zero-value transaction
            }
            transaction = self.account.send_transfer([transfer])

            # Wait for confirmation
            confirmation_time = self.wait_for_confirmation(transaction['transactionId'])

            # Record successful transaction
            total_time = int((confirmation_time - start_time) * 1000)  # Total time in milliseconds

            # Fire custom event for Locust
            self.environment.events.request.fire(
                request_type="IOTA",
                name="send_transaction",
                response_time=total_time,
                response_length=0,
                exception=None,
                context={},  # Optional context
            )
        except Exception as e:
            # Record failed transaction
            total_time = int((time.time() - start_time) * 1000)
            self.environment.events.request.fire(
                request_type="IOTA",
                name="send_transaction",
                response_time=total_time,
                response_length=0,
                exception=e,
                context={},  # Optional context
            )

    def wait_for_confirmation(self, transaction_id, timeout=120):
        start_time = time.time()
        while time.time() - start_time < timeout:
            # Get the transaction status
            inclusion_state = self.client.get_inclusion_state([transaction_id])[0]
            if inclusion_state == 'included':
                return time.time()
            time.sleep(2)  # Wait before checking again
        raise Exception('Transaction not confirmed within timeout')
