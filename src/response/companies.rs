use serde::{Deserialize, Serialize};

// ---------------------------------------------
//  /companies/public_treasury/{coin_id}
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompaniesPublicTreasury {
    pub total_holdings: f64,
    pub total_value_usd: f64,
    pub market_cap_dominance: f64,
    pub companies: Vec<Company>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Company {
    pub name: String,
    pub symbol: String,
    pub country: String,
    pub total_holdings: f64,
    pub total_entry_value_usd: f64,
    pub total_current_value_usd: f64,
    pub percentage_of_total_supply: f64,
}
