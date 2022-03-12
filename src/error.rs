use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Hash parse error: {0}")]
    ParseError(String),

    #[error("Invalid id")]
    InvalidId {},

    #[error("Invalid hash ({0} chars): must be 64 characters")]
    InvalidHash(usize),

    #[error("Anchor already exists")]
    AlreadyExists,
}
