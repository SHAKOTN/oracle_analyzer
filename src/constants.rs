use phf::phf_map;

pub static ORACLE_ADDRESSES: phf::Map<&'static str, &'static str> = phf_map! {
    "ETH-BTC-CL" => "0xdeb288F737066589598e9214E782fa5A8eD689e8",
};
