#[macro_use]
extern crate log;

use std::error::Error;

use csv::Writer;
use dotenv::dotenv;
use tokio::task;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

mod constants;
mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();
    let web3 = helpers::get_web3(std::env::var("ETHNODEURL").expect("ETHNODEURL must be set."));
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
        // Fetch the data from the Oracle
        let future = async move {
            return contract
                .query("latestRoundData", (), None, Options::default(), None)
                .await
                .unwrap();
        };
        let round_data: (U256, U256, U256, U256, U256) = task::spawn(future).await?;
        // Write latest round data to .csv file
        writer.write_record(&[
            round_data.0.to_string(),
            round_data.1.to_string(),
            round_data.2.to_string(),
            round_data.3.to_string(),
            round_data.4.to_string(),
        ])?;
        writer.flush()?;
    }
    Ok(())
}
