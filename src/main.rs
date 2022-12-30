#[macro_use]
extern crate log;

use std::error::Error as StdError;
use std::iter::successors;

use csv::Writer;
use dotenv::dotenv;
use futures::future::join_all;
use web3::contract::{Contract, Error, Options};
use web3::types::U256;

mod constants;
mod helpers;

// Max amount of rounds to fetch
const DEPTH: u32 = 100000;

/// Entry point for the application
#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    dotenv().ok();
    env_logger::init();
    let web3 =
        helpers::get_web3(std::env::var("ETHNODEURL").expect("ETHNODEURL must be set.")).unwrap();
    for (oracle_name, address_str) in constants::ORACLE_ADDRESSES.into_iter() {
        let mut writer =
            Writer::from_path(format!("{oracle}_oracle_data.csv", oracle = oracle_name))?;
        // Write headers to the file
        writer.write_record(&[
            "roundId",
            "answer",
            "startedAt",
            "updatedAt",
            "answeredInRound",
        ])?;
        info!("Fetching Oracle data for {}", oracle_name);
        // Instantiate the contract
        let contract = Contract::from_json(
            web3.eth(),
            address_str.parse().unwrap(),
            include_bytes!("./res/AggregatorCL.json"),
        )
        .unwrap();
        // Get latest round data
        let round_data: (U256, U256, U256, U256, U256) = contract
            .query("latestRoundData", (), None, Options::default(), None)
            .await
            .unwrap();
        // Write latest round data to .csv file
        writer.write_record(&[
            round_data.0.to_string(),
            round_data.1.to_string(),
            round_data.2.to_string(),
            round_data.3.to_string(),
            round_data.4.to_string(),
        ])?;
        writer.flush()?;
        // Generate how many rounds we want to fetch data for
        let num_rounds: Vec<U256> = successors(Some(round_data.0 - 1), |n| Some(n - 1))
            .take(DEPTH as usize)
            .collect();
        let mut execution_revert = false;
        let limit = 100;
        let mut offset = 0;
        while execution_revert != true {
            // Create dynamic array to collect futures
            let mut futures = vec![];
            // Create futures for all the rounds
            for round in &num_rounds[offset..(offset + limit)] {
                let future = Box::pin(helpers::get_round_data(*round, &contract));
                futures.push(future);
            }
            let results: Vec<Result<(U256, U256, U256, U256, U256), Error>> =
                join_all(futures).await;
            for result in results {
                match result {
                    Ok(value) => {
                        writer.write_record(&[
                            value.0.to_string(),
                            value.1.to_string(),
                            value.2.to_string(),
                            value.3.to_string(),
                            value.4.to_string(),
                        ])?;
                        writer.flush()?;
                    }
                    Err(__) => {
                        warn!("Execution reverted!",);
                        execution_revert = true;
                        break;
                    }
                }
            }
            offset += limit;
        }
    }
    Ok(())
}
