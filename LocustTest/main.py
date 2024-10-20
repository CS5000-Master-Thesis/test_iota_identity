from locust import User, task, between, constant, events, FastHttpUser
from iota_sdk import Client, Utils, ClientOptions, StrongholdSecretManager, Wallet, CoinType,SyncOptions, AliasSyncOptions, LedgerInclusionState, WalletEventType, utf8_to_hex
from itertools import count
import time
import os
import shutil
import gevent
import json

TEMP_DIR="./tmp"
STRONGHOLD_PASSWORD="password"
NODE_URL = 'http://localhost:14265'
FAUCET_URL = 'http://localhost:8091/api/enqueue'


########################################################################################

# class WebsiteUser(FastHttpUser):
#     """
#     User class that does requests to the locust web server running on localhost,
#     using the fast HTTP client
#     """

#     host = "http://localhost:14265"

#     @task
#     def stats(self):
#         self.client.get("/api/core/v2/info")


########################################################################################

# class BlockIotaUser(User):
#     wait_time = between(1, 3)

#     def on_start(self):
#         self.client = Client(nodes=[NODE_URL])

#     @task
#     def build_and_post_block(self):
#         start_time = time.time()
#         exep = None

#         try:
#             block = self.client.get_info()
#         except Exception as e:
#             exep = Exception(f"Error: {e}")

#         # print("Publishing block")

#         total_time = int((time.time() - start_time) * 1000)

#         events.request.fire(
#                 request_type="IOTA",
#                 name="post_block",
#                 response_time=total_time,
#                 response_length=0,
#                 exception=exep,
#                 context={},
#             )


########################################################################################


class BlockIotaUser(User):
    wait_time = between(1, 3)

    def on_start(self):
        self.client = Client(nodes=[NODE_URL])

    @task
    def build_and_post_block(self):
        start_time = time.time()
        exep = None

        try:
            block = self.client.build_and_post_block(tag=utf8_to_hex('hello'), data=utf8_to_hex('hello'))
        except Exception as e:
            exep = Exception(f"Error: {e}")

        # print("Publishing block")

        total_time = int((time.time() - start_time) * 1000)

        events.request.fire(
                request_type="IOTA",
                name="post_block",
                response_time=total_time,
                response_length=0,
                exception=exep,
                context={},
            )



########################################################################################

# class AliasIotaUser(User):
#     wait_time = between(5, 10)  # Adjust the wait time as needed
#     user_counter = count(1)

#     def on_stop(self):
#         self.wallet.destroy()
#         time.sleep(1)

#         if os.path.isdir(TEMP_DIR):
#             shutil.rmtree(TEMP_DIR)
#             print(f"Folder '{TEMP_DIR}' has been deleted.")

#     def on_start(self):
#         self.user_id = next(AliasIotaUser.user_counter)

#         wallet_db_path=os.path.join(TEMP_DIR, str(self.user_id), "example-walletdb")
#         stronghold_snapshot_path= os.path.join(TEMP_DIR, str(self.user_id), "example.stronghold")

#         mnemonic = Utils.generate_mnemonic()
#         client_options = ClientOptions(nodes=[NODE_URL])

#         secret_manager = StrongholdSecretManager( stronghold_snapshot_path, STRONGHOLD_PASSWORD)
#         self.wallet = Wallet(wallet_db_path, client_options, CoinType.SHIMMER, secret_manager)
#         self.wallet.store_mnemonic(mnemonic)

#         self.account = self.wallet.create_account('Alice')
#         self.account.set_default_sync_options( SyncOptions(sync_only_most_basic_outputs=True))
#         address = self.account.generate_ed25519_addresses(1)[0]
        
#         print('Address:', self.user_id, address.address)
#         print('MetaData', self.account.get_metadata())

#         balance = self.account.sync(SyncOptions(sync_only_most_basic_outputs=True))
#         print('Balance', self.user_id, balance.baseCoin.total, balance.baseCoin.available )

#         faucet_response = self.wallet.get_client().request_funds_from_faucet( FAUCET_URL, address.address)
#         print(faucet_response)

#         sync_options = SyncOptions(alias=AliasSyncOptions(basic_outputs=True))
#         base_token_balance = self.account.sync(sync_options).baseCoin

#         while base_token_balance.total == '0':
#             base_token_balance = self.account.sync(sync_options).baseCoin
#             time.sleep(1)
#             print('Balance', self.user_id, base_token_balance.total, base_token_balance.available )


#         # balance = self.account.sync(SyncOptions(sync_only_most_basic_outputs=True))
#         # print('Balance', balance)
        
#         # transaction = self.account.create_alias_output(None, None)
#         # print(f'Block sent: {transaction.blockId}')

#         self.test = False
        
#     @task
#     def create_alias_output(self):
        
#         if self.test:
#             print('This should never happen!')

#         self.test = True

#         start_time = time.time()



#         # print('Start', self.user_id, time.ctime(start_time))

#         balance = self.account.sync(SyncOptions(sync_only_most_basic_outputs=True))
#         print('Balance', self.user_id, balance.baseCoin.total, balance.baseCoin.available )

    
#         transaction = self.account.create_alias_output(None, None)
#         print(f'{self.user_id} Block sent: {transaction.blockId}')
#         # block_id = self.account.retry_transaction_until_included(transaction.transactionId)
#         # print(f'Block: {block_id}')

#         # Polling logic for checking if the transaction is included
#         transaction_id = transaction.transactionId
#         max_retries = 100  # Number of times to check before giving up
#         retry_interval = 0.1  # How long to wait between retries (in seconds)
#         retries = 0
#         included = False

#         while retries < max_retries:
#             try:
#                 # Sync the account or use a method to check if the transaction is included
#                 # get_transaction
#                 # get_included_block_metadata
#                 block_metadata = self.wallet.get_client().get_included_block_metadata(transaction_id)
#                 if block_metadata.ledgerInclusionState == LedgerInclusionState.included:
#                     included = True
#                     print(f'Transaction {transaction_id} is included in the block.{retries}')
#                     break
#                 # else:
#                     # print(f'Transaction {transaction_id} not yet included, retrying... ({retries + 1}/{max_retries})')
#             except Exception as e:
#                 e
#                 # print(f'Error checking transaction status: {e}')

#             retries += 1
#             gevent.sleep(retry_interval)  # Yield control to allow other tasks to run during wait

#         exep = None
#         if not included:
#             exep = Exception(f"Transaction {transaction_id} was not included after {max_retries} retries.")

#         # print('End', self.user_id, time.ctime(time.time()))
#         total_time = int((time.time() - start_time) * 1000)

#         self.environment.events.request.fire(
#                 request_type="IOTA",
#                 name="create_alias_output",
#                 response_time=total_time,
#                 response_length=0,
#                 exception= exep, #None,
#                 context={},
#             )
#         self.test = False
        

        



