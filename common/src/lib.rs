use config::YASERDE_CFG;
use packages::traits::SEResource;
use yaserde::de::from_str;
use yaserde::ser::to_string_with_config;

pub mod config;
pub mod packages;
pub mod subscription;
pub mod tls;

pub fn serialize<R: SEResource>(resource: R) -> String {
    to_string_with_config(&resource, &YASERDE_CFG).unwrap()
}

pub fn deserialize<R: SEResource>(resource: &str) -> R {
    from_str(resource).unwrap()
}
