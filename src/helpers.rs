use web3::transports::Http;
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

#[cfg(test)]
mod tests {
    use crate::helpers::get_web3;

    #[test]
    /// Test that the Web3 instance is created successfully given a valid RPC node URL
    fn test_get_web3_happy() {
        get_web3("https://eth-mainnet.gateway.pokt.network/".to_string()).unwrap();
    }

    #[test]
    fn test_get_web3_unhappy() {
        let invalid_url = "some_invalid_url";
        let error = get_web3(invalid_url.to_string()).unwrap_err();
        assert_eq!(
            error,
            format!("Failed to connect to Ethereum node at {}", invalid_url)
        );
    }
}
