use std::panic::{self};

use anyhow::{anyhow, bail, Result};
use traits::SEResource;
use yaserde::de::from_str;
use yaserde::ser::to_string;
pub mod examples;
pub mod object;
pub mod packages;
pub mod traits;

pub fn serialize<R: SEResource>(resource: &R) -> Result<String> {
    log::debug!("Serialize: {}", R::name());
    let res = panic::catch_unwind(|| to_string(resource).map_err(|e| anyhow!(e)));
    match res {
        Ok(res) => res,
        Err(_) => {
            bail!("Fatal Serializer Error: Unable to Serialize {}", R::name())
        }
    }
}

pub fn deserialize<R: SEResource>(resource: &str) -> Result<R> {
    log::debug!("Deserialize: {}", R::name());
    let res = panic::catch_unwind(|| from_str::<R>(resource).map_err(|e| anyhow!(e)));
    match res {
        Ok(res) => res,
        Err(_) => bail!(
            "Fatal XML Parser Error: Unable to Deserialize {}",
            R::name()
        ),
    }
}
