#![deny(missing_docs)]
//! I2C related structs of the SBD schema.


use std::{convert::Infallible, str::FromStr};

use serde::{Deserialize, Serialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};
use core::marker::PhantomData;
use std::fmt;

/// Represents a complete I2C Bus, including the devices.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct I2cBus {
    /// Serial Clock Line pin.
    pub scl_pin: String,
    /// Serial Data Line pin.
    pub sda_pin: String,
    /// User-defined aliases for the bus
    #[serde(default)]
    pub aliases: Vec<String>,
    /// List of devices connected to the bus
    #[serde(default)]
    pub devices: Vec<I2cDevice>,
    /// List of possible MCU I2C peripherals usable for the Bus
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
    #[serde(rename = "type", deserialize_with = "string_or_struct")]
    type_: I2CDeviceType,

    /// Part name of the device coming from the supplier.
    ///
    /// If given, this can be used to automatically generate
    /// the required initialization code.
    /// Right now, this is only possible for sensors for which there is
    /// an `ariel-os-sensor-x` crate.
    #[serde(default)]
    part_name: Option<String>,
    /// Device address on bus in hex.
    address: String,
}


/// Device types.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum I2CDeviceType {
    /// A sensor of a specific type.
    ///
    /// The associated string should be an appropriate label
    #[serde(rename = "sensor")]
    Sensor(String),

    /// Other types of devices.
    Other(String)
}

// Below is copied from https://serde.rs/string-or-struct.html

impl FromStr for I2CDeviceType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(I2CDeviceType::Other(s.to_string()))
    }
}


fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Infallible>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Infallible>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
