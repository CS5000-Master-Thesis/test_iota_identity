use identity_iota::{
    iota::{IotaDID, IotaDocument},
    prelude::Resolver,
};
use iota_sdk::client::Client;
use log::{info, warn};
use rand::Rng;

use crate::{
    didmanager::DIDManager,
    graph::{draw_action_measurements, get_and_create_folder},
    utils::{Action, IotaTangleNetwork, Measurement},
};
use tokio::task;
use tokio::time::{sleep, Duration, Instant};

pub async fn resolve_did_test() {
    // Stronghold snapshot path.
    match DIDManager::new(
        IotaTangleNetwork::Localhost.api_endpoint(),
        IotaTangleNetwork::Localhost.faucet_endpoint(),
    )
    .await
    {
        Ok(mut did_manager) => {
            let index = 0;
            let _ = did_manager.create_did(index).await;

            let did_information = did_manager.did_map.get(&index).unwrap();
            let did: IotaDID = did_information.did.clone();
            info!("DID {did}");
            let num_threads = 50;
            let iterations = 100;
            let mut measurement = Measurement::new();

            spawn_tasks(&mut measurement, num_threads, iterations, did).await;

            let folder_name = get_and_create_folder().unwrap();

            draw_action_measurements(
                &IotaTangleNetwork::Localhost.name(),
                &measurement,
                folder_name,
            );
        }
        Err(e) => {
            warn!("Failed to create DIDManager: {:?}", e);
        }
    }
}

async fn spawn_tasks(
    measurements: &mut Measurement,
    num_threads: usize,
    iterations: usize,
    did: IotaDID,
) {
    let mut handles = vec![];
    let test_start = Instant::now();

    for _ in 0..num_threads {
        let iterations = iterations.clone();
        let did = did.clone();
        let random_delay = rand::thread_rng().gen_range(5..=20);
        sleep(Duration::from_millis(random_delay)).await; // Add random delay to simulate users before starting each thread

        let handle = task::spawn(async move {
            let mut measurement = Measurement::new();
            match Client::builder()
                .with_primary_node(IotaTangleNetwork::Localhost.api_endpoint(), None)
            {
                Ok(builder) => match builder.finish().await {
                    Ok(client) => {
                        // Successfully created the client, you can use the client here

                        let mut resolver = Resolver::<IotaDocument>::new();
                        resolver.attach_iota_handler(client.clone());

                        let action_measurements = measurement
                            .entry(Action::ResolveDid)
                            .or_insert_with(Vec::new);

                        for _ in 0..iterations {
                            let start = Instant::now();

                            match resolver.resolve(&did).await {
                                Ok(resolved_document) => {
                                    assert_eq!(did, *resolved_document.id());
                                    let duration = start.elapsed();
                                    action_measurements.push(duration);
                                }
                                Err(e) => {
                                    warn!("Error: {:?}", e);
                                }
                            };
                        }
                    }
                    Err(e) => {
                        warn!("Error creating the client: {:?}", e);
                    }
                },
                Err(e) => {
                    warn!("Error adding primary node: {:?}", e);
                }
            };
            measurement
        });

        handles.push(handle);
    }

    let test_duration = test_start.elapsed();
    info!("Duration {:?}", test_duration);

    // Await all the tasks to complete
    for handle in handles {
        match handle.await {
            Ok(mut result) => {
                for (action, durations) in &mut result {
                    let element = measurements.entry(*action).or_insert_with(Vec::new);
                    element.append(durations);
                }
            }
            Err(err) => {
                warn!("Invalid thread results: {:?}", err);
            }
        }
    }

    info!("------------------------------------------------");
}
