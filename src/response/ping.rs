#![allow(missing_docs)]
use serde::{Deserialize, Serialize};

// ---------------------------------------------
//  /ping
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimplePing {
    pub gecko_says: String,
}
