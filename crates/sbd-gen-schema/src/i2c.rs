#![deny(missing_docs)]
//! I2C related structs of the SBD schema.

use serde::{Deserialize, Serialize};

/// Represents the MCU side of I2C Bus.
///
/// Notably I2C devices are currently absent from the schema.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct I2cBus {
    /// Serial Clock Line pin.
    pub scl_pin: String,
    /// Serial Data Line pin.
    pub sda_pin: String,
    /// User-defined aliases for the bus.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// List of possible MCU I2C peripherals usable for this bus.
    #[serde(default)]
    pub possible_peripherals: Vec<String>,
}
