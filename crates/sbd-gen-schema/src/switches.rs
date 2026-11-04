//! Structs used to describe various forms of electrical switches.
//!
//! This module is meant to grow if the need for specific support of
//! physical realisation of switches arises. This would take the form of structs
//! that are specific to a single type switchthat can use and encode domain-specific
//! information such as mutually exclusive states in jumper switches or assigning
//! meaning to the combined state of a row of DIP switches.

use serde::{Deserialize, Serialize};

use crate::{
    PinActive, PullResistor,
    common::{StringOrVecString, TOrVecT},
};

/// A Push-button.
///
/// This assumes monostability but will work for bi-stable buttons as well.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Button {
    /// Pin of the MCU that reads the state of the button.
    pub pin: String,
    /// Resistor defining the state of the input when otherwise floating.
    pub resistor: PullResistor,
    /// Active state of the pin.
    pub active: PinActive,
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// A Switch in the electrical sense regardless of its physical form.
///
/// This is meant mainly for toggle switches and can be used to describe other
/// types of switches like push-buttons, jumper switches or DIP switches.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Switch {
    /// Pins of the switch that are connected to input pins.
    pub pins: StringOrVecString,
    /// Resistors defining the state of the inputs when they are otherwise floating.
    pub resistors: TOrVecT<PullResistor>,
    /// Active state of the pins.
    pub active: TOrVecT<PinActive>,
    #[serde(default)]
    pub aliases: Vec<String>,
}
