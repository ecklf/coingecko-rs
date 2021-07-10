use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Localization {
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
pub struct Image {
    pub thumb: Option<String>,
    pub small: Option<String>,
    pub large: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentPrice {
    pub aed: Option<f64>,
    pub ars: Option<f64>,
    pub aud: Option<f64>,
    pub bch: Option<f64>,
    pub bdt: Option<f64>,
    pub bhd: Option<f64>,
    pub bmd: Option<f64>,
    pub bnb: Option<f64>,
    pub brl: Option<f64>,
    pub btc: Option<f64>,
    pub cad: Option<f64>,
    pub chf: Option<f64>,
    pub clp: Option<f64>,
    pub cny: Option<f64>,
    pub czk: Option<f64>,
    pub dkk: Option<f64>,
    pub dot: Option<f64>,
    pub eos: Option<f64>,
    pub eth: Option<f64>,
    pub eur: Option<f64>,
    pub gbp: Option<f64>,
    pub hkd: Option<f64>,
    pub huf: Option<f64>,
    pub idr: Option<f64>,
    pub ils: Option<f64>,
    pub inr: Option<f64>,
    pub jpy: Option<f64>,
    pub krw: Option<f64>,
    pub kwd: Option<f64>,
    pub lkr: Option<f64>,
    pub ltc: Option<f64>,
    pub mmk: Option<f64>,
    pub mxn: Option<f64>,
    pub myr: Option<f64>,
    pub ngn: Option<f64>,
    pub nok: Option<f64>,
    pub nzd: Option<f64>,
    pub php: Option<f64>,
    pub pkr: Option<f64>,
    pub pln: Option<f64>,
    pub rub: Option<f64>,
    pub sar: Option<f64>,
    pub sek: Option<f64>,
    pub sgd: Option<f64>,
    pub thb: Option<f64>,
    #[serde(rename = "try")]
    pub try_field: Option<f64>,
    pub twd: Option<f64>,
    pub uah: Option<f64>,
    pub usd: Option<f64>,
    pub vef: Option<f64>,
    pub vnd: Option<f64>,
    pub xag: Option<f64>,
    pub xau: Option<f64>,
    pub xdr: Option<f64>,
    pub xlm: Option<f64>,
    pub xrp: Option<f64>,
    pub yfi: Option<f64>,
    pub zar: Option<f64>,
    pub bits: Option<f64>,
    pub link: Option<f64>,
    pub sats: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketCap {
    pub aed: Option<f64>,
    pub ars: Option<f64>,
    pub aud: Option<f64>,
    pub bch: Option<f64>,
    pub bdt: Option<f64>,
    pub bhd: Option<f64>,
    pub bmd: Option<f64>,
    pub bnb: Option<f64>,
    pub brl: Option<f64>,
    pub btc: Option<f64>,
    pub cad: Option<f64>,
    pub chf: Option<f64>,
    pub clp: Option<f64>,
    pub cny: Option<f64>,
    pub czk: Option<f64>,
    pub dkk: Option<f64>,
    pub dot: Option<f64>,
    pub eos: Option<f64>,
    pub eth: Option<f64>,
    pub eur: Option<f64>,
    pub gbp: Option<f64>,
    pub hkd: Option<f64>,
    pub huf: Option<f64>,
    pub idr: Option<f64>,
    pub ils: Option<f64>,
    pub inr: Option<f64>,
    pub jpy: Option<f64>,
    pub krw: Option<f64>,
    pub kwd: Option<f64>,
    pub lkr: Option<f64>,
    pub ltc: Option<f64>,
    pub mmk: Option<f64>,
    pub mxn: Option<f64>,
    pub myr: Option<f64>,
    pub ngn: Option<f64>,
    pub nok: Option<f64>,
    pub nzd: Option<f64>,
    pub php: Option<f64>,
    pub pkr: Option<f64>,
    pub pln: Option<f64>,
    pub rub: Option<f64>,
    pub sar: Option<f64>,
    pub sek: Option<f64>,
    pub sgd: Option<f64>,
    pub thb: Option<f64>,
    #[serde(rename = "try")]
    pub try_field: Option<f64>,
    pub twd: Option<f64>,
    pub uah: Option<f64>,
    pub usd: Option<f64>,
    pub vef: Option<f64>,
    pub vnd: Option<f64>,
    pub xag: Option<f64>,
    pub xau: Option<f64>,
    pub xdr: Option<f64>,
    pub xlm: Option<f64>,
    pub xrp: Option<f64>,
    pub yfi: Option<f64>,
    pub zar: Option<f64>,
    pub bits: Option<f64>,
    pub link: Option<f64>,
    pub sats: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TotalVolume {
    pub aed: Option<f64>,
    pub ars: Option<f64>,
    pub aud: Option<f64>,
    pub bch: Option<f64>,
    pub bdt: Option<f64>,
    pub bhd: Option<f64>,
    pub bmd: Option<f64>,
    pub bnb: Option<f64>,
    pub brl: Option<f64>,
    pub btc: Option<f64>,
    pub cad: Option<f64>,
    pub chf: Option<f64>,
    pub clp: Option<f64>,
    pub cny: Option<f64>,
    pub czk: Option<f64>,
    pub dkk: Option<f64>,
    pub dot: Option<f64>,
    pub eos: Option<f64>,
    pub eth: Option<f64>,
    pub eur: Option<f64>,
    pub gbp: Option<f64>,
    pub hkd: Option<f64>,
    pub huf: Option<f64>,
    pub idr: Option<f64>,
    pub ils: Option<f64>,
    pub inr: Option<f64>,
    pub jpy: Option<f64>,
    pub krw: Option<f64>,
    pub kwd: Option<f64>,
    pub lkr: Option<f64>,
    pub ltc: Option<f64>,
    pub mmk: Option<f64>,
    pub mxn: Option<f64>,
    pub myr: Option<f64>,
    pub ngn: Option<f64>,
    pub nok: Option<f64>,
    pub nzd: Option<f64>,
    pub php: Option<f64>,
    pub pkr: Option<f64>,
    pub pln: Option<f64>,
    pub rub: Option<f64>,
    pub sar: Option<f64>,
    pub sek: Option<f64>,
    pub sgd: Option<f64>,
    pub thb: Option<f64>,
    #[serde(rename = "try")]
    pub try_field: Option<f64>,
    pub twd: Option<f64>,
    pub uah: Option<f64>,
    pub usd: Option<f64>,
    pub vef: Option<f64>,
    pub vnd: Option<f64>,
    pub xag: Option<f64>,
    pub xau: Option<f64>,
    pub xdr: Option<f64>,
    pub xlm: Option<f64>,
    pub xrp: Option<f64>,
    pub yfi: Option<f64>,
    pub zar: Option<f64>,
    pub bits: Option<f64>,
    pub link: Option<f64>,
    pub sats: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommunityData {
    pub facebook_likes: Option<f64>,
    pub twitter_followers: Option<f64>,
    #[serde(rename = "reddit_average_posts_48h")]
    pub reddit_average_posts48_h: Option<f64>,
    #[serde(rename = "reddit_average_comments_48h")]
    pub reddit_average_comments48_h: Option<f64>,
    pub reddit_subscribers: Option<f64>,
    #[serde(rename = "reddit_accounts_active_48h")]
    pub reddit_accounts_active48_h: Option<Value>,
    pub telegram_channel_user_count: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeveloperData {
    pub forks: Option<f64>,
    pub stars: Option<f64>,
    pub subscribers: Option<f64>,
    pub total_issues: Option<f64>,
    pub closed_issues: Option<f64>,
    pub pull_requests_merged: Option<f64>,
    pub pull_request_contributors: Option<f64>,
    #[serde(rename = "code_additions_deletions_4_weeks")]
    pub code_additions_deletions4_weeks: CodeAdditionsDeletions4Weeks,
    #[serde(rename = "commit_count_4_weeks")]
    pub commit_count4_weeks: Option<f64>,
    #[serde(rename = "last_4_weeks_commit_activity_series")]
    pub last4_weeks_commit_activity_series: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicInterestStats {
    pub alexa_rank: Option<f64>,
    pub bing_matches: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeAdditionsDeletions4Weeks {
    pub additions: Option<f64>,
    pub deletions: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Links {
    pub homepage: Vec<String>,
    pub blockchain_site: Vec<String>,
    pub official_forum_url: Vec<String>,
    pub chat_url: Vec<String>,
    pub announcement_url: Vec<String>,
    pub twitter_screen_name: Value,
    pub facebook_username: Value,
    pub bitcointalk_thread_identifier: Value,
    pub telegram_channel_identifier: Value,
    pub subreddit_url: Value,
    pub repos_url: ReposUrl,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReposUrl {
    pub github: Vec<String>,
    pub bitbucket: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tickers {
    pub name: String,
    pub tickers: Vec<Ticker>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticker {
    pub base: String,
    pub target: String,
    pub market: Market,
    pub last: f64,
    pub volume: f64,
    pub converted_last: ConvertedLast,
    pub converted_volume: ConvertedVolume,
    pub cost_to_move_up_usd: Option<f64>,
    pub cost_to_move_down_usd: Option<f64>,
    pub trust_score: Option<String>,
    pub bid_ask_spread_percentage: Option<f64>,
    pub timestamp: Option<String>,
    pub last_traded_at: Option<String>,
    pub last_fetch_at: Option<String>,
    pub is_anomaly: bool,
    pub is_stale: bool,
    pub trade_url: Option<String>,
    pub token_info_url: Option<String>,
    pub coin_id: String,
    pub target_coin_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Market {
    pub name: String,
    pub identifier: String,
    pub has_trading_incentive: bool,
    pub logo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConvertedLast {
    pub btc: f64,
    pub eth: f64,
    pub usd: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConvertedVolume {
    pub btc: f64,
    pub eth: f64,
    pub usd: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusUpdates {
    pub status_updates: Vec<StatusUpdate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusUpdate {
    pub description: Option<String>,
    pub category: Option<String>,
    pub created_at: Option<String>,
    pub user: Option<String>,
    pub user_title: Option<String>,
    pub pin: Option<bool>,
    pub project: Option<Project>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub image: Image,
}
