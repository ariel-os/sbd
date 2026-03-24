pub mod ariel;
pub mod common;
pub mod riot;

use std::collections::BTreeSet;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_with::{KeyValueMap, serde_as};

use crate::{
    ariel::{Ariel, ArielTargetExt},
    common::StringOrVecString,
    riot::{Riot, RiotTargetExt},
};

const fn default_version() -> Version {
    semver::Version::new(0, 2, 0)
}

/// Returns the used schema version.
#[must_use]
pub fn schema_version() -> Version {
    #[expect(
        clippy::missing_panics_doc,
        reason = "this is expected to be correct at compile time"
    )]
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SbdFile {
    #[serde(default = "default_version")]
    pub version: Version,
    pub include: Option<Vec<String>>,
    #[serde_as(as = "Option<KeyValueMap<_>>")]
    pub targets: Option<Vec<Target>>,
    pub ariel: Option<Ariel>,
    pub riot: Option<Riot>,
    pub description: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
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
    pub ariel: ArielTargetExt,
    #[serde(default)]
    pub riot: RiotTargetExt,
    pub debugger: Option<Debugger>,

    // peripheral types
    #[serde_as(as = "Option<KeyValueMap<_>>")]
    pub leds: Option<Vec<Led>>,
    #[serde_as(as = "Option<KeyValueMap<_>>")]
    pub buttons: Option<Vec<Button>>,
    #[serde_as(as = "Option<KeyValueMap<_>>")]
    pub uarts: Option<Vec<Uart>>,
    pub spi: Option<SpiBus>,
}

impl Target {
    #[must_use]
    pub fn has_leds(&self) -> bool {
        if let Some(leds) = &self.leds {
            !leds.is_empty()
        } else {
            false
        }
    }

    #[must_use]
    pub fn has_buttons(&self) -> bool {
        if let Some(buttons) = &self.buttons {
            !buttons.is_empty()
        } else {
            false
        }
    }

    #[must_use]
    pub fn has_spi(&self) -> bool {
        self.spi.is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Led {
    #[serde(rename = "$key$")]
    pub name: String,
    pub pin: String,
    pub color: Option<String>,
    pub active: Option<PinActive>,
    #[serde(default)]
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Button {
    #[serde(rename = "$key$")]
    pub name: String,
    pub pin: String,
    pub active: Option<PinActive>,
    #[serde(default)]
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum PinActive {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Quirk {
    SetPin(SetPinOp),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetPinOp {
    pub description: Option<String>,
    pub pin: String,
    pub level: PinLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum PinLevel {
    #[default]
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Debugger {
    #[serde(rename = "type")]
    pub type_: String,
    pub uart: Option<Uart>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Uart {
    #[serde(rename = "$key$")]
    pub name: Option<String>,
    pub rx_pin: String,
    pub tx_pin: String,
    pub cts_pin: Option<String>,
    pub rts_pin: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpiBus {
    pub miso: String,
    pub mosi: String,
    pub sck: String,
    #[serde_as(as = "Option<KeyValueMap<_>>")]
    pub devices: Option<Vec<SpiDevice>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpiDevice {
    #[serde(rename = "$key$")]
    pub name: String,
    pub cs: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SbdFileVersion {
    #[serde(default = "default_version")]
    pub version: Version,
}

impl SbdFileVersion {
    /// Returns whether this version is compatible with this schema version.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        #[expect(
            clippy::missing_panics_doc,
            reason = "any valid version is also a valid version requirement"
        )]
        let req = VersionReq::parse(&self.version.to_string()).unwrap();

        req.matches(&schema_version())
    }
}
