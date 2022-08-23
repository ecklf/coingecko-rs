#![allow(clippy::too_many_arguments)]
use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use reqwest::Error;
use serde::de::DeserializeOwned;

use crate::params::{
    CompaniesCoinId, DerivativeExchangeOrder, DerivativesIncludeTickers, MarketsOrder, OhlcDays,
    PriceChangePercentage, TickersOrder,
};

use crate::response::{
    asset_platforms::AssetPlatform,
    coins::{
        Category, CategoryId, CoinsItem, CoinsListItem, CoinsMarketItem, Contract, History,
        MarketChart,
    },
    common::{StatusUpdates, Tickers},
    companies::CompaniesPublicTreasury,
    derivatives::{Derivative, DerivativeExchangeId},
    events::Events,
    events::{EventCountries, EventTypes},
    exchange_rates::ExchangeRates,
    exchanges::VolumeChartData,
    exchanges::{Exchange, ExchangeId},
    finance::{FinancePlatform, FinanceProduct},
    global::{Global, GlobalDefi},
    indexes::Index,
    indexes::{IndexId, MarketIndex},
    ping::SimplePing,
    simple::{Price, SupportedVsCurrencies},
    trending::Trending,
};

/// CoinGecko client
pub struct CoinGeckoClient {
    host: &'static str,
}

/// Creates a new CoinGeckoClient with host https://api.coingecko.com/api/v3
///
/// # Examples
///
/// ```rust
/// use coingecko_rs::CoinGeckoClient;
/// let client = CoinGeckoClient::default();
/// ```
impl Default for CoinGeckoClient {
    fn default() -> Self {
        CoinGeckoClient::new("https://api.coingecko.com/api/v3")
    }
}

impl CoinGeckoClient {
    /// Creates a new CoinGeckoClient client with a custom host url
    ///
    /// # Examples
    ///
    /// ```rust
    /// use coingecko_rs::CoinGeckoClient;
    /// let client = CoinGeckoClient::new("https://some.url");
    /// ```
    pub fn new(host: &'static str) -> Self {
        CoinGeckoClient { host }
    }

    async fn get<R: DeserializeOwned>(&self, endpoint: &str) -> Result<R, Error> {
        reqwest::get(format!("{host}/{ep}", host = self.host, ep = endpoint))
            .await?
            .json()
            .await
    }

    /// Check API server status
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.ping().await;
    /// }
    /// ```
    pub async fn ping(&self) -> Result<SimplePing, Error> {
        self.get("/ping").await
    }

    /// Get the current price of any cryptocurrencies in any other supported currencies that you need
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.price(&["bitcoin", "ethereum"], &["usd"], true, true, true, true).await;
    /// }
    /// ```
    pub async fn price<Id: AsRef<str>, Curr: AsRef<str>>(
        &self,
        ids: &[Id],
        vs_currencies: &[Curr],
        include_market_cap: bool,
        include_24hr_vol: bool,
        include_24hr_change: bool,
        include_last_updated_at: bool,
    ) -> Result<HashMap<String, Price>, Error> {
        let ids = ids.iter().map(AsRef::as_ref).collect::<Vec<_>>();
        let vs_currencies = vs_currencies.iter().map(AsRef::as_ref).collect::<Vec<_>>();
        let req = format!("/simple/price?ids={}&vs_currencies={}&include_market_cap={}&include_24hr_vol={}&include_24hr_change={}&include_last_updated_at={}", ids.join("%2C"), vs_currencies.join("%2C"), include_market_cap, include_24hr_vol, include_24hr_change, include_last_updated_at);
        self.get(&req).await
    }

    /// Get current price of tokens (using contract addresses) for a given platform in any other currency that you need
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///     let uniswap_contract = "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984";
    ///
    ///     client.token_price(
    ///         "ethereum",
    ///         &[uniswap_contract],
    ///         &["usd"],
    ///         true,
    ///         true,
    ///         true,
    ///         true,
    ///     ).await;
    /// }
    /// ```
    pub async fn token_price<Addr: AsRef<str>, Curr: AsRef<str>>(
        &self,
        id: &str,
        contract_addresses: &[Addr],
        vs_currencies: &[Curr],
        include_market_cap: bool,
        include_24hr_vol: bool,
        include_24hr_change: bool,
        include_last_updated_at: bool,
    ) -> Result<HashMap<String, Price>, Error> {
        let contract_addresses = contract_addresses
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>();
        let vs_currencies = vs_currencies.iter().map(AsRef::as_ref).collect::<Vec<_>>();
        let req = format!("/simple/token_price/{}?contract_addresses={}&vs_currencies={}&include_market_cap={}&include_24hr_vol={}&include_24hr_change={}&include_last_updated_at={}", id, contract_addresses.join("%2C"), vs_currencies.join("%2C"), include_market_cap, include_24hr_vol, include_24hr_change, include_last_updated_at);
        self.get(&req).await
    }

