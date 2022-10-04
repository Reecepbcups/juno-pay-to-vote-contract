#!/bin/bash

CONTRACT_ADDR="juno136nta67q3yjdhf6n9jxheamrzkga7rpe5jk06s53pae4rsundu6qkdskep"
# junod q wasm contract-state smart $CONTRACT_ADDR '{"get_config": {}}' --node https://rpc.uni.juno.deuslabs.fi:443 --chain-id uni-5 --output json

MSG='{"query_vote_status": {"proposal": 16, "address": "juno10r39fueph9fq7a6lgswu4zdsg8t3gxlq670lt0"}}'
junod q wasm contract-state smart $CONTRACT_ADDR "$MSG" --node https://rpc.uni.juno.deuslabs.fi:443 --chain-id uni-5 --output json