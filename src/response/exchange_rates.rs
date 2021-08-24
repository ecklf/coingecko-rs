#![allow(missing_docs)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------
//  /exchange_rates
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeRates {
    pub rates: HashMap<String, ExchangeRateData>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeRateData {
    pub name: String,
    pub unit: String,
    pub value: f64,
    #[serde(rename = "type")]
    pub type_field: String,
}
