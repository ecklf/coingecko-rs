#![allow(missing_docs)]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::EnumIter;
use std::collections::HashMap;

use super::common::{
    CommunityData, CurrentPrice, DeveloperData, Image, Links, Localization, MarketCap,
    PublicInterestStats, Ticker, TotalVolume,
};

// ---------------------------------------------
//  /coins/list
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinsListItem {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub platforms: Option<HashMap<String, Option<String>>>,
}

// ---------------------------------------------
//  /coins/markets
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SparklineIn7D {
    pub price: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinsMarketItem {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: Option<f64>,
    pub market_cap: Option<f64>,
    pub market_cap_rank: Value,
    pub fully_diluted_valuation: Value,
    pub total_volume: Option<f64>,
    #[serde(rename = "high_24h")]
    pub high24_h: Option<f64>,
    #[serde(rename = "low_24h")]
    pub low24_h: Option<f64>,
    #[serde(rename = "price_change_24h")]
    pub price_change24_h: Option<f64>,
    #[serde(rename = "price_change_percentage_24h")]
    pub price_change_percentage24_h: Option<f64>,
    #[serde(rename = "market_cap_change_24h")]
    pub market_cap_change24_h: Option<f64>,
    #[serde(rename = "market_cap_change_percentage_24h")]
    pub market_cap_change_percentage24_h: Option<f64>,
    pub circulating_supply: Option<f64>,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub ath: Option<f64>,
    pub ath_change_percentage: Option<f64>,
    pub ath_date: Option<String>,
    pub atl: Option<f64>,
    pub atl_change_percentage: Option<f64>,
    pub atl_date: Option<String>,
    pub roi: Value,
    pub last_updated: Option<String>,
    #[serde(rename = "sparkline_in_7d")]
    pub sparkline_in7_d: Option<SparklineIn7D>,
    #[serde(rename = "price_change_percentage_14d_in_currency")]
    pub price_change_percentage14_d_in_currency: Option<f64>,
    #[serde(rename = "price_change_percentage_1h_in_currency")]
    pub price_change_percentage1_h_in_currency: Option<f64>,
    #[serde(rename = "price_change_percentage_1y_in_currency")]
    pub price_change_percentage1_y_in_currency: Option<f64>,
    #[serde(rename = "price_change_percentage_200d_in_currency")]
    pub price_change_percentage200_d_in_currency: Option<f64>,
    #[serde(rename = "price_change_percentage_24h_in_currency")]
    pub price_change_percentage24_h_in_currency: Option<f64>,
    #[serde(rename = "price_change_percentage_30d_in_currency")]
    pub price_change_percentage30_d_in_currency: Option<f64>,
    #[serde(rename = "price_change_percentage_7d_in_currency")]
    pub price_change_percentage7_d_in_currency: Option<f64>,
}

// ---------------------------------------------
//  /coins/{id}
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinsItem {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub asset_platform_id: Value,
    pub platforms: Option<HashMap<String, Option<String>>>,
    pub block_time_in_minutes: f64,
    pub hashing_algorithm: Value,
    pub categories: Vec<String>,
    pub public_notice: Value,
    pub additional_notices: Vec<Value>,
    pub localization: Option<Localization>,
    pub description: Description,
    pub links: Links,
    pub image: Image,
    pub country_origin: String,
    pub genesis_date: Value,
    pub contract_address: Option<String>,
    pub sentiment_votes_up_percentage: Value,
    pub sentiment_votes_down_percentage: Value,
    pub market_cap_rank: Value,
    pub coingecko_rank: Value,
    pub coingecko_score: Value,
    pub developer_score: Value,
    pub community_score: Value,
    pub liquidity_score: Value,
    pub public_interest_score: Value,
    pub market_data: Option<MarketData>,
    pub community_data: Option<CommunityData>,
    pub developer_data: Option<DeveloperData>,
    pub public_interest_stats: PublicInterestStats,
    pub status_updates: Vec<Value>,
    pub last_updated: String,
    pub tickers: Option<Vec<Ticker>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Description {
    pub en: Option<String>,
    pub de: Option<String>,
    pub es: Option<String>,
    pub fr: Option<String>,
    pub it: Option<String>,
    pub pl: Option<String>,
    pub ro: Option<String>,
    pub hu: Option<String>,
    pub nl: Option<String>,
    pub pt: Option<String>,
    pub sv: Option<String>,
    pub vi: Option<String>,
    pub tr: Option<String>,
    pub ru: Option<String>,
    pub ja: Option<String>,
    pub zh: Option<String>,
    #[serde(rename = "zh-tw")]
    pub zh_tw: Option<String>,
    pub ko: Option<String>,
    pub ar: Option<String>,
    pub th: Option<String>,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketData {
    pub current_price: CurrentPrice,
    pub total_value_locked: Value,
    pub mcap_to_tvl_ratio: Value,
    pub fdv_to_tvl_ratio: Value,
    pub roi: Value,
    pub ath: Ath,
    pub ath_change_percentage: AthChangePercentage,
    pub ath_date: AthDate,
    pub atl: Atl,
    pub atl_change_percentage: AtlChangePercentage,
    pub atl_date: AtlDate,
    pub market_cap: MarketCap,
    pub market_cap_rank: Value,
    pub fully_diluted_valuation: FullyDilutedValuation,
    pub total_volume: TotalVolume,
    #[serde(rename = "high_24h")]
    pub high24_h: High24H,
    #[serde(rename = "low_24h")]
    pub low24_h: Low24H,
    #[serde(rename = "price_change_24h")]
    pub price_change24_h: Option<f64>,
    #[serde(rename = "price_change_percentage_24h")]
    pub price_change_percentage24_h: Option<f64>,
    #[serde(rename = "price_change_percentage_7d")]
    pub price_change_percentage7_d: Option<f64>,
    #[serde(rename = "price_change_percentage_14d")]
    pub price_change_percentage14_d: Option<f64>,
    #[serde(rename = "price_change_percentage_30d")]
    pub price_change_percentage30_d: Option<f64>,
    #[serde(rename = "price_change_percentage_60d")]
    pub price_change_percentage60_d: Option<f64>,
    #[serde(rename = "price_change_percentage_200d")]
    pub price_change_percentage200_d: Option<f64>,
    #[serde(rename = "price_change_percentage_1y")]
    pub price_change_percentage1_y: Option<f64>,
    #[serde(rename = "market_cap_change_24h")]
    pub market_cap_change24_h: Option<f64>,
    #[serde(rename = "market_cap_change_percentage_24h")]
    pub market_cap_change_percentage24_h: Option<f64>,
    #[serde(rename = "price_change_24h_in_currency")]
    pub price_change24_h_in_currency: Option<PriceChange24HInCurrency>,
    #[serde(rename = "price_change_percentage_1h_in_currency")]
    pub price_change_percentage1_h_in_currency: Option<PriceChangePercentage1HInCurrency>,
    #[serde(rename = "price_change_percentage_24h_in_currency")]
    pub price_change_percentage24_h_in_currency: Option<PriceChangePercentage24HInCurrency>,
    #[serde(rename = "price_change_percentage_7d_in_currency")]
    pub price_change_percentage7_d_in_currency: Option<PriceChangePercentage7DInCurrency>,
    #[serde(rename = "price_change_percentage_14d_in_currency")]
    pub price_change_percentage14_d_in_currency: Option<PriceChangePercentage14DInCurrency>,
    #[serde(rename = "price_change_percentage_30d_in_currency")]
    pub price_change_percentage30_d_in_currency: Option<PriceChangePercentage30DInCurrency>,
    #[serde(rename = "price_change_percentage_60d_in_currency")]
    pub price_change_percentage60_d_in_currency: Option<PriceChangePercentage60DInCurrency>,
    #[serde(rename = "price_change_percentage_200d_in_currency")]
    pub price_change_percentage200_d_in_currency: Option<PriceChangePercentage200DInCurrency>,
    #[serde(rename = "price_change_percentage_1y_in_currency")]
    pub price_change_percentage1_y_in_currency: Option<PriceChangePercentage1YInCurrency>,
    #[serde(rename = "market_cap_change_24h_in_currency")]
    pub market_cap_change24_h_in_currency: Option<MarketCapChange24HInCurrency>,
    #[serde(rename = "market_cap_change_percentage_24h_in_currency")]
    pub market_cap_change_percentage24_h_in_currency:
        Option<MarketCapChangePercentage24HInCurrency>,
    pub total_supply: Value,
    pub max_supply: Value,
    pub circulating_supply: Value,
    #[serde(rename = "sparkline_7d")]
    pub sparkline7_d: Option<Sparkline7D>,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Coin {
    aed,
    ars,
    aud,
    bch,
    bdt,
    bhd,
    bmd,
    bnb,
    brl,
    btc,
    cad,
    chf,
    clp,
    cny,
    czk,
    dkk,
    dot,
    eos,
    eth,
    eur,
    gbp,
    hkd,
    huf,
    idr,
    ils,
    inr,
    jpy,
    krw,
    kwd,
    lkr,
    ltc,
    mmk,
    mxn,
    myr,
    ngn,
    nok,
    nzd,
    php,
    pkr,
    pln,
    rub,
    sar,
    sek,
    sgd,
    thb,
    twd,
    uah,
    usd,
    vef,
    vnd,
    xag,
    xau,
    xdr,
    xlm,
    xrp,
    yfi,
    zar,
    bits,
    link,
    sats,

    #[serde(rename = "try")]
    try_field,
}

pub type Ath = HashMap<Coin, Option<f64>>;
pub type AthChangePercentage = HashMap<Coin, Option<f64>>;
pub type AthDate = HashMap<Coin, Option<String>>;
pub type Atl = HashMap<Coin, Option<f64>>;
pub type AtlChangePercentage = HashMap<Coin, Option<f64>>;
pub type AtlDate = HashMap<Coin, Option<String>>;
pub type FullyDilutedValuation = HashMap<Coin, Option<f64>>;
pub type High24H = HashMap<Coin, Option<f64>>;
pub type Low24H = HashMap<Coin, Option<f64>>;
pub type PriceChange24HInCurrency = HashMap<Coin, Option<f64>>;
pub type PriceChangePercentage1HInCurrency = HashMap<Coin, Option<f64>>;
pub type PriceChangePercentage24HInCurrency = HashMap<Coin, Option<f64>>;
pub type PriceChangePercentage7DInCurrency = HashMap<Coin, Option<f64>>;
pub type PriceChangePercentage14DInCurrency = HashMap<Coin, Option<f64>>;
pub type PriceChangePercentage30DInCurrency = HashMap<Coin, Option<f64>>;
pub type PriceChangePercentage60DInCurrency = HashMap<Coin, Option<f64>>;
pub type PriceChangePercentage200DInCurrency = HashMap<Coin, Option<f64>>;
pub type PriceChangePercentage1YInCurrency = HashMap<Coin, Option<f64>>;
pub type MarketCapChange24HInCurrency = HashMap<Coin, Option<f64>>;
pub type MarketCapChangePercentage24HInCurrency = HashMap<Coin, Option<f64>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sparkline7D {
    pub price: Vec<f64>,
}

// ---------------------------------------------
//  /coins/{id}/history
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct History {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub localization: Option<Localization>,
    pub image: Image,
    pub market_data: HistoryMarketData,
    pub community_data: CommunityData,
    pub developer_data: DeveloperData,
    pub public_interest_stats: PublicInterestStats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryMarketData {
    pub current_price: CurrentPrice,
    pub market_cap: MarketCap,
    pub total_volume: TotalVolume,
}

// ---------------------------------------------
//  /coins/{id}/market_chart
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketChart {
    pub prices: Vec<Vec<f64>>,
    pub market_caps: Vec<Vec<f64>>,
    pub total_volumes: Vec<Vec<f64>>,
}

// ---------------------------------------------
//  /coins/{id}/contract/{contract_address}
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contract {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub asset_platform_id: String,
    pub platforms: Option<HashMap<String, Option<String>>>,
    pub block_time_in_minutes: i64,
    pub hashing_algorithm: ::serde_json::Value,
    pub categories: Vec<String>,
    pub public_notice: ::serde_json::Value,
    pub additional_notices: Vec<::serde_json::Value>,
    pub localization: Localization,
    pub description: Description,
    pub links: Links,
    pub image: Image,
    pub country_origin: String,
    pub genesis_date: ::serde_json::Value,
    pub contract_address: String,
    pub sentiment_votes_up_percentage: f64,
    pub sentiment_votes_down_percentage: f64,
    pub market_cap_rank: i64,
    pub coingecko_rank: i64,
    pub coingecko_score: f64,
    pub developer_score: i64,
    pub community_score: f64,
    pub liquidity_score: f64,
    pub public_interest_score: f64,
    pub market_data: MarketData,
    pub community_data: CommunityData,
    pub developer_data: DeveloperData,
    pub public_interest_stats: PublicInterestStats,
    pub status_updates: Vec<::serde_json::Value>,
    pub last_updated: String,
    pub tickers: Vec<Ticker>,
}

// ---------------------------------------------
//  /coins/categories/list
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryId {
    pub category_id: String,
    pub name: String,
}

// ---------------------------------------------
//  /coins/categories
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub market_cap: f64,
    #[serde(rename = "market_cap_change_24h")]
    pub market_cap_change24_h: f64,
    #[serde(rename = "volume_24h")]
    pub volume24_h: f64,
    pub updated_at: String,
}