    /// Get list of supported_vs_currencies
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.supported_vs_currencies().await;
    /// }
    /// ```
    pub async fn supported_vs_currencies(&self) -> Result<SupportedVsCurrencies, Error> {
        self.get("/simple/supported_vs_currencies").await
    }

    /// List all supported coins id, name and symbol (no pagination required)
    ///
    /// Use this to obtain all the coins’ id in order to make API calls
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.coins_list(true).await;
    /// }
    /// ```
    pub async fn coins_list(&self, include_platform: bool) -> Result<Vec<CoinsListItem>, Error> {
        let req = format!("/coins/list?include_platform={}", include_platform);
        self.get(&req).await
    }

    /// List all supported coins price, market cap, volume, and market related data
    ///
    /// Use this to obtain all the coins market data (price, market cap, volume)
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::{
    ///         params::{MarketsOrder, PriceChangePercentage},
    ///         CoinGeckoClient,
    ///     };
    ///     let client = CoinGeckoClient::default();
    ///     
    ///     client.coins_markets(
    ///         "usd",
    ///         &["bitcoin"],
    ///         None,
    ///         MarketsOrder::GeckoDesc,
    ///         1,
    ///         0,
    ///         true,
    ///         &[
    ///             PriceChangePercentage::OneHour,
    ///             PriceChangePercentage::TwentyFourHours,
    ///             PriceChangePercentage::SevenDays,
    ///             PriceChangePercentage::FourteenDays,
    ///             PriceChangePercentage::ThirtyDays,
    ///             PriceChangePercentage::OneYear,
    ///         ],
    ///     ).await;
    /// }
    /// ```
    pub async fn coins_markets<Id: AsRef<str>>(
        &self,
        vs_currency: &str,
        ids: &[Id],
        category: Option<&str>,
        order: MarketsOrder,
        per_page: i64,
        page: i64,
        sparkline: bool,
        price_change_percentage: &[PriceChangePercentage],
    ) -> Result<Vec<CoinsMarketItem>, Error> {
        let ids = ids.iter().map(AsRef::as_ref).collect::<Vec<_>>();

        let category = match category {
            Some(c) => format!("&category={}", c),
            _ => String::from(""),
        };

        let order = match order {
            MarketsOrder::MarketCapDesc => "market_cap_desc",
            MarketsOrder::MarketCapAsc => "market_cap_asc",
            MarketsOrder::GeckoDesc => "gecko_desc",
            MarketsOrder::GeckoAsc => "gecko_asc",
            MarketsOrder::VolumeDesc => "volume_desc",
            MarketsOrder::VolumeAsc => "volume_asc",
            MarketsOrder::IdDesc => "id_desc",
            MarketsOrder::IdAsc => "id_asc",
        };

        let price_change_percentage = price_change_percentage.iter().fold(
            Vec::with_capacity(price_change_percentage.len()),
            |mut acc, x| {
                let current = match *x {
                    PriceChangePercentage::OneHour => "1h",
                    PriceChangePercentage::TwentyFourHours => "24h",
                    PriceChangePercentage::SevenDays => "7d",
                    PriceChangePercentage::FourteenDays => "14d",
                    PriceChangePercentage::ThirtyDays => "30d",
                    PriceChangePercentage::TwoHundredDays => "200d",
                    PriceChangePercentage::OneYear => "1y",
                };

                acc.push(current);
                acc
            },
        );

        let req = format!("/coins/markets?vs_currency={}&ids={}{}&order={}&per_page={}&page={}&sparkline={}&price_change_percentage={}", vs_currency, ids.join("%2C"), category, order, per_page, page, sparkline, price_change_percentage.join("%2C"));
        self.get(&req).await
    }

    /// Get current data (name, price, market, ... including exchange tickers) for a coin
    ///
    /// **IMPORTANT**:
    /// Ticker object is limited to 100 items, to get more tickers, use coin_tickers
    /// Ticker is_stale is true when ticker that has not been updated/unchanged from the exchange for a while.
    /// Ticker is_anomaly is true if ticker’s price is outliered by our system.
    /// You are responsible for managing how you want to display these information (e.g. footnote, different background, change opacity, hide)
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///     
    ///     client.coin("bitcoin", true, true, true, true, true, true).await;
    /// }
    /// ```
    pub async fn coin(
        &self,
        id: &str,
        localization: bool,
        tickers: bool,
        market_data: bool,
        community_data: bool,
        developer_data: bool,
        sparkline: bool,
    ) -> Result<CoinsItem, Error> {
        let req = format!("/coins/{}?localization={}&tickers={}&market_data={}&community_data={}&developer_data={}&sparkline={}", id, localization, tickers, market_data, community_data, developer_data, sparkline);
        self.get(&req).await
    }

    /// Get coin tickers (paginated to 100 items)
    ///
    /// **IMPORTANT**:
    /// Ticker is_stale is true when ticker that has not been updated/unchanged from the exchange for a while.
    /// Ticker is_anomaly is true if ticker’s price is outliered by our system.
    /// You are responsible for managing how you want to display these information (e.g. footnote, different background, change opacity, hide)
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::{params::TickersOrder, CoinGeckoClient};
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.coin_tickers::<&str>("bitcoin", None, true, 1, TickersOrder::VolumeDesc, true).await;
    /// }
    /// ```
    pub async fn coin_tickers<Ex: AsRef<str>>(
        &self,
        id: &str,
        exchange_ids: Option<&[Ex]>,
        include_exchange_logo: bool,
        page: i64,
        order: TickersOrder,
        depth: bool,
    ) -> Result<Tickers, Error> {
        let order = match order {
            TickersOrder::TrustScoreAsc => "trust_score_asc",
            TickersOrder::TrustScoreDesc => "trust_score_desc",
            TickersOrder::VolumeDesc => "volume_desc",
        };

        let req = match exchange_ids {
            Some(e_ids) => {
                let e_ids = e_ids.iter().map(AsRef::as_ref).collect::<Vec<_>>();
                format!("/coins/{}/tickers?exchange_ids={}&include_exchange_logo={}&page={}&order={}&depth={}", id, e_ids.join("%2C"), include_exchange_logo, &page, order, depth)
            }
            None => format!(
                "/coins/{}/tickers?include_exchange_logo={}&page={}&order={}&depth={}",
                id, include_exchange_logo, &page, order, depth
            ),
        };

        self.get(&req).await
    }

    /// Get historical data (name, price, market, stats) at a given date for a coin
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use chrono::NaiveDate;
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.coin_history("bitcoin", NaiveDate::from_ymd(2017, 12, 30), true).await;
    /// }
    /// ```
    pub async fn coin_history(
        &self,
        id: &str,
        date: NaiveDate,
        localization: bool,
    ) -> Result<History, Error> {
        let formatted_date = date.format("%d-%m-%Y").to_string();

        let req = format!(
            "/coins/{}/history?date={}&localization={}",
            id, formatted_date, localization
        );
        self.get(&req).await
    }

    /// Get historical market data include price, market cap, and 24h volume (granularity auto)
    ///
    /// **Minutely data will be used for duration within 1 day, Hourly data will be used for duration between 1 day and 90 days, Daily data will be used for duration above 90 days.**
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.coin_market_chart("bitcoin", "usd", 1, true).await;
    /// }
    /// ```
    pub async fn coin_market_chart(
        &self,
        id: &str,
        vs_currency: &str,
        days: &str,
        use_daily_interval: bool,
    ) -> Result<MarketChart, Error> {
        let req = match use_daily_interval {
            true => format!(
                "/coins/{}/market_chart?vs_currency={}&days={}",
                id, vs_currency, days
            ),
            false => format!(
                "/coins/{}/market_chart?vs_currency={}&days={}&interval=daily",
                id, vs_currency, days
            ),
        };

        self.get(&req).await
    }

    /// Get historical market data include price, market cap, and 24h volume within a range of timestamp (granularity auto)
    ///
    /// - **Data granularity is automatic (cannot be adjusted)**
    /// - **1 day from query time = 5 minute interval data**
    /// - **1 - 90 days from query time = hourly data**
    /// - **above 90 days from query time = daily data (00:00 UTC)**
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use chrono::NaiveDate;
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     let from = NaiveDate::from_ymd(2014, 2, 16).and_hms(19, 0, 32);
    ///     let to = NaiveDate::from_ymd(2015, 1, 30).and_hms(0, 20, 32);
    ///
    ///     client.coin_market_chart_range("bitcoin", "usd", from, to).await;
    /// }
    /// ```
    pub async fn coin_market_chart_range(
        &self,
        id: &str,
        vs_currency: &str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<MarketChart, Error> {
        let from_unix_timestamp = from.timestamp();
        let to_unix_timestamp = to.timestamp();

        let req = format!(
            "/coins/{}/market_chart/range?vs_currency={}&from={}&to={}",
            id, vs_currency, from_unix_timestamp, to_unix_timestamp
        );
        self.get(&req).await
    }

    /// Get status updates for a given coin
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///     client.coin_status_updates("qtum", 10, 1).await;
    /// }
    /// ```
    pub async fn coin_status_updates(
        &self,
        id: &str,
        per_page: i64,
        page: i64,
    ) -> Result<StatusUpdates, Error> {
        let req = format!(
            "/coins/{}/status_updates?per_page={}&page={}",
            id, per_page, page,
        );

        self.get(&req).await
    }

    /// Get coin's OHLC
    ///
    /// Candle’s body:
    /// 1 - 2 days: 30 minutes
    /// 3 - 30 days: 4 hours
    /// 31 and before: 4 days
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::{params::OhlcDays, CoinGeckoClient};
    ///     let client = CoinGeckoClient::default();
    ///     client.coin_ohlc("bitcoin", "usd", OhlcDays::OneDay).await;
    /// }
    /// ```
    pub async fn coin_ohlc(
        &self,
        id: &str,
        vs_currency: &str,
        days: OhlcDays,
    ) -> Result<Vec<Vec<f64>>, Error> {
        let days = match days {
            OhlcDays::OneDay => 1,
            OhlcDays::SevenDays => 7,
            OhlcDays::FourteenDays => 14,
            OhlcDays::ThirtyDays => 30,
            OhlcDays::NinetyDays => 90,
            OhlcDays::OneHundredEightyDays => 180,
            OhlcDays::ThreeHundredSixtyFiveDays => 365,
        };

        let req = format!(
            "/coins/{}/ohlc?vs_currency={}&days={}",
            id, vs_currency, days
        );
        self.get(&req).await
    }

    /// Get coin info from contract address
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///     let uniswap_contract = "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984";
    ///
    ///     client.contract("ethereum", &uniswap_contract).await;
    /// }
    /// ```
    pub async fn contract(&self, id: &str, contract_address: &str) -> Result<Contract, Error> {
        let req = format!("/coins/{}/contract/{}", id, contract_address);
        self.get(&req).await
    }

    /// Get historical market data include price, market cap, and 24h volume (granularity auto)
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///     let uniswap_contract = "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984";
    ///
    ///     client.contract_market_chart("ethereum", &uniswap_contract, "usd", 1).await;
    /// }
    /// ```
    pub async fn contract_market_chart(
        &self,
        id: &str,
        contract_address: &str,
        vs_currency: &str,
        days: i64,
    ) -> Result<MarketChart, Error> {
        let req = format!(
            "/coins/{}/contract/{}/market_chart/?vs_currency={}&days={}",
            id, contract_address, vs_currency, days
        );
        self.get(&req).await
    }

    /// Get historical market data include price, market cap, and 24h volume within a range of timestamp (granularity auto)
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use chrono::NaiveDate;
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///     let uniswap_contract = "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984";
    ///
    ///     let from = NaiveDate::from_ymd(2014, 2, 16).and_hms(19, 0, 32);
    ///     let to = NaiveDate::from_ymd(2015, 1, 30).and_hms(0, 20, 32);
    ///
    ///     client.contract_market_chart_range("ethereum", &uniswap_contract, "usd", from, to).await;
    /// }
    /// ```
    pub async fn contract_market_chart_range(
        &self,
        id: &str,
        contract_address: &str,
        vs_currency: &str,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<MarketChart, Error> {
        let from_unix_timestamp = from.timestamp();
        let to_unix_timestamp = to.timestamp();

        let req = format!(
            "/coins/{}/contract/{}/market_chart/range?vs_currency={}&from={}&to={}",
            id, contract_address, vs_currency, from_unix_timestamp, to_unix_timestamp
        );
        self.get(&req).await
    }

    /// List all asset platforms (Blockchain networks)
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.asset_platforms().await;
    /// }
    /// ```
    pub async fn asset_platforms(&self) -> Result<Vec<AssetPlatform>, Error> {
        self.get("/asset_platforms").await
    }

    /// List all categories
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.categories_list().await;
    /// }
    /// ```
    pub async fn categories_list(&self) -> Result<Vec<CategoryId>, Error> {
        self.get("/coins/categories/list").await
    }

    /// List all categories with market data
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.categories().await;
    /// }
    /// ```
    pub async fn categories(&self) -> Result<Vec<Category>, Error> {
        self.get("/coins/categories").await
    }

    /// List all exchanges
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.exchanges(10, 1).await;
    /// }
    /// ```
    pub async fn exchanges(&self, per_page: i64, page: i64) -> Result<Vec<Exchange>, Error> {
        let req = format!("/exchanges?per_page={}&page={}", per_page, page);
        self.get(&req).await
    }

    /// List all supported markets id and name (no pagination required)
    ///
    /// Use this to obtain all the markets’ id in order to make API calls
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.exchanges_list().await;
    /// }
    /// ```
    pub async fn exchanges_list(&self) -> Result<Vec<ExchangeId>, Error> {
        self.get("/exchanges/list").await
    }

    /// Get exchange volume in BTC and top 100 tickers only
    ///
    /// **IMPORTANT**:
    /// Ticker object is limited to 100 items, to get more tickers, use exchange_tickers
    /// Ticker is_stale is true when ticker that has not been updated/unchanged from the exchange for a while.
    /// Ticker is_anomaly is true if ticker’s price is outliered by our system.
    /// You are responsible for managing how you want to display these information (e.g. footnote, different background, change opacity, hide)
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.exchange("binance").await;
    /// }
    /// ```
    pub async fn exchange(&self, id: &str) -> Result<Exchange, Error> {
        let req = format!("/exchanges/{}", id);
        self.get(&req).await
    }

    /// Get exchange tickers (paginated)
    ///
    /// **IMPORTANT**:
    /// Ticker is_stale is true when ticker that has not been updated/unchanged from the exchange for a while.
    /// Ticker is_anomaly is true if ticker’s price is outliered by our system.
    /// You are responsible for managing how you want to display these information (e.g. footnote, different background, change opacity, hide)
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::{params::TickersOrder, CoinGeckoClient};
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.exchange_tickers("binance", Some(&["btc"]), true, 1, TickersOrder::TrustScoreAsc, true).await;
    /// }
    /// ```
    pub async fn exchange_tickers<CoinId: AsRef<str>>(
        &self,
        id: &str,
        coin_ids: Option<&[CoinId]>,
        include_exchange_logo: bool,
        page: i64,
        order: TickersOrder,
        depth: bool,
    ) -> Result<Tickers, Error> {
        let order = match order {
            TickersOrder::TrustScoreAsc => "trust_score_asc",
            TickersOrder::TrustScoreDesc => "trust_score_desc",
            TickersOrder::VolumeDesc => "volume_desc",
        };

        let req = match coin_ids {
            Some(c_ids) => {
                let c_ids = c_ids.iter().map(AsRef::as_ref).collect::<Vec<_>>();
                format!("/exchanges/{}/tickers?coin_ids={}&include_exchange_logo={}&page={}&order={}&depth={}", id, c_ids.join("%2C"), include_exchange_logo, &page, order, depth)
            }
            None => format!(
                "/exchanges/{}/tickers?include_exchange_logo={}&page={}&order={}&depth={}",
                id, include_exchange_logo, &page, order, depth
            ),
        };

        self.get(&req).await
    }

    /// Get status updates for a given exchange
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.exchange_status_updates("binance", 10, 1).await;
    /// }
    /// ```
    pub async fn exchange_status_updates(
        &self,
        id: &str,
        per_page: i64,
        page: i64,
    ) -> Result<StatusUpdates, Error> {
        let req = format!(
            "/exchanges/{}/status_updates?per_page={}&page={}",
            id, per_page, page,
        );

        self.get(&req).await
    }

    /// Get volume_chart data for a given exchange
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.exchange_volume_chart("binance", 1).await;
    /// }
    /// ```
    pub async fn exchange_volume_chart(
        &self,
        id: &str,
        days: i64,
    ) -> Result<Vec<VolumeChartData>, Error> {
        let req = format!("/exchanges/{}/volume_chart?days={}", id, days);
        self.get(&req).await
    }

    /// List all finance platforms
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.finance_platforms(10, 1).await;
    /// }
    /// ```
    pub async fn finance_platforms(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<FinancePlatform>, Error> {
        let req = format!("/finance_platforms?per_page={}&page={}", per_page, page,);

        self.get(&req).await
    }

    /// List all finance products
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.finance_products(10, 1).await;
    /// }
    /// ```
    pub async fn finance_products(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<FinanceProduct>, Error> {
        let req = format!("/finance_products?per_page={}&page={}", per_page, page,);

        self.get(&req).await
    }

    /// List all market indexes
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.indexes(10, 1).await;
    /// }
    /// ```
    pub async fn indexes(&self, per_page: i64, page: i64) -> Result<Vec<Index>, Error> {
        let req = format!("/indexes?per_page={}&page={}", per_page, page,);

        self.get(&req).await
    }

    /// Get market index by market id and index id
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.indexes_market_id("binance_futures", "BTC").await;
    /// }
    /// ```
    pub async fn indexes_market_id(&self, market_id: &str, id: &str) -> Result<MarketIndex, Error> {
        let req = format!("/indexes/{}/{}", market_id, id);
        self.get(&req).await
    }

    /// List market indexes id and name
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.indexes_list().await;
    /// }
    /// ```
    pub async fn indexes_list(&self) -> Result<Vec<IndexId>, Error> {
        self.get("/indexes/list").await
    }

    /// List all derivative tickers
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::{params::DerivativesIncludeTickers, CoinGeckoClient};
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.derivatives(Some(DerivativesIncludeTickers::All)).await;
    /// }
    /// ```
    pub async fn derivatives(
        &self,
        include_tickers: Option<DerivativesIncludeTickers>,
    ) -> Result<Vec<Derivative>, Error> {
        let include_tickers = match include_tickers {
            Some(ic_enum) => match ic_enum {
                DerivativesIncludeTickers::All => "all",
                DerivativesIncludeTickers::Unexpired => "unexpired",
            },
            None => "unexpired",
        };

        let req = format!("/derivatives?include_tickers={}", include_tickers);
        self.get(&req).await
    }

    /// List all derivative exchanges
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::{params::DerivativeExchangeOrder, CoinGeckoClient};
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.derivative_exchanges(DerivativeExchangeOrder::NameAsc, 10, 1).await;
    /// }
    /// ```
    pub async fn derivative_exchanges(
        &self,
        order: DerivativeExchangeOrder,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<Derivative>, Error> {
        let order = match order {
            DerivativeExchangeOrder::NameAsc => "name_asc",
            DerivativeExchangeOrder::NameDesc => "name_desc",
            DerivativeExchangeOrder::OpenInterestBtcAsc => "open_interest_btc_asc",
            DerivativeExchangeOrder::OpenInterestBtcDesc => "open_interest_btc_desc",
            DerivativeExchangeOrder::TradeVolume24hBtcAsc => "trade_volume_24h_btc_asc",
            DerivativeExchangeOrder::TradeVolume24hBtcDesc => "trade_volume_24h_btc_desc",
        };

        let req = format!(
            "/derivatives/exchanges?order={}&per_page={}&page={}",
            order, per_page, page
        );
        self.get(&req).await
    }

    /// Show derivative exchange data
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::{params::DerivativesIncludeTickers, CoinGeckoClient};
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.derivatives_exchange("bitmex", Some(DerivativesIncludeTickers::All)).await;
    /// }
    /// ```
    pub async fn derivatives_exchange(
        &self,
        id: &str,
        include_tickers: Option<DerivativesIncludeTickers>,
    ) -> Result<Vec<Derivative>, Error> {
        let include_tickers = match include_tickers {
            Some(ic_enum) => match ic_enum {
                DerivativesIncludeTickers::All => "all",
                DerivativesIncludeTickers::Unexpired => "unexpired",
            },
            None => "unexpired",
        };

        let req = format!(
            "/derivatives/exchanges/{}?include_tickers={}",
            id, include_tickers
        );
        self.get(&req).await
    }

    /// List all derivative exchanges name and identifier
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.derivative_exchanges_list().await;
    /// }
    /// ```
    pub async fn derivative_exchanges_list(&self) -> Result<Vec<DerivativeExchangeId>, Error> {
        self.get("/derivatives/exchanges/list").await
    }

    /// List all status_updates with data (description, category, created_at, user, user_title and pin)
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.status_updates(Some("general"), Some("coin"), 10, 1).await;
    /// }
    /// ```
    pub async fn status_updates(
        &self,
        category: Option<&str>,
        project_type: Option<&str>,
        per_page: i64,
        page: i64,
    ) -> Result<StatusUpdates, Error> {
        let mut params: Vec<String> = Vec::with_capacity(4);

        if let Some(c) = category {
            params.push(format!("category={}", c));
        }

        if let Some(t) = project_type {
            params.push(format!("project_type={}", t));
        }

        params.push(per_page.to_string());
        params.push(page.to_string());

        let req = format!("/status_updates?{}", params.join("&"));

        self.get(&req).await
    }

    /// Get events, paginated by 100
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use chrono::NaiveDate;
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     let from = NaiveDate::from_ymd(2021, 10, 7);
    ///     let to = NaiveDate::from_ymd(2022, 10, 7);
    ///
    ///     client.events(Some("HK"), Some("Event"), 1, true, from, to).await;
    /// }
    /// ```
    pub async fn events(
        &self,
        country_code: Option<&str>,
        event_type: Option<&str>,
        page: i64,
        upcoming_events_only: bool,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<Events, Error> {
        let mut params: Vec<String> = Vec::with_capacity(2);

        if let Some(c) = country_code {
            params.push(format!("country_code={}", c));
        }

        if let Some(t) = event_type {
            params.push(format!("type={}", t));
        }

        let from_date = from_date.format("%Y-%m-%d").to_string();
        let to_date = to_date.format("%Y-%m-%d").to_string();

        let req = format!(
            "/events?{}&page={}&upcoming_events_only={}&from_date={}&to_date={}",
            params.join("&"),
            page,
            upcoming_events_only,
            from_date,
            to_date,
        );

        self.get(&req).await
    }

    /// Get list of event countries
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.event_countries().await;
    /// }
    /// ```
    pub async fn event_countries(&self) -> Result<EventCountries, Error> {
        self.get("/events/types").await
    }

    /// Get list of event types
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.event_types().await;
    /// }
    /// ```
    pub async fn event_types(&self) -> Result<EventTypes, Error> {
        self.get("/events/types").await
    }

    /// Get BTC-to-Currency exchange rates
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.exchange_rates().await;
    /// }
    /// ```
    pub async fn exchange_rates(&self) -> Result<ExchangeRates, Error> {
        self.get("/exchange_rates").await
    }

    /// Top-7 trending coins on CoinGecko as searched by users in the last 24 hours (Ordered by most popular first)
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.trending().await;
    /// }
    /// ```
    pub async fn trending(&self) -> Result<Trending, Error> {
        self.get("/search/trending").await
    }

    /// Get cryptocurrency global data
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.global().await;
    /// }
    /// ```
    pub async fn global(&self) -> Result<Global, Error> {
        self.get("/global").await
    }

    /// Get Top 100 Cryptocurrency Global Eecentralized Finance(defi) data
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::CoinGeckoClient;
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.global_defi().await;
    /// }
    /// ```
    pub async fn global_defi(&self) -> Result<GlobalDefi, Error> {
        self.get("/global/decentralized_finance_defi").await
    }

    /// Get public companies bitcoin or ethereum holdings (Ordered by total holdings descending)
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     use coingecko_rs::{params::CompaniesCoinId, CoinGeckoClient};
    ///     let client = CoinGeckoClient::default();
    ///
    ///     client.companies(CompaniesCoinId::Bitcoin).await;
    /// }
    /// ```
    pub async fn companies(
        &self,
        coin_id: CompaniesCoinId,
    ) -> Result<CompaniesPublicTreasury, Error> {
        let req = match coin_id {
            CompaniesCoinId::Bitcoin => "/companies/public_treasury/bitcoin".to_string(),
            CompaniesCoinId::Ethereum => "/companies/public_treasury/ethereum".to_string(),
        };

        self.get(&req).await
    }
}
