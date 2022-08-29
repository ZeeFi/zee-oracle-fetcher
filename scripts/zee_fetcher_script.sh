#!/bin/bash
echo "Running Zee-Oracle-Fetcher"
echo "Started at" $(date +%T)

export PATH="/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"

# `ls -ai`
# wait 


result_2= `zee-oracle-fetcher coinmarketcap eth --api-type quote --config-oath ".aptos/config.yaml"`
wait 
echo $result_2

echo "Finished at" $(date +%T)