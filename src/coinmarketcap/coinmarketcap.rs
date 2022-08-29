use crate::{coinmarketcap::model::CoinmarketcapApiResponse, ApiType, Coin, ConvertTo};
use anyhow::{Error, Result};
//use chrono::{Duration, Utc, Weekday};
use reqwest;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufWriter};
use std::{env, fmt::Display};

use super::TruncatedTokenPrice;
//use tokio::spawn;
//use tokio_schedule::{every, Job};

pub async fn handle_command(
    coin_type: Coin,
    api_type: ApiType,
    convert_to: ConvertTo,
) -> Result<()> {
    // let every_30_minutes =
    //     every(1)
    //         .minutes()
    //         .at(20)
    //         .in_timezone(&Utc)
    //         .perform(move || async move {
    //             call_api(coin_type, api_type, convert_to).await.unwrap();
    //         });
    // spawn(every_30_minutes);

    call_api(coin_type, api_type, convert_to).await?;
    Ok(())
}

async fn call_api(coin_type: Coin, api_type: ApiType, convert_to: ConvertTo) -> Result<()> {
    let uri_str = &build_url(coin_type, api_type, convert_to).unwrap()[..];
    let result = fetch_quote_data(uri_str).await?;

    let mut coinmarketcap_data: Vec<CoinmarketcapApiResponse> = {
        let coinmarketcap_data = std::fs::read_to_string("coinmarketcap.json")?;

        serde_json::from_str::<Vec<CoinmarketcapApiResponse>>(&coinmarketcap_data).unwrap_or(vec![])
    };

    let truncated_result = truncate_token_price_info(result.clone()).unwrap();

    println!("{:?}", truncated_result);

    // println!("{:?}", coinmarketcap_data);

    coinmarketcap_data.push(result);

    let file = OpenOptions::new()
        .write(true)
        .append(false)
        .open("coinmarketcap.json")
        .unwrap();

    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &coinmarketcap_data)?;
    writer.flush()?;

    Ok(())
}

async fn fetch_quote_data(uri_str: &str) -> Result<CoinmarketcapApiResponse> {
    info!("The URL is {}", uri_str);

    let resp = reqwest::get(uri_str).await?;

    let converted_result = resp.json::<CoinmarketcapApiResponse>().await?;

    Ok(converted_result)
}

fn build_url(coin_type: Coin, api_type: ApiType, convert_to: ConvertTo) -> Result<String> {
    let api_uri = &build_api_uri(api_type).unwrap()[..];

    let coin_type_str = match coin_type {
        Coin::BTC => "btc",
        Coin::ETH => "eth",
    };

    let convert_to = match convert_to {
        ConvertTo::USD => "usd",
    };

    warn!("The coin_type is :: {}", coin_type_str);
    warn!("The Api type is  :: {}", api_uri);
    warn!("The convert to is :: {}", convert_to);

    let api_str = format!(
        "{}&symbol={}&convert={}",
        api_uri, coin_type_str, convert_to
    );

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

fn truncate_token_price_info(result: CoinmarketcapApiResponse) -> Result<TruncatedTokenPrice> {
    let token = result.data.values().next().unwrap();
    let token_quote = token.quote.values().next().unwrap();

    let truncated_token_price = TruncatedTokenPrice {
        name: token.clone().name,
        symbol: token.clone().symbol,
        price: (token_quote.price * 100000000.0).round() as u128,
        last_updated: token_quote.clone().last_updated,
    };

    Ok(truncated_token_price)
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
