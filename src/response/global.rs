#![allow(missing_docs)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------
//  /global
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Global {
    pub data: GlobalData,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalData {
    pub active_cryptocurrencies: f64,
    pub upcoming_icos: f64,
    pub ongoing_icos: f64,
    pub ended_icos: f64,
    pub markets: f64,
    pub total_market_cap: HashMap<String, f64>,
    pub total_volume: HashMap<String, f64>,
    pub market_cap_percentage: HashMap<String, f64>,
    #[serde(rename = "market_cap_change_percentage_24h_usd")]
    pub market_cap_change_percentage24_h_usd: f64,
    pub updated_at: f64,
}

// ---------------------------------------------
//  /global/decentralized_finance_defi
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalDefi {
    pub data: GlobalDefiData,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalDefiData {
    pub defi_market_cap: String,
    pub eth_market_cap: String,
    pub defi_to_eth_ratio: String,
    #[serde(rename = "trading_volume_24h")]
    pub trading_volume24_h: String,
    pub defi_dominance: String,
    pub top_coin_name: String,
    pub top_coin_defi_dominance: f64,
}
