#![deny(missing_docs)]
//! I2C related structs of the SBD schema.

use serde::{Deserialize, Serialize};

/// Represents a complete I2C Bus, including the devices.
///
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
    /// List of devices connected to the bus.
    #[serde(default)]
    pub devices: Vec<I2cDevice>,
    /// List of possible MCU I2C peripherals usable for this bus.
    #[serde(default)]
    pub possible_peripherals: Vec<String>,
}

/// A singular I2C "slave" device.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct I2cDevice {
    /// Device type.
    ///
    /// This is supposed to be an *identifier*.
    /// In the Ariel OS generator, this will set the flag `has_i2c_device_<type>`.
    #[serde(rename = "type")]
    pub type_: String,

    /// Part name of the device coming from the supplier.
    ///
    /// If given, this can be used to automatically generate
    /// the required initialization code.
    /// Right now, this is only possible for sensors for which there is
    /// an `ariel-os-sensor-x` crate.
    #[serde(default)]
    pub part_number: Option<String>,
    /// Device address on bus in hex.
    pub address: String,

    /// Optional description.
    ///
    /// This is mainly meant for devices that aren't supported
    /// by Ariel OS
    #[serde(default)]
    pub description: Option<String>,
}
