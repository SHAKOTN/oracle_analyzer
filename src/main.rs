mod helpers;

use dotenv::dotenv;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv().ok();
    let web3 = helpers::get_web3(std::env::var("ETHNODEURL").expect("ETHNODEURL must be set."));
    let oracle_address: Address = "0xdeb288F737066589598e9214E782fa5A8eD689e8"
        .parse()
        .unwrap();
    let contract = Contract::from_json(
        web3.eth(),
        oracle_address,
        include_bytes!("./res/AggregatorCL.json"),
    )
    .unwrap();
    let round_data: (U256, U256, U256, U256, U256) = contract
        .query("latestRoundData", (), None, Options::default(), None)
        .await
        .unwrap();
    println!("{}", round_data.1);
    Ok(())
}
