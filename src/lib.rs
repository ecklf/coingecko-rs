#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_qualifications)]

//! Rust library for the CoinGecko API.
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! coingecko = "1.0.0"
//! ```

/// Client module
mod client;
/// CoinGecko API Parameters
pub mod params;
/// Response structs for API requests
pub mod response;
/// CoinGecko Client
pub use crate::client::{CoinGeckoClient, COINGECKO_API_DEMO_URL};

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use crate::{
        params::{MarketsOrder, OhlcDays, PriceChangePercentage, TickersOrder},
        CoinGeckoClient,
    };
    use chrono::NaiveDate;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    // ---------------------------------------------
    //  /ping
    // ---------------------------------------------
    #[test]
    #[doc(hidden)]
    fn ping() {
        let client: CoinGeckoClient = CoinGeckoClient::default();
        let res = aw!(client.ping());
        assert!(res.is_ok(), "ping should resolve");
        assert_eq!(res.unwrap().gecko_says, "(V3) To the Moon!");
    }

    // ---------------------------------------------
    //  /simple
    // ---------------------------------------------
    #[test]
    fn price() {
        let client: CoinGeckoClient = CoinGeckoClient::default();
        let res_1 = aw!(client.price(&["bitcoin"], &["usd"], true, true, true, true));

        assert!(res_1.is_ok(), "price should resolve");
        let price_1 = &res_1.unwrap()["bitcoin"];
        assert!(price_1.usd.is_some(), "usd price should be defined");
        assert!(
            price_1.usd_market_cap.is_some(),
            "usd price should be defined"
        );
        assert!(
            price_1.usd24_h_vol.is_some(),
            "usd 24h vol should be defined"
        );
        assert!(
            price_1.usd24_h_change.is_some(),
            "usd 24h change should be defined"
        );
        assert!(
            price_1.last_updated_at.is_some(),
            "usd last update should be defined"
        );

        let res_2 = aw!(client.price(&["ethereum"], &["eur"], true, true, true, true));

        assert!(res_2.is_ok(), "price should resolve");
        let price_2 = &res_2.unwrap()["ethereum"];
        assert!(price_2.eur.is_some(), "eur price should be defined");
        assert!(
            price_2.eur_market_cap.is_some(),
            "eur price should be defined"
        );
        assert!(
            price_2.eur24_h_vol.is_some(),
            "eur 24h vol should be defined"
        );
        assert!(
            price_2.eur24_h_change.is_some(),
            "eur 24h change should be defined"
        );
        assert!(
            price_2.last_updated_at.is_some(),
            "eur last update should be defined"
        );
    }

    #[test]
    fn token_price() {
        let client: CoinGeckoClient = CoinGeckoClient::default();
        let uniswap_contract = "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984";
        let res = aw!(client.token_price(
            "ethereum",
            &[uniswap_contract],
            &["usd"],
            true,
            true,
            true,
            true
        ));

        assert!(res.is_ok(), "token price should resolve");
        let token_price = &res.unwrap()[&uniswap_contract.to_string()];
        assert!(token_price.usd.is_some(), "usd price should be defined");
        assert!(
            token_price.usd_market_cap.is_some(),
            "usd price should be defined"
        );
        assert!(
            token_price.usd24_h_vol.is_some(),
            "usd 24h vol should be defined"
        );
        assert!(
            token_price.usd24_h_change.is_some(),
            "usd 24h change should be defined"
        );
        assert!(
            token_price.last_updated_at.is_some(),
            "usd last update should be defined"
        );
    }

    #[test]
    fn supported_vs_currencies() {
        let client: CoinGeckoClient = CoinGeckoClient::default();
        let res = aw!(client.supported_vs_currencies());

        assert!(res.is_ok(), "supported_vs_currencies should resolve");
        assert!(
            !res.unwrap().is_empty(),
            "should return at least one currency"
        );
    }

    // ---------------------------------------------
    //  /coins
    // ---------------------------------------------
    #[test]
    fn coins_list() {
        let client: CoinGeckoClient = CoinGeckoClient::default();
        let res = aw!(client.coins_list(true));
        assert!(res.is_ok(), "list should resolve");
        assert!(!res.unwrap().is_empty(), "should return at least one coin");
    }

    #[test]
    fn coins_markets() {
        let client: CoinGeckoClient = CoinGeckoClient::default();

        let res = aw!(client.coins_markets(
            "usd",
            &["bitcoin"],
            None,
            MarketsOrder::GeckoDesc,
            1,
            0,
            true,
            &[
                PriceChangePercentage::OneHour,
                PriceChangePercentage::TwentyFourHours,
                PriceChangePercentage::SevenDays,
                PriceChangePercentage::FourteenDays,
                PriceChangePercentage::ThirtyDays,
                PriceChangePercentage::OneYear
            ],
        ));

        assert!(res.is_ok(), "markets should resolve");

        let res2 = aw!(client.coins_markets(
            "usd",
            &[] as &[&str],
            None,
            MarketsOrder::MarketCapDesc,
            250,
            30,
            false,
            &[],
        ));
        assert!(
            res2.is_ok(),
            "markets should resolve for pages near the end"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn coin() {
        let client: CoinGeckoClient = CoinGeckoClient::default();
        // Added a throttle for when public API is used to ensure we don't hit the rate limit
        // assuming the slower default base case
        const API_THROTTLE_TIME: u64 = 6;

        thread::sleep(Duration::from_secs(API_THROTTLE_TIME));
        let res1 = client
            .coin("01coin", false, false, false, false, false, false)
            .await
            .unwrap();
        assert_eq!(&res1.id, "01coin", "coin 01coin should resolve");
        thread::sleep(Duration::from_secs(API_THROTTLE_TIME));
        let res2 = client
            .coin("0xaiswap", false, false, false, false, false, false)
            .await
            .unwrap();
        assert_eq!(&res2.id, "0xaiswap", "coin 0xaiswap should resolve");
    }

    #[test]
    fn coin_tickers() {
        let client: CoinGeckoClient = CoinGeckoClient::default();

        let res1 = aw!(client.coin_tickers::<&str>(
            "bitcoin",
            None,
            true,
            1,
            TickersOrder::VolumeDesc,
            true
        ));

        assert!(res1.is_ok(), "tickers without filter should resolve");

        let res2 = aw!(client.coin_tickers(
            "bitcoin",
            #[allow(clippy::useless_vec)]
            Some(&vec![String::from("binance")]), // &Vec<String> should also work
            true,
            1,
            TickersOrder::VolumeDesc,
            true
        ));

        assert!(res2.is_ok(), "tickers without page should resolve");

        let res3 = aw!(client.coin_tickers(
            "bitcoin",
            Some(&["binance"]),
            true,
            1,
            TickersOrder::VolumeDesc,
            true
        ));

        assert!(res3.is_ok(), "tickers should resolve");
    }

    #[test]
    fn coin_history() {
        let client: CoinGeckoClient = CoinGeckoClient::default();

        let res = aw!(client.coin_history(
            "bitcoin",
            NaiveDate::from_ymd_opt(2017, 12, 30).unwrap(),
            true
        ));

        assert!(res.is_ok(), "history should resolve");
    }

    #[test]
    fn coin_market_chart() {
        let client: CoinGeckoClient = CoinGeckoClient::default();

        let res = aw!(client.coin_market_chart("bitcoin", "usd", 1, true));

        assert!(res.is_ok(), "market chart should resolve");
    }

    #[test]
    fn coin_market_chart_range() {
        let client: CoinGeckoClient = CoinGeckoClient::default();

        let from = NaiveDate::from_ymd(2014, 2, 16).and_hms(19, 0, 32);
        let to = NaiveDate::from_ymd(2015, 1, 30).and_hms(0, 20, 32);

        let res = aw!(client.coin_market_chart_range("bitcoin", "usd", from, to));

        assert!(res.is_ok(), "market chart range should resolve");
    }

    #[test]
    fn coin_ohlc() {
        let client: CoinGeckoClient = CoinGeckoClient::default();

        let res = aw!(client.coin_ohlc("bitcoin", "usd", OhlcDays::OneDay));

        assert!(res.is_ok(), "ohlc should resolve");
    }
}
