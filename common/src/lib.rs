use std::panic::{self};

use anyhow::{anyhow, Result};
use traits::SEResource;
use yaserde::de::from_str;
use yaserde::ser::to_string;
#[cfg(feature = "examples")]
pub mod examples;
pub mod object;
pub mod packages;
pub mod traits;

/// Given a top-level resource, serialize it into an XML string
pub fn serialize<R: SEResource>(resource: &R) -> Result<String> {
    log::debug!("Serialize: {}", R::name());
    let res = panic::catch_unwind(|| to_string(resource).map_err(|e| anyhow!(e)));
    match res {
        Ok(res) => res,
        Err(_) => Err(anyhow!(
            "Fatal Serializer Error: Unable to Serialize {}",
            R::name()
        )),
    }
}

/// Given a string representing a top-level resource, deserialize into it the inferred type
pub fn deserialize<R: SEResource>(resource: &str) -> Result<R> {
    log::debug!("Deserialize: {}", R::name());
    let res = panic::catch_unwind(|| from_str::<R>(resource).map_err(|e| anyhow!(e)));
    match res {
        Ok(res) => res,
        Err(_) => Err(anyhow!(
            "Fatal XML Parser Error: Unable to Deserialize {}",
            R::name()
        )),
    }
}
