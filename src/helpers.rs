use web3::contract::{Contract, Error, Options};
use web3::transports::Http;
use web3::types::U256;
use web3::Web3;

/// Returns a Web3 instance linked to the Ethereum node
pub fn get_web3(rpc_node_url: String) -> Result<Web3<Http>, String> {
    let transport = Http::new(&*rpc_node_url);
    match transport {
        Ok(transport) => Ok(Web3::new(transport)),
        Err(_) => Err(format!(
            "Failed to connect to Ethereum node at {}",
            rpc_node_url
        )),
    }
}

/// Async function to fetch round data for a given round number
pub async fn get_round_data(
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

#[cfg(test)]
mod tests {
    use crate::constants;
    use crate::helpers::get_round_data;
    use crate::helpers::get_web3;

    use mockito;
    use web3::contract::Contract;
    use web3::transports::Http;

    fn gen_hex_string(lst: Vec<u128>) -> String {
        let mut hex_string = String::new();
        for item in lst.iter() {
            let hex_item = format!("{:x}", item);
            let padded_hex_item = format!("{:0>64}", hex_item);
            hex_string.push_str(&padded_hex_item);
        }
        let result = format!("0x{}", hex_string);
        result
    }

    /// Test that the get_round_data function returns the correct data
    #[tokio::test]
    async fn test_get_round_data_happy() {
        // Generate mock data
        let data = [1u128, 2u128, 3u128, 4u128, 5u128];
        let hex_string = gen_hex_string(data.to_vec());
        let server = mockito::server_url();
        let _mock_transport = mockito::mock("POST", "/")
            .with_status(200)
            .with_body(
                format!(r#"{{"jsonrpc":"2.0","result":"{}","id":1}}"#, hex_string),
            )
            .create();

        let mock_server_url = server.to_string();
        let web3 = web3::Web3::new(Http::new(&mock_server_url).unwrap());
        let contract = Contract::from_json(
            web3.eth(),
            constants::ORACLE_ADDRESSES["ETH-BTC-CL"].parse().unwrap(),
            include_bytes!("./res/AggregatorCL.json"),
        )
            .unwrap();
        let round_data = get_round_data(1.into(), &contract).await.unwrap();
        assert_eq!(
            round_data,
            (1.into(), 2.into(), 3.into(), 4.into(), 5.into())
        );
    }

    #[test]
    /// Test that the Web3 instance is created successfully given a valid RPC node URL
    fn test_get_web3_happy() {
        get_web3("https://eth-mainnet.gateway.pokt.network/".to_string()).unwrap();
    }

    #[test]
    /// Test that the Web3 instance creation fails given an invalid RPC node URL
    fn test_get_web3_unhappy() {
        let invalid_url = "some_invalid_url";
        let error = get_web3(invalid_url.to_string()).unwrap_err();
        assert_eq!(
            error,
            format!("Failed to connect to Ethereum node at {}", invalid_url)
        );
    }
}
