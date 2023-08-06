use anyhow::{anyhow, Result};
use config::YASERDE_CFG;
use log::debug;
use packages::traits::SEResource;
use yaserde::de::from_str;
use yaserde::ser::to_string_with_config;
pub mod config;
pub mod examples;
pub mod packages;

pub fn serialize<R: SEResource>(resource: &R) -> Result<String> {
    debug!("Serialize: {}", R::name());
    to_string_with_config(resource, &YASERDE_CFG).map_err(|e| anyhow!(e))
}

pub fn deserialize<R: SEResource>(resource: &str) -> Result<R> {
    debug!("Deserialize: {}", R::name());
    from_str(resource).map_err(|e| anyhow!(e))
}
