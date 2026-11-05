#![deny(missing_docs)]
//! Light emitting devices consisting of Light emitting diodes (LED).

use crate::PinActive;
use serde::{Deserialize, Serialize};

/// Dispatching enum between different types of LEDs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Led {
    /// Monocolor LED
    Monocolor(MonocolorLed),
    /// Duocolor LED
    Duocolor(DuocolorLed),
    /// Tricolor LED
    Tricolor(TricolorLed),
    /// Tetracolor LED
    Tetracolor(TetracolorLed),
    /// Pentacolor LED
    Pentacolor(PentacolorLed),
}

/// Monocolor LED.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonocolorLed {
    /// Pin of the MCU connected to the LED.
    pub pin: String,
    /// Color of the LED.
    pub color: String,
    /// Whether the LED is active high or low.
    pub active: PinActive,
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
    pub colors: [String; 2],
    /// Whether the LED is active high or low.
    /// We assume that it is the same for each pin.
    pub active: PinActive,
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
    pub colors: [String; 3],
    /// Whether the LED is active high or low.
    /// We assume that it is the same for each pin.
    pub active: PinActive,
    /// Possible aliases of the LED.
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Tetracolor LED.
///
/// This can for instance be used for:
/// - RGBA (RGB plus Amber).
/// - RGBCW/RGBW (RGB plus Cool White).
/// - RGBWW (RGB plus Warm White).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TetracolorLed {
    /// Pins of the MCU connected to the LED.
    pub pins: [String; 4],
    /// Color of the LED.
    pub colors: [String; 4],
    /// Whether the LED is active high or low.
    /// We assume that it is the same for each pin.
    pub active: PinActive,
    /// Possible aliases of the LED.
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Pentacolor LED.
///
/// This can for instance be used for:
/// - RGBACW (RGB plus Amber plus cool white)
/// - RGB+CCT (RGB plus Cool White plus Warm White)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PentacolorLed {
    /// Pins of the MCU connected to the LED.
    pub pins: [String; 5],
    /// Color of the LED.
    pub colors: [String; 5],
    /// Whether the LED is active high or low.
    /// We assume that it is the same for each pin.
    pub active: PinActive,
    /// Possible aliases of the LED.
    #[serde(default)]
    pub aliases: Vec<String>,
}

impl Led {
    /// Takes a [`Led`] variant and returns
    /// its pins, colors, activation level and aliases
    /// as a tuple of vectors.
    ///
    /// This is useful for downstream users of the [`Led`] types
    /// while preserving the upside of each variants which is that
    /// by construction there is an equal number of pins and colors declared
    /// for every variant.
    #[must_use]
    pub fn erase_variant(&self) -> (Vec<&String>, Vec<&String>, &PinActive, &Vec<String>) {
        match self {
            Led::Monocolor(l) => (vec![&l.pin], vec![&l.color], &l.active, &l.aliases),
            Led::Duocolor(l) => (
                l.pins.iter().collect(),
                l.colors.iter().collect(),
                &l.active,
                &l.aliases,
            ),
            Led::Tricolor(l) => (
                l.pins.iter().collect(),
                l.colors.iter().collect(),
                &l.active,
                &l.aliases,
            ),
            Led::Tetracolor(l) => (
                l.pins.iter().collect(),
                l.colors.iter().collect(),
                &l.active,
                &l.aliases,
            ),
            Led::Pentacolor(l) => (
                l.pins.iter().collect(),
                l.colors.iter().collect(),
                &l.active,
                &l.aliases,
            ),
        }
    }
}
