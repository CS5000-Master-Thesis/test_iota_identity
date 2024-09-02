use std::collections::HashMap;

use crate::utils::{get_address_with_funds, random_stronghold_path};
use anyhow::anyhow;
use identity_iota::{
    core::Timestamp,
    did::{DIDUrl, DID},
    iota::{IotaClientExt, IotaDID, IotaDocument, IotaIdentityClientExt, NetworkName},
    prelude::Resolver,
    storage::{JwkDocumentExt, JwkMemStore, Storage},
    verification::{jws::JwsAlgorithm, MethodRelationship, MethodScope},
};
use identity_stronghold::StrongholdStorage;
use iota_sdk::{
    client::{secret::stronghold::StrongholdSecretManager, Client, Password},
    types::block::{
        address::Address,
        output::{AliasOutput, AliasOutputBuilder, RentStructure},
    },
};
use log::{debug, info};

struct DIDInformation {
    did: IotaDID,
    fragment: String,
    document: Option<IotaDocument>,
}

pub struct DIDManager {
    client: Client,
    stronghold_storage: StrongholdStorage,
    address: Address,
    network_name: NetworkName,
    resolver: Resolver<IotaDocument>,
    storage: Storage<StrongholdStorage, StrongholdStorage>,
    did_map: HashMap<usize, DIDInformation>,
}

impl DIDManager {
    pub async fn new(api_endpoint: &str, faucet_endpoint: &str) -> anyhow::Result<Self> {
        info!("Creating new DIDManager");

        // Stronghold snapshot path.
        let path = random_stronghold_path();

        info!("1111");

        // Stronghold password.
        let password = Password::from("secure_password".to_owned());

        // Create a new client to interact with the IOTA ledger.
        let client: Client = Client::builder()
            .with_primary_node(api_endpoint, None)?
            .finish()
            .await?;

        info!("2222");

        let stronghold = StrongholdSecretManager::builder()
            .password(password.clone())
            .build(path.clone())?;

        // Create a `StrongholdStorage`.
        // `StrongholdStorage` creates internally a `SecretManager` that can be
        // referenced to avoid creating multiple instances around the same stronghold snapshot.
        let stronghold_storage = StrongholdStorage::new(stronghold);

        info!("33333");

        // Create a DID document.
        let address: Address = get_address_with_funds(
            &client,
            stronghold_storage.as_secret_manager(),
            faucet_endpoint,
        )
        .await?;

        let network_name: NetworkName = client.network_name().await?;

        info!("4444 {}", network_name);

        let storage: Storage<StrongholdStorage, StrongholdStorage> =
            Storage::new(stronghold_storage.clone(), stronghold_storage.clone());

        // Create resolver
        let mut resolver = Resolver::<IotaDocument>::new();
        resolver.attach_iota_handler(client.clone());

        info!("555");

        Ok(Self {
            client: client,
            stronghold_storage: stronghold_storage,
            address: address,
            network_name: network_name,
            resolver: resolver,
            storage: storage,
            did_map: HashMap::new(),
        })
    }

    pub async fn create_did(&mut self, index: usize) -> anyhow::Result<()> {
        info!("Creating new DID");

        // Create a new DID document with a placeholder DID.
        // The DID will be derived from the Alias Id of the Alias Output after publishing.
        let mut document = IotaDocument::new(&self.network_name);

        // Generates a verification method. This will store the key-id as well as the private key
        // in the stronghold file.
        let fragment = document
            .generate_method(
                &self.storage,
                JwkMemStore::ED25519_KEY_TYPE,
                JwsAlgorithm::EdDSA,
                None,
                MethodScope::VerificationMethod,
            )
            .await?;

        // Construct an Alias Output containing the DID document, with the wallet address
        // set as both the state controller and governor.
        let alias_output: AliasOutput = self
            .client
            .new_did_output(self.address, document, None)
            .await?;

        // Publish the Alias Output and get the published DID document.
        let document: IotaDocument = self
            .client
            .publish_did_output(self.stronghold_storage.as_secret_manager(), alias_output)
            .await?;

        debug!("DID created: {document:#}");

        self.did_map.insert(
            index,
            DIDInformation {
                did: document.id().clone(),
                fragment: fragment,
                document: None,
            },
        );

        Ok(())
    }

    pub async fn update_did(&mut self, index: usize) -> anyhow::Result<()> {
        info!("Updating new DID");

        match self.did_map.get_mut(&index) {
            Some(did_info) => {
                // Resolve the latest state of the document.
                let mut document: IotaDocument = self.resolver.resolve(&did_info.did).await?;

                // Insert a new Ed25519 verification method in the DID document.
                let new_fragment: String = document
                    .generate_method(
                        &self.storage,
                        JwkMemStore::ED25519_KEY_TYPE,
                        JwsAlgorithm::EdDSA,
                        None,
                        MethodScope::VerificationMethod,
                    )
                    .await?;

                // Attach a new method relationship to the inserted method.
                document.attach_method_relationship(
                    &document.id().to_url().join(format!("#{new_fragment}"))?,
                    MethodRelationship::Authentication,
                )?;

                document.metadata.updated = Some(Timestamp::now_utc());

                // Remove a verification method.
                let original_method: DIDUrl = document
                    .resolve_method(did_info.fragment.as_str(), None)
                    .unwrap()
                    .id()
                    .clone();
                document
                    .purge_method(&self.storage, &original_method)
                    .await
                    .unwrap();

                // Resolve the latest output and update it with the given document.
                let alias_output: AliasOutput =
                    self.client.update_did_output(document.clone()).await?;

                // Because the size of the DID document increased, we have to increase the allocated storage deposit.
                // This increases the deposit amount to the new minimum.
                let rent_structure: RentStructure = self.client.get_rent_structure().await?;
                let alias_output: AliasOutput = AliasOutputBuilder::from(&alias_output)
                    .with_minimum_storage_deposit(rent_structure)
                    .finish()?;

                // Publish the updated Alias Output.
                let updated: IotaDocument = self
                    .client
                    .publish_did_output(self.stronghold_storage.as_secret_manager(), alias_output)
                    .await?;
                debug!("Updated DID document: {updated:#}");

                did_info.fragment = new_fragment;
            }
            None => return Err(anyhow!("No object found at index {}", index)),
        }

        Ok(())
    }

