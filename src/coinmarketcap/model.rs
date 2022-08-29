use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct CoinmarketcapApiResponse {
    pub status: Status,
    pub data: HashMap<String, Token>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Status {
    pub timestamp: String, // change this to i64
    pub error_code: u8,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub error_message: String,
    pub elapsed: u16,
    pub credit_count: u8,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub notice: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Token {
    pub id: u16,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    // pub num_market_pairs: u16,
    // pub date_added: String, // change this to i64
    // pub tags: Vec<String>,
    // pub max_supply: Option<u64>,
    // pub circulating_supply: f64,
    // pub total_supply: f64,
    // pub is_active: u8,
    // #[serde(deserialize_with = "deserialize_null_default")]
    // pub platform: String,
    // pub cmc_rank: u8,
    // pub is_fiat: u8,
    // #[serde(deserialize_with = "deserialize_null_default")]
    // pub self_reported_circulating_supply: String,
    // #[serde(deserialize_with = "deserialize_null_default")]
    // pub self_reported_market_cap: String,
    // #[serde(deserialize_with = "deserialize_null_default")]
    // pub tvl_ratio: String,
    pub last_updated: String, // change this to i64
    pub quote: HashMap<String, Currency>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Currency {
    pub price: f64,
    pub volume_24h: f64,
    pub volume_change_24h: f64,
    pub percent_change_1h: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
    pub percent_change_30d: f64,
    pub percent_change_60d: f64,
    pub percent_change_90d: f64,
    pub market_cap: f64,
    pub market_cap_dominance: f64,
    pub fully_diluted_market_cap: f64,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub tvl: String,
    pub last_updated: String, // change this to i64
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;

    let return_value = match opt {
        Some(value) => value,
        None => Default::default(),
    };

    Ok(return_value)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TruncatedTokenPrice {
    pub name: String,
    pub symbol: String,
    pub price: u128,
    pub last_updated: String,
}
