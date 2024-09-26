from locust import HttpUser, task, between
from iota_sdk import Utils, OutputId, ClientOptions, StrongholdSecretManager, Wallet, CoinType,SyncOptions, AliasSyncOptions, utf8_to_hex, CreateAliasOutputParams
import time
import os
import shutil
import json

# Helper functions
def byte_string_to_json(byte_string: bytes) -> dict:
    try:
        # Decode the byte string to a normal string
        decoded_string = byte_string.decode('utf-8')
        # Parse the decoded string as JSON
        json_data = json.loads(decoded_string)
        return json_data
    except (UnicodeDecodeError, json.JSONDecodeError) as e:
        print(f"Error decoding or parsing JSON: {e}")
        return None
    
# Set static variables
TEMP_DIR="./tmp"
STRONGHOLD_PASSWORD="password"
NODE_URL = 'http://localhost:14265'
FAUCET_URL = 'http://localhost:8091/api/enqueue'

wallet_db_path=os.path.join(TEMP_DIR, "walletdb_resolve")
stronghold_snapshot_path= os.path.join(TEMP_DIR, "resolve.stronghold")

# Create a Strongold to store secret data
secret_manager = StrongholdSecretManager( stronghold_snapshot_path, STRONGHOLD_PASSWORD)
client_options = ClientOptions(nodes=[NODE_URL])
# Create a Wallet and an account stored in the wallet
wallet = Wallet(wallet_db_path, client_options, CoinType.SHIMMER, secret_manager)
mnemonic = Utils.generate_mnemonic()
wallet.store_mnemonic(mnemonic)
account = wallet.create_account('Alice')
account.set_default_sync_options( SyncOptions(sync_only_most_basic_outputs=True))
# Generate an address to recieve tokens and request tokens from faucet
address = account.generate_ed25519_addresses(1)[0]
faucet_response = wallet.get_client().request_funds_from_faucet( FAUCET_URL, address.address)

print('MetaData', account.get_metadata())
print('Address:', address.address)
# print(faucet_response)

# Verify we have received tokens from faucet
sync_options = SyncOptions(alias=AliasSyncOptions(basic_outputs=True))
base_token_balance = account.sync(sync_options).baseCoin
while base_token_balance.total == '0':
    base_token_balance = account.sync(sync_options).baseCoin
    print(f'Balance: Total: {base_token_balance.total} Available: {base_token_balance.available}' )
    time.sleep(1)

# Create parameters that will be stored in the alias output on the tangle 
alias_params = CreateAliasOutputParams(
    address=address.address,
    metadata=utf8_to_hex("Test to resolve alias metadata"),
    stateMetadata=utf8_to_hex("Test to resolve alias state metadata")
    )

# Create and send the alias output 
transaction = account.create_alias_output( alias_params, None)
# We have to wait until the transaction have been included (referenced by a milestone)
# Our funds will be unavailable until this is complete to avoid double spending. 
block_id = account.retry_transaction_until_included(transaction.transactionId)

# print(f'Block sent: {transaction}')
print(f'Transaction Id: {transaction.transactionId}')
print(f'Block Id: {transaction.blockId}')
# print(f'Block: {block_id}')

# block_data = wallet.get_client().get_block_data(block_id)
# print(f'BlockData: {json.dumps(block_data.as_dict(), indent=4)}')

index = 0 # Index is always 0
original_output_id = Utils.compute_output_id(transaction.transactionId, index)
print(f'original_output_id: {original_output_id}')

original_alias_id = Utils.compute_alias_id(original_output_id)
print(f'original_alias_id: {original_alias_id}')

original_alias_id_bech32 = Utils.alias_id_to_bech32(original_alias_id, wallet.get_client().get_bech32_hrp())
print(f'original_alias_id_bech32: {original_alias_id_bech32}')

print(f'\nAlias output available at:\nhttp://localhost:8011/dashboard/explorer/address/{original_alias_id_bech32}\n')

# Cleanup 
wallet.destroy()
if os.path.isdir(TEMP_DIR):
    shutil.rmtree(TEMP_DIR)
    print(f"Folder '{TEMP_DIR}' has been deleted.")


class ResolveAliasUser(HttpUser):
    wait_time = between(1, 3)       
    host = NODE_URL

    # @task
    # def resolve_alias_task(self):
    #     # Get output_id from on alias_id
    #     r = self.client.get(f"/api/indexer/v1/outputs/alias/{original_alias_id}")
    #     json_data = byte_string_to_json(r.content)
    #     output_id = json_data['items'][0]
    #     print(f'Received output_id: {output_id}')

    #     # Get the output data. This would have contained the DID document
    #     r = self.client.get(f"/api/core/v2/outputs/{output_id}")
    #     json_data = byte_string_to_json(r.content)
    #     print(f'Received output: {json.dumps(json_data, indent=4)}')

    @task
    def get_alias_task(self):
        # https://wiki.iota.org/apis/indexer/returns-the-output-id-of-the-current-unspent-alias-output-for-alias-id/
        self.client.get(f"/api/indexer/v1/outputs/alias/{original_alias_id}")

    @task
    def get_output_task(self):
        # https://wiki.iota.org/apis/core/v2/find-an-output-by-its-identifier/
        self.client.get(f"/api/core/v2/outputs/{original_output_id}")





