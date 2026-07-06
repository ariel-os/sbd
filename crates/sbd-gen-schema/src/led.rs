#![deny(missing_docs)]
//! Light emitting devices consisting of Light emitting diodes (LED).

use crate::PinActive;
use serde::{Deserialize, Serialize};

/// Monocolor LED.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonocolorLed {
    /// Pin of the MCU connected to the LED.
    pub pin: String,
    /// Color of the LED.
    pub color: Option<String>,
    /// Whether the LED is active high or low.
    pub active: Option<PinActive>,
    /// Possible aliases of the LED.
    #[serde(default)]
    pub aliases: Vec<String>,
}
