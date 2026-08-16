use std::{error::Error, fmt};

include!(concat!(env!("OUT_DIR"), "/paypal_country_code.rs"));

#[derive(Debug)]
pub enum CountryParseError {
    InvalidCountryCode(String),
}

impl Error for CountryParseError {
    fn description(&self) -> &str {
        "error parsing country code"
    }
}

impl fmt::Display for CountryParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for PaypalCountryCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PaypalCountryCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;

        s.parse::<Self>()
            .map_err(|_| serde::de::Error::custom(format!("invalid PayPal country code: {s}")))
    }
}
