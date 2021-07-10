pub enum MarketsOrder {
    MarketCapDesc,
    MarketCapAsc,
    GeckoDesc,
    GeckoAsc,
    VolumeDesc,
    VolumeAsc,
    IdDesc,
    IdAsc,
}

pub enum PriceChangePercentage {
    OneHour,
    TwentyFourHours,
    SevenDays,
    FourteenDays,
    ThirtyDays,
    TwoHundredDays,
    OneYear,
}

pub enum TickersOrder {
    TrustScoreAsc,
    TrustScoreDesc,
    VolumeDesc,
}

pub enum OhlcDays {
    OneDay,
    SevenDays,
    FourteenDays,
    ThirtyDays,
    NinetyDays,
    OneHundredEightyDays,
    ThreeHundredSixtyFiveDays,
}

pub enum DerivativesIncludeTickers {
    All,
    Unexpired,
}

pub enum DerivativeExchangeOrder {
    NameAsc,
    NameDesc,
    OpenInterestBtcAsc,
    OpenInterestBtcDesc,
    TradeVolume24hBtcAsc,
    TradeVolume24hBtcDesc,
}

pub enum CompaniesCoinId {
    Bitcoin,
    Ethereum,
}
