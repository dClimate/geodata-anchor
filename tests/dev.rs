//! Development-related functionality.
//!
//! This module contains support for integration testing against a
//! Cosmos SDK-compatible full node running inside of Docker.
#![allow(dead_code)]
use std::{ffi::OsStr, panic, process, str, time::Duration};

use tendermint_rpc as rpc;
use tendermint_rpc::Client;
use cosmrs::tx::{self, Tx};

use tokio::time;
/// Execute a given `docker` command, returning what was written to stdout
/// if the command completed successfully.
///
/// Panics if the `docker` process exits with an error code.
pub fn exec_docker_command<A, S>(name: &str, args: A) -> String
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = process::Command::new("docker")
        .arg(name)
        .args(args)
        .stdout(process::Stdio::piped())
        .output()
        .unwrap_or_else(|err| panic!("error invoking `docker {}`: {}", name, err));

    if !output.status.success() {
        panic!("`docker {}` exited with error status: {:?}", name, output);
    }

    str::from_utf8(&output.stdout)
        .expect("UTF-8 error decoding docker output")
        .trim_end()
        .to_owned()
}

/// Wait for the node to produce the first block.
///
/// This should be used at the beginning of the test lifecycle to ensure
/// the node is fully booted.
pub async fn poll_for_first_block(rpc_client: &rpc::HttpClient) {
    rpc_client
        .wait_until_healthy(Duration::from_secs(5))
        .await
        .unwrap();

    let mut attempts_remaining = 25;

    while let Err(e) = rpc_client.latest_block().await {
        if !matches!(e.detail(), rpc::error::ErrorDetail::Serde(_)) {
            panic!("unexpected error waiting for first block: {:?}", e);
        }

        if attempts_remaining == 0 {
            panic!("timeout waiting for first block");
        }

        attempts_remaining -= 1;
        time::sleep(Duration::from_millis(200)).await;
    }
}

/// Wait for a transaction with the given hash to appear in the blockchain
pub async fn poll_for_tx(rpc_client: &rpc::HttpClient, tx_hash: tx::Hash) -> Tx {
    let attempts = 20;

    for _ in 0..attempts {
        if let Ok(tx) = Tx::find_by_hash(rpc_client, tx_hash).await {
            return tx;
        }
    }

    panic!("couldn't find transaction after {} attempts!", attempts);
}
