mod didmanager;
mod graph;
mod utils;

use didmanager::DIDManager;
use utils::Action;
// use graph::gen_line_chart;
// use graph::test_clustered_bar_chart;

use futures::future::join_all;
use futures::stream::{self, StreamExt};
use log::{info, warn};
use tokio::task;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();

    // gen_line_chart();
    // test_clustered_bar_chart();

    // The API endpoint of an IOTA node, e.g. Hornet.
    let api_endpoint: &str = "http://localhost";

    // The faucet endpoint allows requesting funds for testing purposes.
    let faucet_endpoint: &str = "http://localhost/faucet/api/enqueue";

    // Define a concurrency level. For example, 10 means up to 10 tasks run in parallel.
    let number_of_did_managers = 3;
    let concurrency_level = 3;

    let futures =
        (0..number_of_did_managers).map(|_| DIDManager::new(api_endpoint, faucet_endpoint));
    let mut did_managers: Vec<DIDManager> = join_all(futures)
        .await
        .into_iter()
        .collect::<Result<_, _>>()?;

    let actions = vec![
        Action::CreateDid,
        Action::UpdateDid,
        Action::ResolveDid,
        Action::DeactivateDid,
        Action::ReactivateDid,
        Action::DeleteDid,
    ];

    let iterations = vec![1];
    // let iterations = vec![1, 10, 100, 1000, 10000];

    for action in actions {
        for iter in &iterations {
            benchmark_operation(
                &mut did_managers,
                action.clone(),
                iter.clone(),
                concurrency_level,
            )
            .await;
        }
    }

    Ok(())
}

async fn spawn_tasks() {}

async fn benchmark_operation(
    did_managers: &mut Vec<DIDManager>,
    action: Action,
    iterations: usize,
    concurrency_level: usize,
) {
    let start = Instant::now();

    let mut handles = vec![];

    for did_manager in did_managers {
        let action = action.clone();

        let handle = task::spawn(async move {
            for index in 0..iterations {
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
        });

        handles.push(handle);
    }

    // Await all the tasks to complete
    for handle in handles {
        if let Err(e) = handle.await {
            warn!("Task failed: {:?}", e);
        }
    }

    let duration = start.elapsed();
    info!(
        "{} operation took: {:?} for {} iterations with {} threads",
        action.name(),
        duration,
        iterations,
        concurrency_level
    );
}

// async fn benchmark_operation(
//     did_managers: &mut Vec<DIDManager>,
//     action: Action,
//     iterations: usize,
//     concurrency_level: usize,
// ) {
//     let start = Instant::now();

//     stream::iter(did_managers)
//         .for_each_concurrent(concurrency_level, |did_manager| {
//             let action = action.clone();

//             async move {
//                 for index in 0..iterations {
//                     match action {
//                         Action::CreateDid => {
//                             if let Err(e) = did_manager.create_did(index).await {
//                                 warn!("Failed to create DID: {:?}", e);
//                             }
//                         }
//                         Action::DeleteDid => {
//                             if let Err(e) = did_manager.delete_did(index).await {
//                                 warn!("Failed to delete DID: {:?}", e);
//                             }
//                         }
//                         Action::UpdateDid => {
//                             if let Err(e) = did_manager.update_did(index).await {
//                                 warn!("Failed to update DID: {:?}", e);
//                             }
//                         }
//                         Action::ResolveDid => {
//                             if let Err(e) = did_manager.resolve_did(index).await {
//                                 warn!("Failed to resolve DID: {:?}", e);
//                             }
//                         }
//                         Action::DeactivateDid => {
//                             if let Err(e) = did_manager.deactivate_did(index).await {
//                                 warn!("Failed to deactivate DID: {:?}", e);
//                             }
//                         }
//                         Action::ReactivateDid => {
//                             if let Err(e) = did_manager.reactivate_did(index).await {
//                                 warn!("Failed to reactivate DID: {:?}", e);
//                             }
//                         }
//                     }
//                 }
//             }
//         })
//         .await;

//     let duration = start.elapsed();
//     info!(
//         "{} operation took: {:?} for {} iterations with {} threads",
//         action.name(),
//         duration,
//         iterations,
//         concurrency_level
//     );
// }
