use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io,
    path::Path,
};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use sbd_gen_schema::common::StringOrVecString;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LazeFile {
    pub contexts: Option<Vec<LazeContext>>,
    pub builders: Option<Vec<LazeContext>>,
}

impl LazeFile {
    pub fn new() -> Self {
        Self::default()
    }

    #[expect(dead_code)]
    pub fn to_file(&self, path: &Path) -> Result<(), io::Error> {
        let mut file = File::create(path)?;
        serde_yaml::to_writer(&mut file, self).map_err(io::Error::other)?;
        Ok(())
    }

    pub fn to_string(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LazeContext {
    pub name: String,
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    pub provides: BTreeSet<String>,
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    pub requires: BTreeSet<String>,
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    pub selects: BTreeSet<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub env: BTreeMap<String, StringOrVecString>,
}

impl LazeContext {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}
