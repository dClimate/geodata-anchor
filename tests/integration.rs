//! Integration test which submits transactions to a running docker `juno` node.
//!
//! Requires Docker.
use bip32::XPrv;
use bip39::{Language, Mnemonic, Seed};
use cosmrs::{
    cosmwasm::{AccessConfig, MsgStoreCode},
    crypto::secp256k1,
    tx::{self, AccountNumber, Fee, Msg, SignDoc, SignerInfo},
    Coin,
};
use std::fs::File;
use std::io::prelude::*;
use std::{panic, str};
use tracing::info;

/// Chain ID to use for tests
const CHAIN_ID: &str = "testing";

/// RPC port
const RPC_PORT1: u16 = 26656;
const RPC_PORT2: u16 = 26657;

/// Expected account number
const ACCOUNT_NUMBER: AccountNumber = 1;

/// Bech32 prefix for an account
const ACCOUNT_PREFIX: &str = "juno";

/// Denom name
const DENOM: &str = "ujunox";

/// Example memo
const MEMO: &str = "test memo";
mod dev;
use tendermint_rpc as rpc;

#[tokio::test]
async fn test_workflow() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // start juno
    let docker_args = [
        "-d",
        "-e",
        "STAKE_TOKEN=ujunox",
        "-e",
        "UNSAFE_CORS=true",
        "-p",
        &format!("{}:{}", RPC_PORT1, RPC_PORT1),
        "-p",
        &format!("{}:{}", RPC_PORT2, RPC_PORT2),
        "ghcr.io/cosmoscontracts/juno:v2.3.1",
        "./setup_and_run.sh",
        "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y",
    ];
    let container_id = dev::exec_docker_command("run", docker_args);
    
    // set up private key, public key and account from juno built-in test account
    let mnemonic = Mnemonic::from_phrase("clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose", Language::English).unwrap();
    let seed = Seed::new(&mnemonic, "");
    // let root_privk = XPrv::new(&seed).unwrap();
    let privk = XPrv::derive_from_path(&seed, &"m/44'/118'/0'/0/0".parse().unwrap()).unwrap();
    let bytes = privk.private_key().to_bytes();
    let sender_private_key = secp256k1::SigningKey::from_bytes(bytes.as_slice()).unwrap();
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id(ACCOUNT_PREFIX).unwrap();

    let amount = Coin {
        amount: 1u8.into(),
        denom: DENOM.parse().unwrap(),
    };

    // store
    let mut contract_code = File::open("./artifacts/geodata_anchor.wasm").unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    contract_code.read_to_end(&mut buffer).unwrap();

    let msg_store = MsgStoreCode {
        sender: sender_account_id,
        wasm_byte_code: buffer,
        instantiate_permission: None::<AccessConfig>,
    }
    .to_any()
    .unwrap();

    let chain_id = CHAIN_ID.parse().unwrap();
    let sequence_number = 0;
    let gas = 20_000_000;
    let fee = Fee::from_amount_and_gas(amount, gas);
    let timeout_height = 9001u16;

    let tx_body = tx::Body::new(vec![msg_store], MEMO, timeout_height);
    let auth_info =
        SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, ACCOUNT_NUMBER).unwrap();
    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();

    let rpc_address = format!("http://localhost:{}", RPC_PORT2);
    let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap();
    dev::poll_for_first_block(&rpc_client).await;

    let tx_commit_response = tx_raw.broadcast_commit(&rpc_client).await.unwrap();

    if tx_commit_response.check_tx.code.is_err() {
        panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
    }

    if tx_commit_response.deliver_tx.code.is_err() {
        panic!("deliver_tx failed: {:?}", tx_commit_response.deliver_tx);
    }
    let code = tx_commit_response.deliver_tx.code.value();
    let tx: tx::Tx = dev::poll_for_tx(&rpc_client, tx_commit_response.hash).await;
    assert_eq!(&tx_body, &tx.body);
    assert_eq!(&auth_info, &tx.auth_info);
    info!("store code succeeded, tx code: {:?}", code);
    dev::exec_docker_command("kill", &[&container_id]);
}