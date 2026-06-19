use crate::packages::types::MRIDType;
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

#[cfg(feature = "csip_aus")]
const CSIPAUS_MAX_DECIMAL_PEN: u32 = 99_999_999;

/// An IANA Private Enterprise Number (PEN) pre-encoded for the least-significant
/// 32 bits of an mRID.
///
/// Use [`Pen::ieee2030_5`] for the base IEEE 2030.5 binary `UInt32` encoding.
#[cfg_attr(
    feature = "csip_aus",
    doc = " When the `csip_aus` feature is enabled, use [`Pen::csipaus`] for the CSIP-AUS / TS 5573 decimal-digits-as-hex encoding required by some client-generated identifiers."
)]
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
pub struct Pen(u32);

impl Pen {
    /// Encode a PEN according to base IEEE 2030.5: the raw `u32` value is placed
    /// in bits 0-31 of the mRID.
    pub const fn ieee2030_5(pen_id: u32) -> Self {
        Self(pen_id)
    }

    /// Encode a PEN according to CSIP-AUS / TS 5573: each decimal digit of the
    /// IANA PEN is placed into one hex nibble of the trailing 8 mRID characters.
    ///
    /// Returns `None` when the PEN cannot fit in 8 decimal digits.
    #[cfg(feature = "csip_aus")]
    pub fn csipaus(pen_id: u32) -> Option<Self> {
        if pen_id > CSIPAUS_MAX_DECIMAL_PEN {
            return None;
        }

        let encoded = (0..8u32)
            .map(|i| ((pen_id / 10u32.pow(i)) % 10) << (i * 4))
            .sum::<u32>();

        Some(Self(encoded))
    }

    /// Returns the PEN value encoded for bits 0-31 of the mRID.
    pub fn get(&self) -> u32 {
        self.0
    }
}

/// Given an IANA Private Enterprise Number (PEN), produce a unique mRID.
///
/// The supplied [`Pen`] is already encoded for bits 0-31. Choose
/// [`Pen::ieee2030_5`] to preserve base IEEE 2030.5 behavior.
#[cfg_attr(
    feature = "csip_aus",
    doc = " With the `csip_aus` feature enabled, choose [`Pen::csipaus`] for CSIP-AUS / TS 5573 client-generated mRIDs that require decimal PEN digits in the trailing 8 hex characters."
)]
pub fn mrid_gen(pen: Pen) -> MRIDType {
    let mut rng = rand::thread_rng();
    let random: u64 = rng.gen();
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards.")
        .as_secs() as u32;
    let count = MRID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let middle = now_secs ^ count;

    MRIDType(((random as u128) << 64) | ((middle as u128) << 32) | u128::from(pen.get()))
}

/// Given an IANA Private Enterprise Number (PEN) and a 96-bit resource id,
/// produce a deterministic mRID.
///
/// The supplied [`Pen`] occupies bits 0-31, and `id` occupies bits 32-127.
/// Two calls with the same `pen` and `id` always produce the same mRID,
/// allowing callers to recompute the mRID for an existing resource (e.g. to
/// avoid recreating it on the server).
///
/// Returns `None` if `id` does not fit in 96 bits.
pub fn mrid_with_id(pen: Pen, id: u128) -> Option<MRIDType> {
    if id >> 96 != 0 {
        return None;
    }
    Some(MRIDType((id << 32) | u128::from(pen.get())))
}

#[test]
fn mrid_contains_pen() {
    let pen = Pen::ieee2030_5(1337);
    let out = mrid_gen(pen).0;
    assert_eq!(pen.get(), out as u32);
}

#[test]
fn mrid_contains_ieee_pen_bits() {
    let pen = Pen::ieee2030_5(0xDEAD_BEEF);
    let out = mrid_gen(pen).0;
    assert_eq!(0xDEAD_BEEF, out as u32);
}

#[cfg(feature = "csip_aus")]
#[test]
fn mrid_contains_csipaus_pen_digits() {
    let pen = Pen::csipaus(54321).unwrap();
    assert_eq!(0x0005_4321, pen.get());

    let mrid = mrid_gen(pen);
    assert!(format!("{:032X}", mrid.0).ends_with("00054321"));
}

#[cfg(feature = "csip_aus")]
#[test]
fn csipaus_pen_bounds() {
    assert_eq!(Some(Pen::ieee2030_5(0x9999_9999)), Pen::csipaus(99_999_999));
    assert_eq!(None, Pen::csipaus(100_000_000));
    assert_eq!(None, Pen::csipaus(u32::MAX));
}

#[test]
fn mrid_with_id_is_deterministic() {
    let pen = Pen::ieee2030_5(1337);
    let id = 0x0000_0000_DEAD_BEEF_CAFE_BABE_1234_5678;
    assert_eq!(mrid_with_id(pen, id), mrid_with_id(pen, id));
}

#[test]
fn mrid_with_id_places_pen_and_id() {
    let pen = Pen::ieee2030_5(0xDEAD_BEEF);
    let id: u128 = 0x1234_5678_9ABC_DEF0_1122_3344;
    let mrid = mrid_with_id(pen, id).unwrap();
    assert_eq!(mrid.0 as u32, 0xDEAD_BEEF);
    assert_eq!(mrid.0 >> 32, id);
}

#[test]
fn mrid_with_id_rejects_oversized_id() {
    let pen = Pen::ieee2030_5(0);
    assert!(mrid_with_id(pen, (1u128 << 96) - 1).is_some());
    assert!(mrid_with_id(pen, 1u128 << 96).is_none());
    assert!(mrid_with_id(pen, u128::MAX).is_none());
}

#[test]
fn mrid_unique() {
    let num_ids = 1_000_000;
    let mut id_set = std::collections::HashSet::new();
    for _ in 0..num_ids {
        let mrid = mrid_gen(Pen::ieee2030_5(0)).0;
        assert!(!id_set.contains(&mrid), "Duplicate MRID generated");
        id_set.insert(mrid);
    }
}
