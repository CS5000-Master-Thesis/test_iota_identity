mod didmanager;
mod graph;
mod plotlytest;
mod utils;

use std::collections::HashMap;

use didmanager::DIDManager;
use graph::{draw_all_measurements, draw_custom};
use plotlytest::{box_plot_styling_outliers, fully_styled_box_plot};
use utils::{Action, IotaTangleNetwork, Measurement};

use log::{info, warn};
use tokio::task;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();

    box_plot_styling_outliers();

    // let num_threads = num_cpus::get();
    // println!("Number of available logical CPUs: {}", num_threads);

    // let networks = vec![
    //     IotaTangleNetwork::Localhost,
    //     // IotaTangleNetwork::ShimmerTestnet,
    //     // IotaTangleNetwork::IotaTestnet2_0,
    // ];

    // let mut all_measurements: HashMap<IotaTangleNetwork, Measurement> = HashMap::new();

    // for network in networks {
    //     let measurements = all_measurements
    //         .entry(network)
    //         .or_insert_with(Measurement::new);
    //     spawn_tasks(measurements, num_threads, network).await?;
    // }

    // let pretty_json = serde_json::to_string_pretty(&all_measurements).unwrap();
    // info!("Result: {} \n", pretty_json);

    // if let Err(e) = draw_all_measurements(&all_measurements) {
    //     warn!("Failed generate images: {:?}", e);
    // }

    Ok(())
}

async fn spawn_tasks(
    measurements: &mut Measurement,
    num_threads: usize,
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

                    let iterations = 5;

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
        let mut result = handle.await.unwrap();

        for (action, durations) in &mut result {
            let element = measurements.entry(*action).or_insert_with(Vec::new);
            element.append(durations);
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
