#![allow(missing_docs)]
use serde::{Deserialize, Serialize};

// ---------------------------------------------
//  /events
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Events {
    pub data: Vec<Event>,
    pub count: i64,
    pub page: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub description: String,
    pub organizer: String,
    pub start_date: String,
    pub end_date: String,
    pub website: String,
    pub email: String,
    pub venue: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub screenshot: String,
}
// ---------------------------------------------
//  /events/countries
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventCountries {
    pub data: Vec<Country>,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Country {
    pub country: Option<String>,
    pub code: String,
}

// ---------------------------------------------
//  /events/types
// ---------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventTypes {
    pub data: Vec<String>,
    pub count: i64,
}
