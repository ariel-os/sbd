use crate::PinActive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonocolorLed {
    pub pin: String,
    pub color: Option<String>,
    pub active: Option<PinActive>,
    #[serde(default)]
    pub aliases: Vec<String>,
}
