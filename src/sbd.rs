use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_with::{KeyValueMap, serde_as};

use crate::{
    ariel::{Ariel, ArielBoardExt},
    riot::{Riot, RiotBoardExt},
};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SbdFile {
    pub include: Option<Vec<String>>,
    #[serde_as(as = "Option<KeyValueMap<_>>")]
    pub boards: Option<Vec<Board>>,
    pub ariel: Option<Ariel>,
    pub riot: Option<Riot>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

    pub fn has_uarts(&self) -> bool {
        if let Some(uarts) = &self.uarts {
            !uarts.is_empty()
        } else {
            false
        }
    }

    pub fn has_host_facing_uart(&self) -> bool {
        if let Some(uarts) = &self.uarts {
            uarts.iter().any(|u| u.host_facing)
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// FIXME: update Ariel and then #[serde(deny_unknown_fields)]
pub struct Led {
    #[serde(rename = "$key$")]
    pub name: String,
    pub pin: String,
    pub color: Option<String>,
    pub inverted: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// FIXME: update Ariel and then #[serde(deny_unknown_fields)]
pub struct Button {
    #[serde(rename = "$key$")]
    pub name: String,
    pub pin: String,
    pub active_low: Option<bool>,
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
    /// Peripheral device names, each of which is fundamentally available to serve this connection
    /// as the peripheral that takes control of the TX and RX pins.
    ///
    /// This item is on the fringe of being a fact about the board, because while it is a fact, it
    /// is a pure function of the MCU and the assigned pins.
    ///
    /// The way this is used in Ariel is also just borderline correct, as Ariel OS's UART devices
    /// are (at least on platforms such as nRF) composite devices that combine a UART driver with
    /// several other related peripherals (eg. `UARTE0 => UARTE0 + TIMER4 + PPI_CH14 + PPI_CH15 +
    /// PPI_GROUP5`), encompassing an instance of MCU specific information that is encoded in the
    /// Ariel OS source code. The way these names are used there is correct under the (currently
    /// valid) assumption that the composite items are named after the main UART driver they
    /// include. (Also, currently, Ariel OS picks the first of them, while generally this is not an
    /// ordered structure).
    pub possible_peripherals: Option<Vec<String>>,

    /// Set if the board supports using it with a host system (e.g. the build host), and this UART
    /// would typically face that system.
    ///
    /// For example, this is set on boards with built-in programmers on UARTs that are exposed by
    /// the programmer as USB serial devices. Typical applications querying this are tools that
    /// reprot debug or measurement data.
    #[serde(default)]
    pub host_facing: bool,
}
