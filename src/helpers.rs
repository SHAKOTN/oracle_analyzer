use web3::transports::Http;
use web3::Web3;

/// Instantiates web3 instance using given rpc_node_url String
pub fn get_web3(rpc_node_url: String) -> Web3<Http> {
    let transport = Http::new(&*rpc_node_url).unwrap();
    let web3 = Web3::new(transport);
    return web3;
}
