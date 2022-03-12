#!/bin/bash

TX=$( \
  junod tx wasm store \
  ./artifacts/geodata_anchor.wasm \
  --from sample-test-keyname \
  --chain-id=testing --gas auto \
  --gas-adjustment 1.3 \
  --gas-prices 0.025ujunox \
  --output json -y | jq -r '.txhash' \
)
echo "TX: $TX"

sleep 5

CODE_ID=$( \
  junod query tx $TX \
  --output json | jq -r '.logs[0].events[-1].attributes[0].value' \
)
echo "CODE_ID: $CODE_ID"

sleep 5

CONTRACT_ADDRESS=$( \
junod tx wasm instantiate $CODE_ID \
  '{"admins":["juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y"],"users":["juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y"],"mutable":false}' \
  --amount 50000ujunox \
  --label "CW1 example contract" \
  --from sample-test-keyname \
  --chain-id testing \
  -b block \
  --output json \
  -y | jq -r  .logs[0].events[0].attributes[0].value \
)
echo "CONTRACT_ADDRESS: $CONTRACT_ADDRESS"

junod query wasm list-contract-by-code $CODE_ID