    ///
    ///
    ///
    pub async fn resolve_did(&self, index: usize) -> anyhow::Result<()> {
        info!("Resolving DID");

        match self.did_map.get(&index) {
            Some(did_info) => {
                let resolved_document: IotaDocument = self.resolver.resolve(&did_info.did).await?;
                assert_eq!(did_info.did, *resolved_document.id());

                debug!("The did resolved is: {}", did_info.did);
            }
            None => return Err(anyhow!("No object found at index {}", index)),
        }
        Ok(())
    }

    ///
    ///
    ///
    pub async fn deactivate_did(&mut self, index: usize) -> anyhow::Result<()> {
        info!("Deactivating DID");

        match self.did_map.get_mut(&index) {
            Some(did_info) => {
                let resolved_document: IotaDocument = self.resolver.resolve(&did_info.did).await?;

                did_info.document = Some(resolved_document);

                // Deactivate the DID by publishing an empty document.
                // This process can be reversed since the Alias Output is not destroyed.
                // Deactivation may only be performed by the state controller of the Alias Output.
                let deactivated_output: AliasOutput =
                    self.client.deactivate_did_output(&did_info.did).await?;

                // Optional: reduce and reclaim the storage deposit, sending the tokens to the state controller.
                let rent_structure = self.client.get_rent_structure().await?;
                let deactivated_output = AliasOutputBuilder::from(&deactivated_output)
                    .with_minimum_storage_deposit(rent_structure)
                    .finish()?;

                // Publish the deactivated DID document.
                let _ = self
                    .client
                    .publish_did_output(
                        self.stronghold_storage.as_secret_manager(),
                        deactivated_output,
                    )
                    .await?;

                // Resolving a deactivated DID returns an empty DID document
                // with its `deactivated` metadata field set to `true`.
                let deactivated: IotaDocument = self.resolver.resolve(&did_info.did).await?;
                assert_eq!(deactivated.metadata.deactivated, Some(true));
                debug!("Deactivated DID document: {deactivated:#}");
            }
            None => return Err(anyhow!("No object found at index {}", index)),
        }
        Ok(())
    }

    ///
    ///
    ///
    pub async fn reactivate_did(&mut self, index: usize) -> anyhow::Result<()> {
        info!("Reactivating DID");

        match self.did_map.get_mut(&index) {
            Some(did_info) => {
                match &did_info.document {
                    Some(document) => {
                        // Re-activate the DID by publishing a valid DID document.
                        let reactivated_output: AliasOutput =
                            self.client.update_did_output(document.clone()).await?;

                        // Increase the storage deposit to the minimum again, if it was reclaimed during deactivation.
                        let rent_structure = self.client.get_rent_structure().await?;
                        let reactivated_output = AliasOutputBuilder::from(&reactivated_output)
                            .with_minimum_storage_deposit(rent_structure)
                            .finish()?;
                        self.client
                            .publish_did_output(
                                self.stronghold_storage.as_secret_manager(),
                                reactivated_output,
                            )
                            .await?;

                        // Resolve the reactivated DID document.
                        let reactivated: IotaDocument =
                            self.resolver.resolve(&did_info.did).await?;
                        assert_eq!(*document, reactivated);
                        assert!(!reactivated.metadata.deactivated.unwrap_or_default());

                        debug!("Reactivated DID document: {reactivated:#}");
                    }
                    None => return Err(anyhow!("DID was never deactivated {}", did_info.did)),
                }
            }
            None => return Err(anyhow!("No object found at index {}", index)),
        }

        Ok(())
    }

    ///
    ///
    ///
    pub async fn delete_did(&self, index: usize) -> anyhow::Result<()> {
        info!("Deleting DID");

        match self.did_map.get(&index) {
            Some(did_info) => {
                // Deletes the Alias Output and its contained DID Document, rendering the DID permanently destroyed.
                // This operation is *not* reversible.
                // Deletion can only be done by the governor of the Alias Output.
                self.client
                    .delete_did_output(
                        self.stronghold_storage.as_secret_manager(),
                        self.address,
                        &did_info.did,
                    )
                    .await?;

                // Attempting to resolve a deleted DID results in a `NoOutput` error.
                match self.client.resolve_did(&did_info.did).await {
                    Ok(_) => return Err(anyhow!("DID was not deleted {}", did_info.did)),
                    Err(err) => {
                        assert!(matches!(
                            err,
                            identity_iota::iota::Error::DIDResolutionError(
                                iota_sdk::client::Error::Node(
                                    iota_sdk::client::node_api::error::Error::NotFound(..)
                                )
                            )
                        ));
                    }
                }
            }
            None => return Err(anyhow!("No object found at index {}", index)),
        }
        Ok(())
    }
}
