#![allow(missing_docs)]
use serde::{Deserialize, Serialize};

// ---------------------------------------------
//  /finance_platforms
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinancePlatform {
    pub name: String,
    pub facts: String,
    pub category: String,
    pub centralized: bool,
    pub website_url: String,
}

// ---------------------------------------------
//  /finance_products
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinanceProduct {
    pub platform: String,
    pub identifier: String,
    pub supply_rate_percentage: Option<String>,
    pub borrow_rate_percentage: Option<String>,
    pub number_duration: Option<f64>,
    pub length_duration: Option<f64>,
    pub start_at: i64,
    pub end_at: i64,
    pub value_at: i64,
    pub redeem_at: i64,
}
