use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Timestamp;

use crate::state::Validation;

/// TODO: implement access control based on admins and users
/// admins can instatiate and modify access lists, if mutable
/// admins can execute
/// users can query
/// see cw1-whitelist
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub users: Vec<String>,
    pub mutable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create(CreateMsg),
    Validate(ValidateMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreateMsg {
    /// hex of geodata objectid (PK)
    pub id: String,
    /// hex of account objectid
    pub account: String,
    /// hex-encoded hash of geodata (must be 32*2 = 64 chars)
    pub hash: String,
    /// geodata created
    pub created: Timestamp,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ValidateMsg {
    /// hex of geodata objectid (PK)
    pub id: String,
    /// hex of account objectid
    pub account: String,
    /// hex-encoded hash of geodata (must be 32*2 = 64 chars)
    pub hash: String,
    /// validation created
    pub created: Timestamp,
}

pub fn is_valid_id(id: &str) -> bool {
    id.as_bytes().len() == 24
}

/// TODO: add Valid {id: String, hash: String}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the details of the anchor, error if not created.
    /// Return type: DetailsResponse.
    Details { id: String },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct DetailsResponse {
    /// geodata id (PK)
    pub id: String,
    pub account: String,
    pub hash: String,
    pub source: String,
    pub created: Timestamp,
    pub validations: Vec<Validation>,
}
