use phf::phf_map;

pub static ORACLE_ADDRESSES: phf::Map<&'static str, &'static str> = phf_map! {
    "ETH-BTC-CL" => "0xdeb288F737066589598e9214E782fa5A8eD689e8",
    // "ETH-USD-CL" => "0x5f4ec3df9cbd43714fe2740f5e3616155c5b8419",
};
