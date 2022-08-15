echo "Running Fetcher"
echo "Started at" $(date +%T)

result_1= `fetcher coinmarketcap eth`
wait 
echo $result_1

echo "Finished at" $(date +%T)