use std::process::Command;

use buildandpostblocktest::run_for_all_nodes_configurations_block_test;
use graph::{line_plot_decline_bps_vs_min_pow_score, line_plot_decline_bps_vs_node_count};
use log::{info, warn};
use resolvedidtest::resolve_did_test;
use testutils::{read_and_print_raw_data, run_test};
use tokio::time::{sleep, Duration};
use utils::{wait_until_enter_pressed, IotaTangleNetwork};

mod buildandpostblocktest;
mod didmanager;
mod graph;
mod resolvedidtest;
mod testutils;
mod utils;

#[derive(Debug, Clone, Copy, Default)]
pub struct Params {
    pub num_threads: usize,
    pub iterations: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().map_err(|e| anyhow::anyhow!("Failed to load .env file: {}", e))?;
    env_logger::init();

    /////////////////////// Test DID functions /////////////////////////////
    // let num_threads = std::cmp::min(num_cpus::get(), 1);
    // let iterations = 1;
    let params = vec![
        // Params {
        //     num_threads: 1,
        //     iterations: 500,
        // },
        Params {
            num_threads: 5,
            iterations: 10,
        },
    ];

    for param in params {
        let networks = vec![
            IotaTangleNetwork::Localhost,
            // IotaTangleNetwork::IotaTestnet,
            // IotaTangleNetwork::ShimmerTestnet,
        ];

        if let Err(e) = run_test(&networks, param.num_threads, param.iterations).await {
            log::error!("Error occurred in test_localhost: {:?}", e);
            return Err(e);
        }
    }

    ///////////////////// Create one DID and resolve /////////////////////////////
    // resolve_did_test().await;

    /////////////////////// Build and post blocks /////////////////////////////
    // let number_of_tasks = 2;
    // let number_of_iterations = 10_000;
    // let local_pow = true;
    // let min_pow_score = 0; // Just for the title of the graph
    // run_for_all_nodes_configurations_block_test(
    //     number_of_tasks,
    //     number_of_iterations,
    //     local_pow,
    //     min_pow_score,
    // )
    // .await;

    // for index in 0..2 {
    //     let number_of_tasks = 2;
    //     let number_of_iterations = 10_000;
    //     let local_pow = if index == 0 { false } else { true };
    //     let min_pow_score = 0; // Just for the title of the graph
    //     run_for_all_nodes_configurations_block_test(
    //         number_of_tasks,
    //         number_of_iterations,
    //         local_pow,
    //         min_pow_score,
    //     )
    //     .await;

    //     let result = Command::new("docker")
    //         .arg("compose")
    //         .arg("start")
    //         .arg("hornet-3")
    //         .arg("hornet-4")
    //         .current_dir("../PrivateTangle")
    //         .status();

    //     match result {
    //         Ok(output) => {
    //             // let stdout = String::from_utf8_lossy(&output.stdout);
    //             info!("Command Output:\n{}", output);
    //         }
    //         Err(e) => {
    //             warn!("Failed to execute command: {}", e);
    //         }
    //     }
    //     sleep(Duration::from_secs(60)).await;
    // }

    /////////////////////// Generate line graphs /////////////////////////////
    // line_plot_decline_bps_vs_node_count();
    // line_plot_decline_bps_vs_min_pow_score();

    /////////////////////// Print raw_data /////////////////////////////
    // let file_names = vec![
    //     "./temp/iota_testnet",
    //     "./temp/localhost_1_task_500",
    //     "./temp/localhost_5_tasks_100",
    // ];
    // for file_name in file_names {
    //     read_and_print_raw_data(file_name);
    // }

    log::info!("Application finished successfully.");
    Ok(())
}
