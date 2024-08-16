// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use iota_sdk::client::api::GetAddressesOptions;
use iota_sdk::client::node_api::indexer::query_parameters::QueryParameter;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::crypto::keys::bip39;
use iota_sdk::types::block::address::Address;
use iota_sdk::types::block::address::Bech32Address;
use iota_sdk::types::block::address::Hrp;
use rand::distributions::DistString;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use strum::EnumIter;

pub type Measurement = HashMap<Action, Vec<std::time::Duration>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum IotaTangleNetwork {
    Localhost,
    ShimmerTestnet,
    IotaTestnet2_0,
}

impl IotaTangleNetwork {
    pub fn name(&self) -> &'static str {
        match self {
            IotaTangleNetwork::Localhost => "Localhost",
            IotaTangleNetwork::ShimmerTestnet => "Shimmer testnet",
            IotaTangleNetwork::IotaTestnet2_0 => "IOTA 2.0 testnet",
        }
    }

    pub fn api_endpoint(&self) -> &'static str {
        match self {
            IotaTangleNetwork::Localhost => "http://localhost",
            IotaTangleNetwork::ShimmerTestnet => "https://api.testnet.shimmer.network",
            IotaTangleNetwork::IotaTestnet2_0 => "https://api.nova-testnet.iotaledger.net/",
        }
    }

    pub fn faucet_endpoint(&self) -> &'static str {
        match self {
            IotaTangleNetwork::Localhost => "http://localhost/faucet/api/enqueue",
            IotaTangleNetwork::ShimmerTestnet => {
                "https://faucet.testnet.shimmer.network/api/enqueue"
            }
            IotaTangleNetwork::IotaTestnet2_0 => {
                "https://faucet.nova-testnet.iotaledger.net//api/enqueue"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, EnumIter)]
pub enum Action {
    CreateDid,
    DeleteDid,
    UpdateDid,
    DeactivateDid,
    ReactivateDid,
    ResolveDid,
}

impl Action {
    pub fn name(&self) -> &'static str {
        match self {
            Action::CreateDid => "Create DID",
            Action::DeleteDid => "Delete DID",
            Action::UpdateDid => "Update DID",
            Action::DeactivateDid => "Deactivate DID",
            Action::ReactivateDid => "Reactivate DID",
            Action::ResolveDid => "Resolve DID",
        }
    }
}

/// Generates an address from the given [`SecretManager`] and adds funds from the faucet.
pub async fn get_address_with_funds(
    client: &Client,
    stronghold: &SecretManager,
    faucet_endpoint: &str,
) -> anyhow::Result<Address> {
    let address: Bech32Address = get_address(client, stronghold).await?;

    request_faucet_funds(client, address, faucet_endpoint)
        .await
        .context("failed to request faucet funds")?;

    Ok(*address)
}

/// Initializes the [`SecretManager`] with a new mnemonic, if necessary,
/// and generates an address from the given [`SecretManager`].
pub async fn get_address(
    client: &Client,
    secret_manager: &SecretManager,
) -> anyhow::Result<Bech32Address> {
    let random: [u8; 32] = rand::random();
    let mnemonic = bip39::wordlist::encode(random.as_ref(), &bip39::wordlist::ENGLISH)
        .map_err(|err| anyhow::anyhow!(format!("{err:?}")))?;

    if let SecretManager::Stronghold(ref stronghold) = secret_manager {
        match stronghold.store_mnemonic(mnemonic).await {
            Ok(()) => (),
            Err(iota_sdk::client::stronghold::Error::MnemonicAlreadyStored) => (),
            Err(err) => anyhow::bail!(err),
        }
    } else {
        anyhow::bail!("expected a `StrongholdSecretManager`");
    }

    let bech32_hrp: Hrp = client.get_bech32_hrp().await?;
    let address: Bech32Address = secret_manager
        .generate_ed25519_addresses(
            GetAddressesOptions::default()
                .with_range(0..1)
                .with_bech32_hrp(bech32_hrp),
        )
        .await?[0];

    Ok(address)
}

/// Requests funds from the faucet for the given `address`.
async fn request_faucet_funds(
    client: &Client,
    address: Bech32Address,
    faucet_endpoint: &str,
) -> anyhow::Result<()> {
    iota_sdk::client::request_funds_from_faucet(faucet_endpoint, &address).await?;

    tokio::time::timeout(std::time::Duration::from_secs(45), async {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            let balance = get_address_balance(client, &address)
                .await
                .context("failed to get address balance")?;
            if balance > 0 {
                break;
            }
        }
        Ok::<(), anyhow::Error>(())
    })
    .await
    .context("maximum timeout exceeded")??;

    Ok(())
}

/// Returns the balance of the given Bech32-encoded `address`.
async fn get_address_balance(client: &Client, address: &Bech32Address) -> anyhow::Result<u64> {
    let output_ids = client
        .basic_output_ids(vec![
            QueryParameter::Address(address.to_owned()),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;

    let outputs = client.get_outputs(&output_ids).await?;

    let mut total_amount = 0;
    for output_response in outputs {
        total_amount += output_response.output().amount();
    }

    Ok(total_amount)
}

/// Creates a random stronghold path in the temporary directory, whose exact location is OS-dependent.
pub fn random_stronghold_path() -> PathBuf {
    let mut file = std::env::current_dir().unwrap();
    file.push("test_strongholds");
    file.push(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32));
    file.set_extension("stronghold");
    file.to_owned()
}
