#!/bin/bash
# Run this after deploying and getting the code ID
# User should pass in the code ID to the contract like:
# ./instantiate-local.sh 2
if [ -z "$1" ]
then
    echo "Must provide code ID (Example ./instantiate-local.sh 660)"
    exit 1
else
    CODE_ID=$1
fi

INIT='{"contract_admin": "juno10c3slrqx3369mfsr9670au22zvq082jaej8ve4", "denom": "ujunox"}'
junod tx wasm instantiate "$CODE_ID" "$INIT" --label "juno-paid" --from testwasm --node https://rpc.uni.juno.deuslabs.fi:443 --chain-id uni-5 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --output json -b block --no-admin -y

# juno136nta67q3yjdhf6n9jxheamrzkga7rpe5jk06s53pae4rsundu6qkdskep