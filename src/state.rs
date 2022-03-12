use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Binary, Order, StdResult, Storage, Timestamp};
use cw_storage_plus::{Bound, Map};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Anchor {
    pub account: String,
    pub hash: Binary,
    pub source: Addr,
    pub created: Timestamp,
}

pub const ANCHORS: Map<&str, Anchor> = Map::new("anchors");

/// This returns the list of ids for all active anchors
pub fn all_anchor_ids<'a>(
    storage: &dyn Storage,
    start: Option<Bound<'a, &'a str>>,
    limit: usize,
) -> StdResult<Vec<String>> {
    ANCHORS
        .keys(storage, start, None, Order::Ascending)
        .take(limit)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::MockStorage;
    use cosmwasm_std::Binary;

    #[test]
    fn test_no_anchor_ids() {
        let storage = MockStorage::new();
        let ids = all_anchor_ids(&storage, None, 10).unwrap();
        assert_eq!(0, ids.len());
    }

    fn dummy_anchor() -> Anchor {
        Anchor {
            account: Default::default(),
            source: Addr::unchecked("source"),
            hash: Binary("hash".into()),
            created: Default::default(),
        }
    }

    #[test]
    fn test_all_anchor_ids() {
        let mut storage = MockStorage::new();
        ANCHORS.save(&mut storage, "lazy", &dummy_anchor()).unwrap();
        ANCHORS.save(&mut storage, "assign", &dummy_anchor()).unwrap();
        ANCHORS.save(&mut storage, "zen", &dummy_anchor()).unwrap();

        let ids = all_anchor_ids(&storage, None, 10).unwrap();
        assert_eq!(3, ids.len());
        assert_eq!(
            vec!["assign".to_string(), "lazy".to_string(), "zen".to_string()],
            ids
        )
    }
}
