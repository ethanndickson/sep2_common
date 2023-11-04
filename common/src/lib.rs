use std::panic::{self};

use anyhow::{anyhow, Result};
use traits::SEType;
use yaserde::de::from_str;
use yaserde::ser::to_string;
#[cfg(feature = "examples")]
pub mod examples;
pub mod object;
pub mod packages;
pub mod traits;

/// Given an IEEE 2030.5 data type, serialize it into an XML string
pub fn serialize<R: SEType>(resource: &R) -> Result<String> {
    log::debug!("Serializing: {}", R::name());
    panic::catch_unwind(|| to_string(resource).map_err(|e| anyhow!(e))).map_err(|_| {
        anyhow!(
            "Fatal Serializer Error: Unable to Serialize {} due to a panic",
            R::name()
        )
    })?
}

/// Given a string representing an IEEE 2030.5 data type, deserialize into it the inferred type
pub fn deserialize<R: SEType>(resource: &str) -> Result<R> {
    log::debug!("Deserializing: {}", R::name());
    panic::catch_unwind(|| from_str::<R>(resource).map_err(|e| anyhow!(e))).map_err(|_| {
        anyhow!(
            "Fatal XML Parser Error: Unable to Deserialize {} due to a panic",
            R::name()
        )
    })?
}
