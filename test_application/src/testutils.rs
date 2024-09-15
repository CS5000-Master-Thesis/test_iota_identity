use log::{info, warn};
use tokio::task;
use tokio::time::Instant;

use crate::didmanager::DIDManager;
use crate::graph::{draw_action_measurements, draw_all_measurements};
use crate::utils::{Action, IotaTangleNetwork, Measurement};
use std::collections::HashMap;

pub async fn test_public_testnet() -> anyhow::Result<()> {
    // let num_threads = num_cpus::get();
    let num_threads = std::cmp::min(num_cpus::get(), 3);
    let iterations = 2;

    info!("Number of available logical CPUs: {}", num_threads);

    let networks = vec![
        IotaTangleNetwork::IotaTestnet,
        IotaTangleNetwork::ShimmerTestnet,
    ];

    let mut all_measurements: HashMap<IotaTangleNetwork, Measurement> = HashMap::new();

    for network in networks {
        let measurements = all_measurements
            .entry(network)
            .or_insert_with(Measurement::new);
        spawn_tasks(measurements, num_threads, iterations, network).await?;
    }

    // let pretty_json = serde_json::to_string_pretty(&all_measurements).unwrap();
    // info!("Result: {} \n", pretty_json);

    if let Err(e) = draw_all_measurements(&all_measurements) {
        warn!("Failed generate images: {:?}", e);
    }
    Ok(())
}

pub async fn test_localhost() -> anyhow::Result<()> {
    let iterations = 5;
    let mut measurements: Measurement = Measurement::new();
    let mut handles = vec![];
    let networks = vec![
        IotaTangleNetwork::LocalhostHornet1,
        IotaTangleNetwork::LocalhostHornet2,
        IotaTangleNetwork::LocalhostHornet3,
        IotaTangleNetwork::LocalhostHornet4,
    ];

    for network in networks {
        let network = network.clone();
        let iterations = iterations.clone();
        let handle = task::spawn(async move {
            let mut measurement = Measurement::new();

            match DIDManager::new(network.api_endpoint(), network.faucet_endpoint()).await {
                Ok(mut did_manager) => {
                    let actions = vec![
                        Action::CreateDid,
                        Action::UpdateDid,
                        Action::ResolveDid,
                        Action::DeactivateDid,
                        Action::ReactivateDid,
                        Action::DeleteDid,
                    ];

                    for action in &actions {
                        let action_measurements =
                            measurement.entry(*action).or_insert_with(Vec::new);

                        for index in 0..iterations {
                            let start = Instant::now();

                            benchmark_operation(&mut did_manager, action, index).await;

                            let duration = start.elapsed();
                            action_measurements.push(duration);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to create DIDManager: {:?}", e);
                }
            }

            measurement
        });

        handles.push(handle);
    }

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

    // let pretty_json = serde_json::to_string_pretty(&all_measurements).unwrap();
    // info!("Result: {} \n", pretty_json);

    if let Err(e) = draw_action_measurements(&measurements) {
        warn!("Failed generate images: {:?}", e);
    }
    Ok(())
}

async fn spawn_tasks(
    measurements: &mut Measurement,
    num_threads: usize,
    iterations: usize,
    network: IotaTangleNetwork,
) -> anyhow::Result<()> {
    let mut handles = vec![];

    info!(
        "Starting testing for {}\nAPI: {}\nFaucet: {}",
        network.name(),
        network.api_endpoint(),
        network.faucet_endpoint()
    );

    for _ in 0..num_threads {
        let network = network.clone();
        let iterations = iterations.clone();
        let handle = task::spawn(async move {
            let mut measurement = Measurement::new();

            match DIDManager::new(network.api_endpoint(), network.faucet_endpoint()).await {
                Ok(mut did_manager) => {
                    let actions = vec![
                        Action::CreateDid,
                        Action::UpdateDid,
                        Action::ResolveDid,
                        Action::DeactivateDid,
                        Action::ReactivateDid,
                        Action::DeleteDid,
                    ];

                    for action in &actions {
                        let action_measurements =
                            measurement.entry(*action).or_insert_with(Vec::new);

                        for index in 0..iterations {
                            let start = Instant::now();

                            benchmark_operation(&mut did_manager, action, index).await;

                            let duration = start.elapsed();
                            action_measurements.push(duration);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to create DIDManager: {:?}", e);
                }
            }

            measurement
        });

        handles.push(handle);
    }

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
    Ok(())
}

async fn benchmark_operation(did_manager: &mut DIDManager, action: &Action, index: usize) {
    match action {
        Action::CreateDid => {
            if let Err(e) = did_manager.create_did(index).await {
                warn!("Failed to create DID: {:?}", e);
            }
        }
        Action::DeleteDid => {
            if let Err(e) = did_manager.delete_did(index).await {
                warn!("Failed to delete DID: {:?}", e);
            }
        }
        Action::UpdateDid => {
            if let Err(e) = did_manager.update_did(index).await {
                warn!("Failed to update DID: {:?}", e);
            }
        }
        Action::ResolveDid => {
            if let Err(e) = did_manager.resolve_did(index).await {
                warn!("Failed to resolve DID: {:?}", e);
            }
        }
        Action::DeactivateDid => {
            if let Err(e) = did_manager.deactivate_did(index).await {
                warn!("Failed to deactivate DID: {:?}", e);
            }
        }
        Action::ReactivateDid => {
            if let Err(e) = did_manager.reactivate_did(index).await {
                warn!("Failed to reactivate DID: {:?}", e);
            }
        }
    }
}
