use std::{
    collections::HashMap,
    process::{Command, Stdio},
};

use iota_sdk::client::{api::ClientBlockBuilderOptions, Client};
use log::{info, warn};
use serde::Serialize;
use serde_json::to_string_pretty;
use strum::IntoEnumIterator;
use tokio::time::{sleep, Duration, Instant};

use crate::{
    graph::{draw_action_measurements, get_and_create_folder},
    utils::{
        calculate_stats, print_measurement_stats, save_to_raw_data_file, save_to_results_file,
        utf8_to_hex, wait_until_enter_pressed, Action, IotaTangleNetwork, Measurement,
        MeasurementResult, Stats,
    },
};

#[derive(Debug, Clone, Copy, Serialize)]
struct BuildBlockAndPublishStatResult {
    pub blocks: usize,
    pub bps: f64,
    pub duartion: f64,
    pub failures: usize,
    pub stats: Stats,
}

pub async fn run_for_all_nodes_configurations_block_test(
    number_of_tasks: usize,
    number_of_iterations: usize,
    local_pow: bool,
    min_pow_score: usize,
) {
    let configurations = vec![Action::nodes_4, Action::nodes_3, Action::nodes_2];
    let mut measurement = Measurement::new();
    let mut result_stats: HashMap<Action, BuildBlockAndPublishStatResult> = HashMap::new();
    let mut start_node_number = 4;

    for (index, action) in configurations.iter().enumerate() {
        build_and_post_block_test(
            &action,
            &mut measurement,
            &mut result_stats,
            number_of_tasks,
            number_of_iterations,
            local_pow,
        )
        .await;

        // Turn off one node
        if index != configurations.len() - 1 {
            let image_name = format!("hornet-{}", start_node_number);
            start_node_number -= 1;
            info!("Turn off one node {}", image_name);

            let result = Command::new("docker")
                .arg("compose")
                .arg("stop")
                .arg(image_name)
                .current_dir("../PrivateTangle")
                .status();

            match result {
                Ok(output) => {
                    // let stdout = String::from_utf8_lossy(&output.stdout);
                    info!("Command Output:\n{}", output);
                }
                Err(e) => {
                    warn!("Failed to execute command: {}", e);
                }
            }
            sleep(Duration::from_secs(10)).await;
        }
    }

    let folder_name = get_and_create_folder().unwrap();
    let json_data = to_string_pretty(&measurement).unwrap();
    if let Err(e) = save_to_raw_data_file(json_data, &folder_name) {
        warn!("Error when saving file: {}", e);
    }
    let json_data = to_string_pretty(&result_stats).unwrap();
    if let Err(e) = save_to_results_file(json_data, &folder_name) {
        warn!("Error when saving file: {}", e);
    }

    println!("Local PoW {}", local_pow);
    println!("Min PoW Score {}", min_pow_score);

    println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10} | {8: <10}",
        "Action", "Blocks", "Error", "Duration", "BPS", "Min", "Max", "Mean", "Variance"
    );
    for action in Action::iter() {
        if let Some(stats) = result_stats.get(&action) {
            println!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10.3} | {4: <10.3} | {5: <10.4} | {6: <10.4} | {7: <10.4} | {8: <10.4e}",
            action.name(),
            stats.blocks,
            stats.failures,
            stats.duartion,
            stats.bps,
            stats.stats.min,
            stats.stats.max,
            stats.stats.mean,
            stats.stats.variance,
        );
        }
    }

    let local_pow_string = if local_pow { "Local PoW" } else { "Remote PoW" };
    let plot_title = format!(
        "{} (MinPoWScore: {}, {})",
        &Action::CreateAndPostBlock.name(),
        min_pow_score,
        local_pow_string
    );
    draw_action_measurements(&plot_title, &measurement, &folder_name);
}

async fn build_and_post_block_test(
    action: &Action,
    measurements: &mut Measurement,
    results: &mut HashMap<Action, BuildBlockAndPublishStatResult>,
    number_of_tasks: usize,
    number_of_iterations: usize,
    local_pow: bool,
) {
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
                                    action_measurements.push(duration.as_secs_f64());
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

    // info!("Task complete: Build and post block {}", action.name());
    // info!("Blocks: {:?}", number_of_blocks);
    // info!("BPS: {:.3}", blocks_per_second);
    // info!("Duration: {:.3}", test_duration.as_secs_f64());
    // info!("Failures: {:?}", result.failures);
    // print_measurement_stats(&result.measurement);

    let mut result_stats = BuildBlockAndPublishStatResult {
        blocks: number_of_blocks,
        bps: blocks_per_second,
        duartion: test_duration.as_secs_f64(),
        failures: result.failures,
        stats: Stats::default(),
    };

    for (_action, durations) in &mut result.measurement {
        result_stats.stats = calculate_stats(durations);

        // Copy measurements
        let element = measurements.entry(*action).or_insert_with(Vec::new);
        element.append(durations);
    }

    results.insert(*action, result_stats);
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
