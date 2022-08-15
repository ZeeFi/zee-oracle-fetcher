#!/bin/bash
echo "Running Fetcher"
echo "Started at" $(date +%T)

export PATH="/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"

# `ls -ai`
# wait 


result_2= `fetcher coinmarketcap eth --api-type quote`
wait 
echo $result_2

echo "Finished at" $(date +%T)