use buildandpostblocktest::run_tasks;
use testutils::run_test;
use utils::IotaTangleNetwork;

mod buildandpostblocktest;
mod didmanager;
mod graph;
mod resolvedidtest;
mod testutils;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().map_err(|e| anyhow::anyhow!("Failed to load .env file: {}", e))?;
    env_logger::init();

    let num_threads = std::cmp::min(num_cpus::get(), 1);
    let iterations = 1;

    let networks = vec![
        IotaTangleNetwork::Localhost,
        // IotaTangleNetwork::IotaTestnet,
        // IotaTangleNetwork::ShimmerTestnet,
    ];

    if let Err(e) = run_test(&networks, num_threads, iterations).await {
        log::error!("Error occurred in test_localhost: {:?}", e);
        return Err(e);
    }

    // if let Err(e) = run_tasks().await {
    //     log::error!("Error occurred in test_localhost: {:?}", e);
    //     return Err(e);
    // }

    log::info!("Application finished successfully.");
    Ok(())
}
