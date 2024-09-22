use iota_sdk::client::{api::ClientBlockBuilderOptions, Client};
use log::{info, warn};
use tokio::time::Instant;

use crate::utils::{utf8_to_hex, IotaTangleNetwork};

pub async fn run_tasks() -> anyhow::Result<()> {
    let number_of_tasks = 24;
    let number_of_iterations = 10;
    let mut tasks = Vec::new();
    let mut all_durations: Vec<std::time::Duration> = Vec::new();

    for _ in 0..number_of_tasks {
        let number_of_iterations = number_of_iterations;
        tasks.push(tokio::spawn(async move {
            // Create a new client to interact with the IOTA ledger.
            let mut durations = Vec::new();

            let client_builder = Client::builder()
                .with_primary_node(&IotaTangleNetwork::Localhost.api_endpoint(), None);

            match client_builder {
                Ok(builder) => match builder.finish().await {
                    Ok(client) => {
                        for _ in 0..number_of_iterations {
                            // let start = Instant::now();
                            let _ = build_and_post_block(&client).await;
                            // let duration = start.elapsed();
                            // durations.push(duration);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to create client: {:?}", e);
                    }
                },
                Err(e) => {
                    warn!("Failed to build client: {:?}", e);
                }
            }

            // match Client::builder()
            //     .with_primary_node(&IotaTangleNetwork::LocalhostHornet1.name(), None)?
            //     .finish()
            //     .await
            // {
            //     Ok(client) => {
            //         for _ in 0..number_of_iterations {
            //             let start = Instant::now();

            //             let _ = build_and_post_block(&client).await.unwrap();

            //             let duration = start.elapsed();
            //             durations.push(duration);
            //         }
            //     }
            //     Err(e) => {
            //         warn!("Failed to create DIDManager: {:?}", e);
            //     }
            // }
            durations
        }));
    }

    // Wait for all tasks to complete
    // let _results: Vec<_> = join_all(tasks).await;

    for handle in tasks {
        match handle.await {
            Ok(mut result) => {
                all_durations.append(&mut result);
            }
            Err(err) => {
                warn!("Invalid thread results: {:?}", err);
            }
        }
    }

    Ok(())
}

async fn build_and_post_block(client: &Client) -> anyhow::Result<()> {
    // info!("build_and_post_block");

    let tag = utf8_to_hex("Hello tag");
    let data = utf8_to_hex("Hello data");

    // ClientBlockBuilderOptions
    let options = ClientBlockBuilderOptions {
        coin_type: None,
        account_index: None,
        initial_address_index: None,
        inputs: None,
        input_range: None,
        output: None,
        output_hex: None,
        outputs: None,
        custom_remainder_address: None,
        tag: Some(tag),
        data: Some(data),
        parents: None,
        burn: None,
    };

    let mut block_builder = client.build_block();
    block_builder = block_builder.set_options(options).await?;

    let _block = block_builder.finish().await?;
    // let block_id = block.id();
    // info!("BlockId: {}", block_id);

    Ok(())
}
