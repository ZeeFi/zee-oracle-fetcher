#!/bin/bash
echo "Running Fetcher"
echo "Started at" $(date +%T)

result_1= `fetcher coinmarketcap eth --api-type quote`
wait 
echo $result_1

echo "Finished at" $(date +%T)