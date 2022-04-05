# geodata-anchor

Provides a cosmwasm anchor for geodata object with its hash
### Requirements
* [Rust](https://www.rust-lang.org/tools/install)
### development
cargo watch -x test -w src

### schemas:
cargo schema

### scripts, streamlined run, optimize, and deploy:
* build juno:
```sh
sudo apt-get install make build-essential gcc git jq chrony -y
git clone https://github.com/CosmosContracts/juno.git && cd juno
git fetch --tags
git checkout v2.3.1
make build && make install
junod version
```
* [create default test account](https://docs.junonetwork.io/smart-contracts-and-junod-development/junod-local-dev-setup)
```sh
junod keys add sample-test-keyname --recover
# clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose
```
* run juno local node, deploy contract:
```sh
# run juno local node:
./scripts/run_local_node.sh

# in new terminal:
./scripts/optimize.sh
# builds to artifacts/geodata_anchor.wasm

./scripts/local_deploy.sh
# this will echo CONTRACT_ADDRESS, created dynamically from local juno instance.
# $CONTRACT_ADDRESS must be applied to geodata-rest/src/config/default.json before starting rest layer
```
### TODO:
* add access control, as cw1-whitelist
* implement add QueryMsgValid {id: String, hash: String}