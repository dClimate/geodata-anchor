#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};


use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    is_valid_id, CreateMsg, DetailsResponse, ExecuteMsg, InstantiateMsg,
    QueryMsg,
};
use crate::state::{Anchor, ANCHORS};

// Version info, for migration info
const CONTRACT_NAME: &str = "crates.io:geodata-anchor";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // No setup
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Create(msg) => {
            execute_create(deps, env, info, msg)
        }
    }
}

pub fn execute_create(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: CreateMsg,
) -> Result<Response, ContractError> {
    if !is_valid_id(&msg.id) {
        return Err(ContractError::InvalidId {});
    }

    let hash = parse_hex_32(&msg.hash)?;

    let anchor = Anchor {
        account: msg.account.clone(),
        hash: Binary(hash),
        source: info.sender,
        created: msg.created,
    };

    // Try to store it, fail if the id already exists
    ANCHORS.update(deps.storage, &msg.id, |existing| match existing {
        None => Ok(anchor),
        Some(_) => Err(ContractError::AlreadyExists {}),
    })?;

    let res = Response::new()
        .add_attribute("action", "create")
        .add_attribute("id", msg.id)
        .add_attribute("hash", msg.hash)
        .add_attribute("account", msg.account);
    Ok(res)
}

fn parse_hex_32(data: &str) -> Result<Vec<u8>, ContractError> {
    match hex::decode(data) {
        Ok(bin) => {
            if bin.len() == 32 {
                Ok(bin)
            } else {
                Err(ContractError::InvalidHash(bin.len() * 2))
            }
        }
        Err(e) => Err(ContractError::ParseError(e.to_string())),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Details { id } => to_binary(&query_details(deps, id)?),
    }
}

fn query_details(deps: Deps, id: String) -> StdResult<DetailsResponse> {
    let anchor = ANCHORS.load(deps.storage, &id)?;

    let details = DetailsResponse {
        id,
        account: anchor.account.into(),
        hash: hex::encode(anchor.hash.as_slice()),
        source: anchor.source.into(),
        created: anchor.created.into(),
    };
    Ok(details)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Timestamp};
    use crate::state::all_anchor_ids;
    use sha2::{Digest, Sha256};
    use super::*;

    fn preimage() -> String {
        hex::encode(b"This is a string, 32 bytes long.")
    }

    fn custom_preimage(int: u16) -> String {
        hex::encode(format!("This is a custom string: {:>7}", int))
    }

    fn real_hash() -> String {
        hex::encode(&Sha256::digest(&hex::decode(preimage()).unwrap()))
    }

    fn custom_hash(int: u16) -> String {
        hex::encode(&Sha256::digest(&hex::decode(custom_preimage(int)).unwrap()))
    }

    fn mock_instantiate_msg() -> InstantiateMsg {
        let alice = "alice";
        let bob = "bob";
        let carl = "carl";
        let ted = "ted";
        let instantiate_msg = InstantiateMsg {
            admins: vec![alice.to_string(), bob.to_string(), carl.to_string()],
            users: vec![ted.to_string()],
            mutable: true,
        };
        instantiate_msg
    }


    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();

        let info = mock_info("anyone", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, mock_instantiate_msg()).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_create() {
        let mut deps = mock_dependencies();

        let info = mock_info("anyone", &[]);
        instantiate(deps.as_mut(), mock_env(), info, mock_instantiate_msg()).unwrap();

        let sender = String::from("sender0001");
        let balance = coins(100, "tokens");
        let valid_id = String::from("012345678901234567890123");

        // Cannot create, invalid ids
        let info = mock_info(&sender, &balance);
        for id in &["aa", "aaaabbbbccccd"] {
            let create = CreateMsg {
                id: id.to_string(),
                hash: real_hash(),
                account: String::from("acct0001"),
                created: Timestamp::from_seconds(1),
            };
            let err = execute(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                ExecuteMsg::Create(create.clone()),
            )
            .unwrap_err();
            assert_eq!(err, ContractError::InvalidId {});
        }

        // Cannot create, invalid hash
        let info = mock_info(&sender, &balance);
        let create = CreateMsg {
            id: valid_id.clone(),
            hash: "bu115h17".to_string(),
            account: String::from("acct0001"),
            created: Timestamp::from_seconds(1),
        };
        let err = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Create(create)).unwrap_err();
        assert_eq!(
            err,
            ContractError::ParseError("Invalid character \'u\' at position 1".into())
        );

        // Can create, all valid
        let info = mock_info(&sender, &balance);
        let create = CreateMsg {
            id: valid_id.clone(),
            hash: real_hash(),
            account: String::from("acct0001"),
            created: Timestamp::from_seconds(1),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::Create(create)).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(("action", "create"), res.attributes[0]);

        // Cannot re-create (modify), already existing
        let create = CreateMsg {
            id: valid_id.clone(),
            hash: real_hash(),
            account: String::from("acct0001"),
            created: Timestamp::from_seconds(1),
        };
        let err = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::Create(create)).unwrap_err();
        assert_eq!(err, ContractError::AlreadyExists {});
    }

    #[test]
    fn test_query() {
        let mut deps = mock_dependencies();

        let info = mock_info("anyone", &[]);
        instantiate(deps.as_mut(), mock_env(), info, mock_instantiate_msg()).unwrap();

        let sender1 = String::from("sender0001");
        let sender2 = String::from("sender0002");
        let valid_id1 = String::from("012345678901234567890123");
        let valid_id2 = String::from("012345678901234567890124");

        // Create 2 anchors
        let info = mock_info(&sender1, &[]);
        let create1 = CreateMsg {
            id: valid_id1,
            hash: custom_hash(1),
            account: String::from("acct0001"),
            created: Timestamp::from_seconds(1),
        };
        execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Create(create1.clone()),
        )
        .unwrap();

        let info = mock_info(&sender2, &[]);
        let create2 = CreateMsg {
            id: valid_id2,
            hash: custom_hash(2),
            account: String::from("acct0002"),
            created: Timestamp::from_seconds(2),
        };
        execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::Create(create2.clone()),
        )
        .unwrap();

        let ids = all_anchor_ids(deps.as_mut().storage, None, 10).unwrap();
        assert_eq!(2, ids.len());
        // Get the details for the first anchor id
        let query_msg = QueryMsg::Details {
            id: ids[0].clone(),
        };
        let res: DetailsResponse =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(
            res,
            DetailsResponse {
                id: create1.id,
                hash: create1.hash,
                account: create1.account,
                source: sender1,
                created: create1.created,
            }
        );

        // Get the details for the second anchor id
        let query_msg = QueryMsg::Details {
            id: ids[1].clone(),
        };
        let res: DetailsResponse =
            from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(
            res,
            DetailsResponse {
                id: create2.id,
                hash: create2.hash,
                account: create2.account,
                source: sender2,
                created: create2.created,
            }
        );
    }
}
