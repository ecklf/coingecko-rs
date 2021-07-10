use serde::{Deserialize, Serialize};

// ---------------------------------------------
//  /indexes
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    pub name: String,
    pub id: String,
    pub market: String,
    pub last: Option<f64>,
    pub is_multi_asset_composite: Option<bool>,
}

// ---------------------------------------------
//  /index/{market_id}/{id}
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketIndex {
    pub name: String,
    pub market: String,
    pub last: Option<f64>,
    pub is_multi_asset_composite: Option<bool>,
}

// ---------------------------------------------
//  /indexes/list
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexId {
    pub name: String,
    pub id: String,
}