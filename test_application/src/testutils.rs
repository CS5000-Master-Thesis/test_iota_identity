use log::{info, warn};
use tokio::task;
use tokio::time::{sleep, Duration, Instant};

use crate::didmanager::DIDManager;
use crate::graph::draw_all_measurements;
use crate::utils::{Action, IotaTangleNetwork, Measurement};
use std::collections::HashMap;

pub async fn run_test(
    networks: &Vec<IotaTangleNetwork>,
    num_threads: usize,
    iterations: usize,
) -> anyhow::Result<()> {
    let mut all_measurements: HashMap<IotaTangleNetwork, Measurement> = HashMap::new();

    for network in networks {
        let measurements = all_measurements
            .entry(*network)
            .or_insert_with(Measurement::new);
        spawn_tasks(measurements, num_threads, iterations, *network).await?;
    }

    // let pretty_json = serde_json::to_string_pretty(&all_measurements).unwrap();
    // info!("Result: {} \n", pretty_json);

    if let Err(e) = draw_all_measurements(&all_measurements) {
        warn!("Failed generate images: {:?}", e);
    }
    Ok(())
}

// pub async fn test_localhost() -> anyhow::Result<()> {
//     let num_threads = 5;
//     let iterations = 100;
//     let network = IotaTangleNetwork::Localhost;
//     let mut measurements: Measurement = Measurement::new();
//     let mut handles = vec![];

//     match DIDManager::new(network.api_endpoint(), network.faucet_endpoint()).await {
//         Ok(mut did_manager) => {
//             let _ = did_manager.create_did(0).await;
//             let did = did_manager.get_did(0);

//             for _ in 0..num_threads {
//                 let iterations = iterations.clone();
//                 let did = did.clone();
//                 let did_manager = &did_manager;
//                 let handle = task::spawn(async move {
//                     let action = Action::ResolveDid;
//                     let mut measurement = Measurement::new();

//                     let action_measurements = measurement.entry(action).or_insert_with(Vec::new);

//                     for _index in 0..iterations {
//                         let start = Instant::now();

//                         let _ = did_manager.resolve_did_2(&did).await;

//                         let duration = start.elapsed();
//                         action_measurements.push(duration);
//                     }

//                     measurement
//                 });

//                 handles.push(handle);
//             }
//         }
//         Err(e) => {
//             warn!("Failed to create DIDManager: {:?}", e);
//         }
//     }

//     // Await all the tasks to complete
//     for handle in handles {
//         match handle.await {
//             Ok(mut result) => {
//                 for (action, durations) in &mut result {
//                     let element = measurements.entry(*action).or_insert_with(Vec::new);
//                     element.append(durations);
//                 }
//             }
//             Err(err) => {
//                 warn!("Invalid thread results: {:?}", err);
//             }
//         }
//     }

//     // let pretty_json = serde_json::to_string_pretty(&all_measurements).unwrap();
//     // info!("Result: {} \n", pretty_json);

//     if let Err(e) = draw_action_measurements(&measurements) {
//         warn!("Failed generate images: {:?}", e);
//     }
//     Ok(())
// }

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

            sleep(Duration::from_millis(500)).await; // Wait 500 milliseconds before starting each thread

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

                            did_manager.run_action(action, index).await;

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
