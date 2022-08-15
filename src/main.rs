extern crate dotenv;
use dotenv::dotenv;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use anyhow::Result;
use clap::{Parser, Subcommand};

mod coinmarketcap;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    info!("Calling the commands");

    let opt = Cli::parse();
    entry(opt).await?;
    Ok(())
}

#[derive(Parser)]
#[clap(
    name = "fetcher",
    about = "data fetcher from central entities",
    version
)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// fetch the data from Coinmarketcap
    Coinmarketcap {
        #[clap(arg_enum)]
        coin_type: Coin,
        #[clap(arg_enum, long, value_parser, default_value = "quote")]
        api_type: ApiType,
        #[clap(arg_enum, long, value_parser, default_value = "usd")]
        convert_to: ConvertTo,
    },
    /// fetch the data from Coingecko
    Coingecko { coin_type: String },
    /// fetch the data from Binance
    Binance { coin_type: String },
}

#[derive(Clone, clap::ArgEnum, Copy)]
pub enum ApiType {
    Quote,
    Listing,
}

#[derive(Clone, clap::ArgEnum, Copy)]
pub enum Coin {
    ETH,
    BTC,
}

#[derive(Clone, clap::ArgEnum, Copy)]
pub enum ConvertTo {
    USD,
}

pub async fn entry(opt: Cli) -> Result<()> {
    match opt.command {
        Commands::Coinmarketcap {
            coin_type,
            api_type,
            convert_to,
        } => {
            coinmarketcap::handle_command(coin_type, api_type, convert_to).await?;
            Ok(())
        }
        Commands::Coingecko { coin_type: _ } => Ok(()),
        Commands::Binance { coin_type: _ } => Ok(()),
    }
}
