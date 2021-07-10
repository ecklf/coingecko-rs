use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------
//  /search/trending
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trending {
    pub coins: Vec<TrendingCoin>,
    pub exchanges: Vec<Value>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendingCoin {
    pub item: TrendingCoinMarketData,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendingCoinMarketData {
    pub id: String,
    pub coin_id: f64,
    pub name: String,
    pub symbol: String,
    pub market_cap_rank: f64,
    pub thumb: String,
    pub small: String,
    pub large: String,
    pub slug: String,
    pub price_btc: f64,
    pub score: f64,
}
