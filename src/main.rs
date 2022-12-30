#[macro_use]
extern crate log;

use csv::Writer;
use dotenv::dotenv;
use futures::future::join_all;
use std::error::Error as StdError;
use std::iter::successors;
use web3::contract::{Contract, Error, Options};
use web3::transports::Http;
use web3::types::{Address, U256};

mod constants;
mod helpers;

const DEPTH: u32 = 100000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    dotenv().ok();
    env_logger::init();
    let web3 =
        helpers::get_web3(std::env::var("ETHNODEURL").expect("ETHNODEURL must be set.")).unwrap();
    // TODO: Candidate for concurrent approach
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
        let oracle_address: Address = address_str.parse().unwrap();
        // Instantiate the contract
        let contract = Contract::from_json(
            web3.eth(),
            oracle_address,
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
        let numbers: Vec<U256> = successors(Some(round_data.0 - 1), |n| Some(n - 1))
            .take(DEPTH as usize)
            .collect();
        // Create dynamic array to collect futures
        let mut futures = vec![];
        // Create futures for all the rounds
        for number in numbers {
            let future = Box::pin(get_round_data(number, &contract));
            futures.push(future);
        }
        let results: Vec<Result<(U256, U256, U256, U256, U256), Error>> = join_all(futures).await;
        for result in results {
            match result {
                Ok(value) => {
                    info!("Fetched result succesfully!");
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
                    error!("Execution reverted!",);
                    continue;
                }
            }
        }
    }
    Ok(())
}

/// Async get round data by number
async fn get_round_data(
    round_number: U256,
    contract: &Contract<Http>,
) -> Result<(U256, U256, U256, U256, U256), Error> {
    let round_data: Result<(U256, U256, U256, U256, U256), Error> = contract
        .query("getRoundData", round_number, None, Options::default(), None)
        .await;
    match round_data {
        Ok(round_data) => Ok(round_data),
        Err(error) => Err(error),
    }
}
