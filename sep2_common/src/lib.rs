use crate::packages::{primitives::HexBinary128, types::MRIDType};
use anyhow::{anyhow, Result};
use rand::Rng;
use std::{
    sync::atomic::AtomicU32,
    time::{SystemTime, UNIX_EPOCH},
};
use traits::SEType;

#[cfg(feature = "examples")]
pub mod examples;
pub mod packages;
pub mod traits;

/// Given an IEEE 2030.5 data type, serialize it into an XML string
pub fn serialize<R: SEType>(resource: &R) -> Result<String> {
    log::debug!("Serializing: {}", R::name());
    std::panic::catch_unwind(|| sepserde::ser::to_string(resource).map_err(|e| anyhow!(e)))
        .map_err(|_| {
            anyhow!(
                "Fatal Serializer Error: Unable to Serialize {} due to a panic",
                R::name()
            )
        })?
}

/// Given a string representing an IEEE 2030.5 data type, deserialize into it the inferred type
pub fn deserialize<R: SEType>(resource: &str) -> Result<R> {
    log::debug!("Deserializing: {}", R::name());
    std::panic::catch_unwind(|| sepserde::de::from_str::<R>(resource).map_err(|e| anyhow!(e)))
        .map_err(|_| {
            anyhow!(
                "Fatal XML Parser Error: Unable to Deserialize {} due to a panic",
                R::name()
            )
        })?
}

static MRID_COUNT: AtomicU32 = AtomicU32::new(0);

/// Given the IANA Private Enterprise Number (PEN) ID, produce a (unique) MRID
pub fn mrid_gen(pen_id: u32) -> MRIDType {
    let mut rng = rand::thread_rng();
    let id: u128 = rng.gen();
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards.")
        .as_secs() as u128;
    let count = MRID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) as u128;
    HexBinary128((time << 32) | (id << 32) | (count << 32) | pen_id as u128)
}

#[test]
fn mrid_contains_pen() {
    let pen: u32 = 1337;
    let out = mrid_gen(pen).0;
    assert_eq!(u128::from(pen), out & (pen as u128));
}

#[test]
fn mrid_unique() {
    let num_ids = 1_000_000;
    let mut id_set = std::collections::HashSet::new();
    for _ in 0..num_ids {
        let mrid = mrid_gen(0).0;
        assert!(!id_set.contains(&mrid), "Duplicate MRID generated");
        id_set.insert(mrid);
    }
}
