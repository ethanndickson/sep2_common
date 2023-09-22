use crate::packages::{primitives::HexBinary128, types::MRIDType};
use rand::Rng;
use std::{
    sync::atomic::AtomicU32,
    time::{SystemTime, UNIX_EPOCH},
};

static MRID_COUNT: AtomicU32 = AtomicU32::new(0);

/// Given the IANA Private Enterprise Number (PEN) ID, produce a MRID
pub fn mrid_gen(pen_id: u32) -> MRIDType {
    let mut rng = rand::thread_rng();
    let id: u128 = rng.gen();
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards.")
        .as_secs() as u128;
    let count = MRID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) as u128;
    HexBinary128((time << 96) | (id << 64) | (count << 32) | pen_id as u128)
}
