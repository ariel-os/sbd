#![deny(missing_docs)]
//! Light emitting devices consisting of Light emitting diodes (LED).

use crate::PinActive;
use serde::{Deserialize, Serialize};

/// Dispatching enum between different types of LEDs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Led {
    /// Monocolor LEDs
    Monocolor(MonocolorLed),
    /// Duocolor LEDs
    Duocolor(DuocolorLed),
    /// Tricolor LEDs
    Tricolor(TricolorLed),
    /// Tetracolor LEDs
    Tetracolor(TetracolorLed),
    /// Pentacolor LEDs
    Pentacolor(PentacolorLed),
}


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

/// Duocolor LED.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DuocolorLed {
    /// Pins of the MCU connected to the LED.
    pub pins: [String; 2],
    /// Color of the LED.
    pub colors: Option<[String; 2]>,
    /// Whether the LED is active high or low.
    /// We assume that it is the same for each pin.
    pub active: Option<PinActive>,
    /// Possible aliases of the LED.
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Tricolor LED.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TricolorLed {
    /// Pins of the MCU connected to the LED.
    pub pins: [String; 3],
    /// Color of the LED.
    pub colors: Option<[String; 3]>,
    /// Whether the LED is active high or low.
    /// We assume that it is the same for each pin.
    pub active: Option<PinActive>,
    /// Possible aliases of the LED.
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Tetracolor LED.
///
/// This is meant to be used for:
/// - RGBA (RGB plus Amber).
/// - RGBCW/RGBW (RGB plus Cool White).
/// - RGBWW (RGB plus Warm White).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TetracolorLed {
    /// Pins of the MCU connected to the LED.
    pub pins: [String; 4],
    /// Color of the LED.
    pub colors: Option<[String; 4]>,
    /// Whether the LED is active high or low.
    /// We assume that it is the same for each pin.
    pub active: Option<PinActive>,
    /// Possible aliases of the LED.
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Pentacolor LED.
///
/// This is meant to be used for:
/// - RGBACW (RGB plus Amber plus cool white)
/// - RGB-CW-WW (RGB plus Cool White plus Warm White)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PentacolorLed {
    /// Pins of the MCU connected to the LED.
    pub pins: [String; 5],
    /// Color of the LED.
    pub colors: Option<[String; 5]>,
    /// Whether the LED is active high or low.
    /// We assume that it is the same for each pin.
    pub active: Option<PinActive>,
    /// Possible aliases of the LED.
    #[serde(default)]
    pub aliases: Vec<String>,
}
