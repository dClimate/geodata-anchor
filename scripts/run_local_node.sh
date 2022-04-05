#!/bin/bash

# https://github.com/CosmosContracts/juno
# https://docs.junonetwork.io/smart-contracts-and-junod-development/junod-local-dev-setup#run-juno
docker run -it \
  -p 26656:26656 \
  -p 26657:26657 \
  -e STAKE_TOKEN=ujunox \
  -e UNSAFE_CORS=true \
  ghcr.io/cosmoscontracts/juno:v2.3.1 \
  ./setup_and_run.sh juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y