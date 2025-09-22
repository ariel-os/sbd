use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_with::{KeyValueMap, serde_as};

use crate::{
    ariel::{Ariel, ArielBoardExt},
    riot::{Riot, RiotBoardExt},
};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SbdFile {
    pub description: Option<String>,
    pub include: Option<Vec<String>>,
    #[serde_as(as = "Option<KeyValueMap<_>>")]
    pub boards: Option<Vec<Board>>,
    pub ariel: Option<Ariel>,
    pub riot: Option<Riot>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Board {
    #[serde(rename = "$key$")]
    pub name: String,
    pub chip: String,
    pub description: Option<String>,
    pub include: Option<Vec<String>>,
    #[serde(default)]
    pub flags: BTreeSet<String>,
    #[serde(default)]
    pub quirks: Vec<Quirk>,
    #[serde(default)]
    pub ariel: ArielBoardExt,
    #[serde(default)]
    pub riot: RiotBoardExt,
    pub debugger: Option<Debugger>,

    // peripheral types
    #[serde_as(as = "Option<KeyValueMap<_>>")]
    pub leds: Option<Vec<Led>>,
    #[serde_as(as = "Option<KeyValueMap<_>>")]
    pub buttons: Option<Vec<Button>>,
    #[serde_as(as = "Option<KeyValueMap<_>>")]
    pub uarts: Option<Vec<Uart>>,
}

impl Board {
    pub fn has_leds(&self) -> bool {
        if let Some(leds) = &self.leds {
            !leds.is_empty()
        } else {
            false
        }
    }

    pub fn has_buttons(&self) -> bool {
        if let Some(buttons) = &self.buttons {
            !buttons.is_empty()
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Led {
    #[serde(rename = "$key$")]
    pub name: String,
    pub pin: String,
    pub color: Option<String>,
    pub inverted: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Button {
    #[serde(rename = "$key$")]
    pub name: String,
    pub pin: String,
    pub active_low: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Quirk {
    SetPin(SetPinOp),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SetPinOp {
    pub description: Option<String>,
    pub pin: String,
    pub level: PinLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PinLevel {
    #[default]
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Debugger {
    #[serde(rename = "type")]
    pub type_: String,
    pub uart: Option<Uart>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uart {
    #[serde(rename = "$key$")]
    pub name: Option<String>,
    pub rx_pin: String,
    pub tx_pin: String,
    pub cts_pin: Option<String>,
    pub rts_pin: Option<String>,
}
