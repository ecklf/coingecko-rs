use serde::{Deserialize, Serialize};

/// Market display order for `coins_markets`
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum MarketsOrder {
    /// Marketcap descending
    MarketCapDesc,
    /// Marketcap ascending
    MarketCapAsc,
    /// Coingecko descending
    GeckoDesc,
    /// Coingecko ascending
    GeckoAsc,
    /// Volume descending
    VolumeDesc,
    /// Volume ascending
    VolumeAsc,
    /// ID descending
    IdDesc,
    /// ID ascending
    IdAsc,
}

/// Price change percentage times for `coins_markets`
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum PriceChangePercentage {
    /// 1h
    OneHour,
    /// 24h
    TwentyFourHours,
    /// 7d
    SevenDays,
    /// 14d
    FourteenDays,
    /// 30d
    ThirtyDays,
    /// 200d
    TwoHundredDays,
    /// 1y
    OneYear,
}

/// Tickers order for `coin_tickers` and `exchange_tickers`
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum TickersOrder {
    /// Trust Score ascending
    TrustScoreAsc,
    /// Trust Score descending
    TrustScoreDesc,
    /// Volume descending
    VolumeDesc,
}

/// Ohlc times for `coin_ohlc`
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum OhlcDays {
    /// 1d
    OneDay,
    /// 7d
    SevenDays,
    /// 14d
    FourteenDays,
    /// 30d
    ThirtyDays,
    /// 90d
    NinetyDays,
    /// 180d
    OneHundredEightyDays,
    /// 365
    ThreeHundredSixtyFiveDays,
}

/// Tickers to include for `derivatives` and `derivatives_exchange`
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum DerivativesIncludeTickers {
    /// All tickers
    All,
    /// Unexpired tickers
    Unexpired,
}

/// Order of exchanges for `derivative_exchanges`
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum DerivativeExchangeOrder {
    /// Name ascending
    NameAsc,
    /// Name descending
    NameDesc,
    /// Open interest BTC ascending
    OpenInterestBtcAsc,
    /// Open interest BTC descending
    OpenInterestBtcDesc,
    /// 24h BTC trade volume ascending
    TradeVolume24hBtcAsc,
    /// 24h BTC trade volume descending
    TradeVolume24hBtcDesc,
}

/// IDs for coins held in treasury for `companies`
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum CompaniesCoinId {
    /// Bitcoin
    Bitcoin,
    /// Ethereum
    Ethereum,
}
