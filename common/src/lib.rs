use anyhow::{anyhow, Result};
use traits::SEResource;
use yaserde::de::from_str;
use yaserde::ser::to_string;
pub mod examples;
pub mod packages;
pub mod traits;

pub fn serialize<R: SEResource>(resource: &R) -> Result<String> {
    log::debug!("Serialize: {}", R::name());
    to_string(resource).map_err(|e| anyhow!(e))
}

pub fn deserialize<R: SEResource>(resource: &str) -> Result<R> {
    log::debug!("Deserialize: {}", R::name());
    from_str(resource).map_err(|e| anyhow!(e))
}
