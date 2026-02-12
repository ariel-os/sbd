use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Riot {
    pub chips: BTreeMap<String, RiotChipMapEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotChipMapEntry {
    pub cpu: String,
    pub cpu_model: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub quirks: BTreeMap<String, RiotQirkEntry>,
    pub peripherals: Option<RiotChipPeripherals>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotBoardExt {
    // TODO
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotQirkEntry {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotChipPeripherals {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub uarts: BTreeMap<String, RiotChipUartPeripheral>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RiotChipUartPeripheral {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub config: BTreeMap<String, String>,
    pub isr: Option<String>,
}
