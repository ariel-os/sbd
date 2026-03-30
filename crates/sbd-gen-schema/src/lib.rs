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
///
/// - If the new schema adds new functionality but doesn't break old schema files: only a non-SemVer-breaking bump is required
/// - Otherwise, this needs a SemVer-breaking bump.
///
/// In both cases, the schema version must be updated accordingly.
#[must_use]
pub const fn schema_version() -> Version {
    semver::Version::new(0, 3, 1)
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
    #[serde(default)]
    pub leds: Vec<Led>,
    #[serde(default)]
    pub buttons: Vec<Button>,
    #[serde_as(as = "Option<KeyValueMap<_>>")]
    pub uarts: Option<Vec<Uart>>,
}

impl Target {
    #[must_use]
    pub fn has_leds(&self) -> bool {
        !self.leds.is_empty()
    }

    #[must_use]

    pub fn has_buttons(&self) -> bool {
        !self.buttons.is_empty()
    }

    /// Returns true if there are any UARTs listed for this board.
    #[must_use]
    pub fn has_uarts(&self) -> bool {
        if let Some(uarts) = &self.uarts {
            !uarts.is_empty()
        } else {
            false
        }
    }

    /// Returns true if there are any UARTs listed for this board that have the
    /// [`Uart::host_facing`] property.
    #[must_use]
    pub fn has_host_facing_uart(&self) -> bool {
        if let Some(uarts) = &self.uarts {
            uarts.iter().any(|u| u.host_facing)
        } else {
            false
        }
    }

    pub fn test_default() -> Self {
        Self {
            name: "test-target".to_string(),
            ariel: ArielTargetExt::default(),
            buttons: vec![],
            chip: "test-chip".to_string(),
            debugger: None,
            description: None,
            flags: BTreeSet::default(),
            include: None,
            leds: vec![],
            quirks: vec![],
            riot: RiotTargetExt::default(),
            uarts: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Led {
    pub pin: String,
    pub color: Option<String>,
    pub active: Option<PinActive>,
    #[serde(default)]
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Button {
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
    /// Peripheral device names, any of which is fundamentally available to serve this connection
    /// as the peripheral that takes control of the TX and RX pins.
    ///
    /// # Usage
    ///
    /// All items in the list are peripheral names of the MCU for which the UART interface is
    /// implemented. For example, on EFM32, a pin combination might be configurable either using
    /// `LEUART0` or `USART1`, in which case those are given as values.
    ///
    /// On some OSes and platforms (e. g., at the time of writing, in Ariel OS on nRF devices),
    /// using that device name might entail using companion peripherals that are statically
    /// selected (e. g. `UARTE0` being bundled with `TIMER4`, `PPI_CH14`, `PPI_CH15` and
    /// `PPI_GROUP5`). This is an implementation detail of the OS; the name in this list is still
    /// only the name of the one peripheral that performs the UART functionality.
    ///
    /// # Future development
    ///
    /// When future versions of `sbd` or the OSes consuming this file learn to process per-MCU
    /// information, this field might go away. Instead, the possible peripherals might be deduced
    /// purely from the MCU's peripheral mapping and the `*_pin` values.
    ///
    /// When multiple UARTs are in use in an application and their possible peripherals overlap,
    /// deciding which of the choices to take is a [hard problem]. When none of the peripherals are
    /// available, the OS's mechanism of choosing a peripheral may need enhancing: For example,
    /// Ariel OS (at the time of writing) only selects the first peripheral. Future versions might
    /// pick the first one that has not previously been taken, and ideally, a static choice would
    /// be made at build time solving the satisfiability problem.
    ///
    /// When no peripheral is given, or all are used for other purposes, the OS may fall back to
    /// bit-banging operation; currently, they do not.
    ///
    /// [hard problem]: https://en.wikipedia.org/wiki/Boolean_satisfiability_problem
    #[serde(default)]
    pub possible_peripherals: Vec<String>,

    /// Set if the board supports using it with a host system (e.g. the build host), and this UART
    /// would typically face that system.
    ///
    /// For example, this is set on boards with built-in programmers on UARTs that are exposed by
    /// the programmer as USB serial devices. Typical applications querying this are tools that
    /// report debug or measurement data.
    #[serde(default)]
    pub host_facing: bool,
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
