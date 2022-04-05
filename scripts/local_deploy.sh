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
  --no-admin \
  -b block \
  --output json -y | jq -r '.logs[0].events[0].attributes[0].value' \
)
echo "CONTRACT_ADDRESS: $CONTRACT_ADDRESS"

junod query wasm list-contract-by-code $CODE_ID

CREATE=$( \
junod tx wasm execute $CONTRACT_ADDRESS \
  '{"create":{"id":"012345678901234567890123","hash":"bdda97435bea603cd428e8112cec883cbd492d23bdda97435bea603cd428e811","account":"acct1","created":"1"}}' \
  --from juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y \
  --chain-id testing \
  --gas-prices 0.1ujunox --gas auto --gas-adjustment 1.3 -b block  \
  --output json -y | jq -r '.logs[0].events[-1]' \
)
echo "CREATE: $CREATE"

DETAILS=$( \
junod query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"details":{"id":"012345678901234567890123"}}' \
  --chain-id testing \
  --output json | jq -r '.data' \
)
echo "DETAILS: $DETAILS"
