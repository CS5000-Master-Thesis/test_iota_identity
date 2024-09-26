use iota_sdk::client::{api::ClientBlockBuilderOptions, Client};
use log::{info, warn};
use tokio::time::{sleep, Duration, Instant};

use crate::{
    graph::{draw_action_measurements, get_and_create_folder},
    utils::{
        calculate_stats, utf8_to_hex, wait_until_enter_pressed, Action, IotaTangleNetwork,
        Measurement, MeasurementResult,
    },
};

pub async fn run_for_all_nodes_configurations_block_test() {
    let configurations = vec![Action::nodes_4, Action::nodes_3, Action::nodes_2];
    let mut measurement = Measurement::new();

    for (index, action) in configurations.iter().enumerate() {
        build_and_post_block_test(&action, &mut measurement).await;

        if index != configurations.len() - 1 {
            info!("Turn off one node.");
            wait_until_enter_pressed();
        }
    }

    let folder_name = get_and_create_folder().unwrap();
    draw_action_measurements(
        &Action::CreateAndPostBlock.name(),
        &measurement,
        folder_name,
    );
}

pub async fn build_and_post_block_test(action: &Action, measurements: &mut Measurement) {
    let number_of_tasks = 2;
    let number_of_iterations = 10_000;
    let local_pow = true;

    let mut tasks = Vec::new();
    let mut result = MeasurementResult::new();
    let test_start = Instant::now();
    info!("--------------------------------------------------");

    for index in 0..number_of_tasks {
        let action = action.clone();
        let number_of_iterations = number_of_iterations.clone();
        let local_pow = local_pow.clone();
        let network = if index % 2 == 0 {
            IotaTangleNetwork::Localhost
        } else {
            IotaTangleNetwork::Localhost2
        };

        tasks.push(tokio::spawn(async move {
            let mut result = MeasurementResult::new();

            let client_builder = Client::builder()
                .with_local_pow(local_pow)
                .with_fallback_to_local_pow(local_pow)
                .with_primary_node(&network.api_endpoint(), None);

            match client_builder {
                Ok(builder) => match builder.finish().await {
                    Ok(client) => {
                        // let local_pow = client.get_local_pow().await;
                        // info!("Local PoW {}", local_pow);
                        // let info = client.get_info().await.unwrap().node_info;
                        // info!("{info}");

                        let action_measurements =
                            result.measurement.entry(action).or_insert_with(Vec::new);

                        for _ in 0..number_of_iterations {
                            let start = Instant::now();
                            match build_and_post_block(&client, network).await {
                                Ok(_) => {
                                    let duration = start.elapsed();
                                    action_measurements.push(duration);
                                }
                                Err(e) => {
                                    result.failures += 1;
                                    warn!("Failed to post block: {:?}", e);
                                }
                            };
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
            result
        }));
    }

    // Wait for all tasks to complete
    // let _results: Vec<_> = join_all(tasks).await;

    for handle in tasks {
        match handle.await {
            Ok(mut task_result) => {
                for (action, durations) in &mut task_result.measurement {
                    let element = result.measurement.entry(*action).or_insert_with(Vec::new);
                    element.append(durations);
                }
                result.failures += task_result.failures;
            }
            Err(err) => {
                warn!("Invalid thread results: {:?}", err);
            }
        }
    }

    let test_duration = test_start.elapsed();
    let number_of_blocks = number_of_tasks * number_of_iterations;
    let blocks_per_second = (number_of_blocks as f64) / test_duration.as_secs_f64();

    info!("Task complete: Build and post block {}", action.name());
    info!("Blocks: {:?}", number_of_blocks);
    info!("BPS: {:.3}", blocks_per_second);
    info!("Duration: {:.3}", test_duration.as_secs_f64());
    info!("Failures: {:?}", result.failures);

    for (_action, durations) in &mut result.measurement {
        let secs_f64: Vec<f64> = durations.iter().map(|d| d.as_secs_f64()).collect();
        let stats = calculate_stats(secs_f64);
        info!("Min: {:.3}", stats.min);
        info!("Max: {:.3}", stats.max);
        info!("Average: {:.3}", stats.average);

        // Copy measurements
        let element = measurements.entry(*action).or_insert_with(Vec::new);
        element.append(durations);
    }
    info!("--------------------------------------------------");
}

async fn build_and_post_block(client: &Client, network: IotaTangleNetwork) -> anyhow::Result<()> {
    // info!("build_and_post_block");

    let dsa = format!("Hello tag {:?}", network.api_endpoint());

    let tag = utf8_to_hex(&dsa);
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

    // info!("BlockId: {:?}", _block);

    // if network == IotaTangleNetwork::Localhost2 {
    //     let block_id = _block.id();
    //     info!("BlockId: {}", block_id);
    // }

    Ok(())
}
