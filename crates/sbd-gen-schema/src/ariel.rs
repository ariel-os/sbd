use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::StringOrVecString;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Ariel {
    pub chips: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ArielBoardExt {
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub flags: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub global_env: BTreeMap<String, StringOrVecString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swi: Option<String>,
}
