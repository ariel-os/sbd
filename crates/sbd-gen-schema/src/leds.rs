use serde::{Deserialize, Serialize};

use crate::{PinActive, common::StringOrVecString};

/// Monocolor LED.
///
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonocolorLed {
    #[serde(rename = "$key$")]
    pub name: String,
    /// Pin of the MCU connected to the LED.
    pub pin: String,
    /// Color of the LED.
    pub color: Option<String>,
    pub active: Option<PinActive>,
    /// Possible aliases of the LED.
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Bicolor LED.
///
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BicolorLed {
    #[serde(rename = "$key$")]
    pub name: String,
    /// Pins of the MCU connected to the LED.
    pub pins: [String; 2],
    /// Colors of the individual diodes of the bicolor led.
    pub colors: Option<[String; 2]>,
    pub active: Option<[PinActive; 2]>,
    #[serde(default)]
    /// Possible aliases of the LED.
    pub aliases: Vec<String>,
    /// Flag indicating if this is a LED with only two leads
    /// and no connection to the ground.
    #[serde(default)]
    pub two_leads: bool,
}

/// LED Matrix using ROW + COL inputs to control ROW * COL outputs.
///
/// This does not make any assumptions on the phyiscal and geographical
/// placement of the individual LEDs on the board. They maybe be wired in
/// square matrix, in a diagonal array or any other geometrical arrangement.
/// The only assumption is that the inputs are divided into two sets such that
/// driving one of each lights up a single led.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LedMatrix {
    #[serde(rename = "$key$")]
    pub name: String,
    /// Colors of each single LED or a single color if
    /// every LED in the matrix have the same color.
    pub colors: StringOrVecString,
    /// set of "row" input pins.
    pub row: Vec<String>,
    /// set of "column" input pins.
    pub col: Vec<String>,
    /// Logical level required to activate the leds.
    pub active: Option<PinActive>,
    /// Aliases for the LED matrix.
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// LED using a single wire a serial communication protocol, also called a Smart-Pixel.
///
/// One or more pixel can be controlled from a single wire.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SmartLed {
    #[serde(rename = "$key$")]
    pub name: String,
    /// Pin used for the serial communication protocol.
    pub pin: String,
    /// Number of addressable pixels.
    pub size: usize,
    /// Aliases for the smart pixel.
    #[serde(default)]
    pub aliases: Vec<String>,
}
