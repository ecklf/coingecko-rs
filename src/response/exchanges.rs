use serde::{Deserialize, Serialize};

// ---------------------------------------------
//  /exchanges
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Exchange {
    pub id: String,
    pub name: String,
    pub year_established: Option<i64>,
    pub country: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub has_trading_incentive: Option<bool>,
    pub trust_score: Option<i64>,
    pub trust_score_rank: Option<i64>,
    #[serde(rename = "trade_volume_24h_btc")]
    pub trade_volume24_h_btc: Option<f64>,
    #[serde(rename = "trade_volume_24h_btc_normalized")]
    pub trade_volume24_h_btc_normalized: Option<f64>,
}

// ---------------------------------------------
//  /exchanges/list
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeId {
    pub id: String,
    pub name: String,
}

// ---------------------------------------------
//  /exchanges/{id}/volume_chart
// ---------------------------------------------
pub type VolumeChartData = Vec<(i64, String)>;
