use buildandpostblocktest::{
    build_and_post_block_test, run_for_all_nodes_configurations_block_test,
};
use resolvedidtest::resolve_did_test;
use testutils::run_test;
use utils::{wait_until_enter_pressed, IotaTangleNetwork};

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

    /////////////////////// Test DID functions /////////////////////////////
    let num_threads = std::cmp::min(num_cpus::get(), 5);
    let iterations = 5;

    let networks = vec![
        IotaTangleNetwork::Localhost,
        // IotaTangleNetwork::IotaTestnet,
        // IotaTangleNetwork::ShimmerTestnet,
    ];

    if let Err(e) = run_test(&networks, num_threads, iterations).await {
        log::error!("Error occurred in test_localhost: {:?}", e);
        return Err(e);
    }

    ///////////////////// Create one DID and resolve /////////////////////////////
    // resolve_did_test().await;

    /////////////////////// Build and post blocks /////////////////////////////
    // run_for_all_nodes_configurations_block_test().await;

    log::info!("Application finished successfully.");
    Ok(())
}
