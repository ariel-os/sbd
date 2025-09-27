//! Pin name to tuple conversion
//!
//! This has been AI generated, by Gemini 2.5 flash, with the following prompt:
//! ```
//! I need a rust regex function that turns gpio names like P0_01, P1_12, GPIO15 into a (port, pin) tuple.
//! ```
//!

/// Parses a GPIO name string into a (port, pin) tuple.
///
/// This function handles several common formats for GPIO names, including:
/// - `P0_01`, `P1_12`, etc.
/// - GPIO15, GPIO23, etc.
///
/// # Arguments
///
/// * `gpio_name` - A string slice containing the GPIO name.
///
/// # Returns
///
/// An `Option<(u8, u8)>` which is:
/// - `Some((port, pin))` if the name was successfully parsed.
/// - `None` if the format is not recognized.
pub fn parse_gpio_name(gpio_name: &str) -> Option<(u8, u8)> {
    // Regex for "P<port>_<pin>" format (e.g., P0_01, P1_12)
    let p_regex = lazy_regex::regex!(r"^P(\d+)_(\d+)$");
    if let Some(captures) = p_regex.captures(gpio_name) {
        let port_str = captures.get(1).unwrap().as_str();
        let pin_str = captures.get(2).unwrap().as_str();
        if let (Ok(port), Ok(pin)) = (port_str.parse::<u8>(), pin_str.parse::<u8>()) {
            return Some((port, pin));
        }
    }

    // Regex for "GPIO<pin>" format (e.g., GPIO15)
    let gpio_regex = lazy_regex::regex!(r"^GPIO(\d+)$");
    if let Some(captures) = gpio_regex.captures(gpio_name) {
        let pin_str = captures.get(1).unwrap().as_str();
        if let Ok(pin) = pin_str.parse::<u8>() {
            // For this format, the port is typically 0, and the pin is the full number.
            return Some((0, pin));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_p_format() {
        assert_eq!(parse_gpio_name("P0_01"), Some((0, 1)));
        assert_eq!(parse_gpio_name("P1_12"), Some((1, 12)));
        assert_eq!(parse_gpio_name("P12_34"), Some((12, 34)));
    }

    #[test]
    fn test_parse_gpio_format() {
        assert_eq!(parse_gpio_name("GPIO15"), Some((0, 15)));
        assert_eq!(parse_gpio_name("GPIO23"), Some((0, 23)));
    }

    #[test]
    fn test_invalid_format() {
        assert_eq!(parse_gpio_name("P0_A1"), None);
        assert_eq!(parse_gpio_name("P_01"), None);
        assert_eq!(parse_gpio_name("GPIO_15"), None);
        assert_eq!(parse_gpio_name("p0_01"), None); // Case sensitive
        assert_eq!(parse_gpio_name("INVALID"), None);
    }
}
