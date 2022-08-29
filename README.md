# Fetecher

Command line interface for fetching the data. This tool is built for fetching data and storing that into file. 

## Purpose 

So what's the motivation behind building this CLI - This can be used to fetch any crypto value and store that into file. This can later be used to push the data into blockchain and thus acting like an oracle.



## Installation
`cargo install --git https://github.com/valekar/fetcher.git`


## Usage 
The way to use this CLI is 

```bash
1. USAGE:
    zee-oracle-fetcher <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    binance          fetch the data from Binance
    coingecko        fetch the data from Coingecko
    coinmarketcap    fetch the data from Coinmarketcap
    help             Print this message or the help of the given subcommand(s)

#############################################################################################

2. USAGE:
    zee-oracle-fetcher coinmarketcap [OPTIONS] --api-type <API_TYPE> <CURRENCY> --config-ath <path_string>
```


### Example 

1. Run the following command to fetch the data eth data using the quote URL from coinmarketcap
    `zee-oracle-fetcher coinmarketcap eth --api-type quote --config-path "./.aptos/config.yaml"`     


### Supported Markets

This currently supports `eth` and `btc`. And the API supported is `coinmarketcap`  . You could fetch the data from any `api` data provider. 


### Config Path
Pass the config path for sending the data to blockchain


### Docker Image
Docker image runs a Cron job every 15 minutes. This frequency can be changed to anything that you want. 

### Limitations

Currently `coinmarketcap` supports only 333 calls per day. Hence the fetch limit has been put to 15 minutes in the Docker file.  
