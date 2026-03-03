mod ariel;
pub mod common;
mod riot;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_with::{KeyValueMap, serde_as};

use crate::{
    ariel::{Ariel, ArielBoardExt},
    common::StringOrVecString,
    riot::{Riot, RiotBoardExt},
};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SbdFile {
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
    pub fn has_uarts(&self) -> bool {
        if let Some(uarts) = &self.uarts {
            !uarts.is_empty()
        } else {
            false
        }
    }

    #[must_use]
    pub fn has_host_facing_uart(&self) -> bool {
        if let Some(uarts) = &self.uarts {
            uarts.iter().any(|u| u.host_facing)
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
    /// Peripheral device names, any of which is fundamentally available to serve this connection
    /// as the peripheral that takes control of the TX and RX pins.
    ///
    /// # Usage
    ///
    /// All items in the list are peripheral names of the MCU for which the UART interface is
    /// implemented. For example, on EFM32, a pin combination might be configurable either using
    /// `LEUART0` or `USART1`, in which case those are given as values.
    ///
    /// On some OSes and platforms (e. g., at the time of writing, in ArielOS on nRF devices),
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
