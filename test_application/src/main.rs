use testutils::test_localhost;

mod didmanager;
mod graph;
mod testutils;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().map_err(|e| anyhow::anyhow!("Failed to load .env file: {}", e))?;
    env_logger::init();

    if let Err(e) = test_localhost().await {
        log::error!("Error occurred in test_localhost: {:?}", e);
        return Err(e);
    }

    log::info!("Application finished successfully.");
    Ok(())
}
