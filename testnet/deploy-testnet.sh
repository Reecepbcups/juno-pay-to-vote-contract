#!/bin/bash
junod tx wasm store artifacts/juno_paid_to_vote.wasm --from juno10c3slrqx3369mfsr9670au22zvq082jaej8ve4 --node https://juno-testnet-rpc.polkachu.com:443 --chain-id uni-5 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --output json -b block -y

# 660