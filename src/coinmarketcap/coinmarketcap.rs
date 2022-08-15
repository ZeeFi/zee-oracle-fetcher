use crate::{coinmarketcap::model::CoinmarketcapApiResponse, ApiType, ConvertTo, Currency};
use anyhow::{Error, Result};
use reqwest;
use std::{env, fmt::Display};

pub async fn handle_command(
    currency: Currency,
    api_type: ApiType,
    convert_to: ConvertTo,
) -> Result<()> {
    call_api(currency, api_type, convert_to).await?;
    Ok(())
}

async fn call_api(currency: Currency, api_type: ApiType, convert_to: ConvertTo) -> Result<()> {
    let uri_str = &build_url(currency, api_type, convert_to).unwrap()[..];
    let result = fetch_quote_data(uri_str).await?;

    println!("{:?}", result);

    Ok(())
}

async fn fetch_quote_data(uri_str: &str) -> Result<CoinmarketcapApiResponse> {
    info!("The URL is {}", uri_str);

    let resp = reqwest::get(uri_str).await?;

    let converted_result = resp.json::<CoinmarketcapApiResponse>().await?;

    Ok(converted_result)
}

fn build_url(currency: Currency, api_type: ApiType, convert_to: ConvertTo) -> Result<String> {
    let api_uri = &build_api_uri(api_type).unwrap()[..];

    let currency_str = match currency {
        Currency::BTC => "btc",
        Currency::ETH => "eth",
    };

    let convert_to = match convert_to {
        ConvertTo::USD => "usd",
    };

    warn!("The currency is :: {}", currency_str);
    warn!("The Api type is  :: {}", api_uri);
    warn!("The convert to is :: {}", convert_to);

    let api_str = format!("{}&symbol={}&convert={}", api_uri, currency_str, convert_to);

    Ok(api_str)
}

fn build_api_uri(api_type: ApiType) -> Result<String> {
    info!("Building API URI from ENV varaibles");

    let coinmarket_uri = env::var("COINMARKETCAP_URI")?;

    let coinmarket_api_key = env::var("COINMARKETCAP_API_KEY")?;

    let api_uri = match api_type {
        ApiType::Quote => {
            let coinmarketcap_quote_str = env::var("COINMARKETCAP_QUOTE_STR")?;
            format!(
                "{}{}{}",
                coinmarket_uri, coinmarketcap_quote_str, coinmarket_api_key
            )
        }
        ApiType::Listing => return Err(Error::msg(FetcherError::NotImplemented.to_string())),
    };

    Ok(api_uri)
}

#[derive(Debug)]
enum FetcherError {
    NotImplemented,
}

impl Display for FetcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
