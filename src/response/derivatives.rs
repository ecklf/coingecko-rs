#![allow(missing_docs)]
use super::common::{ConvertedLast, ConvertedVolume};
use serde::{Deserialize, Serialize};

// ---------------------------------------------
//  /derivatives
// ---------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Derivative {
    pub market: String,
    pub symbol: String,
    pub index_id: String,
    pub price: String,
    #[serde(rename = "price_percentage_change_24h")]
    pub price_percentage_change24_h: f64,
    pub contract_type: String,
    pub index: Option<f64>,
    pub basis: f64,
    pub spread: Option<f64>,
    pub funding_rate: f64,
    pub open_interest: Option<f64>,
    #[serde(rename = "volume_24h")]
    pub volume24_h: f64,
    pub last_traded_at: i64,
    pub expired_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DerivativeExchange {
    pub name: String,
    pub id: String,
    pub open_interest_btc: Option<f64>,
    #[serde(rename = "trade_volume_24h_btc")]
    pub trade_volume24_h_btc: Option<String>,
    pub number_of_perpetual_pairs: Option<i64>,
    pub number_of_futures_pairs: Option<i64>,
    pub image: Option<String>,
    pub year_established: Option<i64>,
    pub country: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DerivativeExchangeData {
    pub name: String,
    pub open_interest_btc: f64,
    #[serde(rename = "trade_volume_24h_btc")]
    pub trade_volume24_h_btc: String,
    pub number_of_perpetual_pairs: i64,
    pub number_of_futures_pairs: i64,
    pub image: String,
    pub year_established: Option<i64>,
    pub country: String,
    pub description: String,
    pub url: String,
    pub tickers: Vec<DerivativeExchangeTicker>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DerivativeExchangeTicker {
    pub symbol: String,
    pub base: String,
    pub target: String,
    pub trade_url: String,
    pub contract_type: String,
    pub last: f64,
    pub h24_percentage_change: f64,
    pub index: Option<f64>,
    pub index_basis_percentage: f64,
    pub bid_ask_spread: f64,
    pub funding_rate: f64,
    pub open_interest_usd: f64,
    pub h24_volume: f64,
    pub converted_volume: ConvertedVolume,
    pub converted_last: ConvertedLast,
    pub last_traded: i64,
    pub expired_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DerivativeExchangeId {
    pub name: String,
    pub id: String,
}
