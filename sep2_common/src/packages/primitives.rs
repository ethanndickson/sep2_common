use core::fmt;
use std::{
    ops::{Deref, Index},
    str::FromStr,
};

use sepserde::{DefaultYaSerde, PrimitiveYaSerde};

#[cfg(test)]
use crate::{deserialize, serialize};

use crate::traits::Validate;

/// We purposefully don't use type aliases, as our procedural macros cannot determine whether a type is a primitive using an alias to it
/// This means types that are just primitive aliases cannot be used without these primitive newtypes.
/// We require newtypes for non-standard integer types regardless.
/// Unsigned integer, max inclusive 255 (2^8-1)
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Uint8(pub u8);

impl Deref for Uint8 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Uint8 {}
/// Unsigned integer, max inclusive 65535 (2^16-1)
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Uint16(pub u16);

impl Deref for Uint16 {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Uint16 {}
/// Unsigned integer, max inclusive 4294967295 (2^32-1)
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Uint32(pub u32);

impl Uint32 {
    pub fn get(&self) -> u32 {
        self.0
    }
}

impl Validate for Uint32 {}
/// Unsigned integer, max inclusive 1099511627775 (2^40-1)
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Uint40(pub u64);

impl Deref for Uint40 {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Uint40 {
    fn validate(&self) -> Result<(), String> {
        if self.0 > "281474976710655".parse::<u64>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 281474976710655.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

/// Unsigned integer, max inclusive 281474976710655 (2^48-1)
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Uint48(pub u64);

impl Deref for Uint48 {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Uint48 {
    fn validate(&self) -> Result<(), String> {
        if self.0 > "281474976710655".parse::<u64>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 281474976710655.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

/// Unsigned integer, max inclusive 18446744073709551615 (2^64-1)
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Uint64(pub u64);

impl Deref for Uint64 {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Uint64 {}
/// Signed integer, min -128 max +127
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Int8(pub i8);

impl Deref for Int8 {
    type Target = i8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Int8 {}
/// Signed integer, min -32768 max +32767
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Int16(pub i16);

impl Deref for Int16 {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Int16 {}
/// Signed integer, max inclusive 2147483647 (2^31), min inclusive -2147483647
/// (same as xs:int)
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Int32(pub i32);

impl Deref for Int32 {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Int32 {}
/// Signed integer, max inclusive 140737488355328 (2^47), min inclusive
/// -140737488355328
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Int48(pub i64);

impl Deref for Int48 {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Int48 {
    fn validate(&self) -> Result<(), String> {
        if self.0 > "140737488355328".parse::<i64>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value! \nExpected: 0 <= 140737488355328.\nActual: 0 == {}", self.0));
        }
        if self.0 < "-140737488355328".parse::<i64>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value! \nExpected: 0 >= -140737488355328.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

/// Signed integer, max inclusive 9223372036854775807 (2^63), min inclusive
/// -9223372036854775808 (same as xs:long)
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Int64(pub i64);

impl Int64 {
    pub fn get(&self) -> i64 {
        self.0
    }
}

impl Validate for Int64 {}

/// Writes `value` as a hex string, compactly, with pairs of hex characters as
/// hexBinary requires, so an odd number of characters gets a leading "0".
pub(crate) fn write_hex_binary(f: &mut fmt::Formatter<'_>, value: u128) -> fmt::Result {
    let digits = format!("{value:X}");
    if !digits.len().is_multiple_of(2) {
        f.write_str("0")?;
    }
    f.write_str(&digits)
}

/// Reads a hex string of at most `max_octets` octets. Lowercase, leading
/// zeros, an odd digit count and a `0x` prefix are all tolerated, the last
/// because this crate emitted it through 0.2.0.
pub(crate) fn parse_hex_binary(s: &str, max_octets: usize) -> Result<u128, String> {
    let digits = hex_digits(s, max_octets)?;
    u128::from_str_radix(digits, 16).map_err(|_| too_wide(max_octets))
}

/// Reads a hex string into exactly `N` octets, left-padded with zero octets.
pub(crate) fn parse_hex_binary_bytes<const N: usize>(s: &str) -> Result<[u8; N], String> {
    // `hex_digits` bounds only the significant digits, so drop the leading
    // zeros before indexing; what remains is at most `N` octets wide.
    let digits = hex_digits(s, N)?.trim_start_matches('0');
    let mut out = [0u8; N];
    let mut octet = N;
    let mut end = digits.len();
    while end > 0 {
        let start = end.saturating_sub(2);
        octet -= 1;
        out[octet] = u8::from_str_radix(&digits[start..end], 16).map_err(|_| not_hex())?;
        end = start;
    }
    Ok(out)
}

fn too_wide(max_octets: usize) -> String {
    format!("hexBinary value does not fit in {max_octets} octets")
}

fn not_hex() -> String {
    "hexBinary value contains a non-hexadecimal character".to_owned()
}

/// Strips the legacy `0x` prefix and rejects anything that is not at most
/// `max_octets` octets of hexadecimal.
fn hex_digits(s: &str, max_octets: usize) -> Result<&str, String> {
    let digits = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    if digits.is_empty() {
        return Err("hexBinary value is empty".to_owned());
    }
    // `from_str_radix` would otherwise accept a leading `+`.
    if !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(not_hex());
    }
    if digits.trim_start_matches('0').len() > max_octets * 2 {
        return Err(too_wide(max_octets));
    }
    Ok(digits)
}

/// Generates the hexBinary `Display`/`FromStr` pair for a newtype over an
/// unsigned integer, where `$octets` is the XSD `maxLength` for the type.
macro_rules! hex_binary_serde {
    ($name:ident, $inner:ty, $octets:literal) => {
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write_hex_binary(f, u128::from(self.0))
            }
        }

        impl FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let value = parse_hex_binary(s, $octets)?;
                <$inner>::try_from(value)
                    .map($name)
                    .map_err(|_| too_wide($octets))
            }
        }
    };
}

/// An 8-bit field encoded as a hex string (2 hex characters). Where applicable,
/// bit 0, or the least significant bit, goes on the right. Note that hexBinary
/// requires pairs of hex characters, so an odd number of characters requires a
/// leading "0".
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, DefaultYaSerde)]
pub struct HexBinary8(pub u8);

impl Validate for HexBinary8 {}

hex_binary_serde!(HexBinary8, u8, 1);

/// A 16-bit field encoded as a hex string (4 hex characters max). Where
/// applicable, bit 0, or the least significant bit, goes on the right. Note that
/// hexBinary requires pairs of hex characters, so an odd number of characters
/// requires a leading "0".
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, DefaultYaSerde)]
pub struct HexBinary16(pub u16);

impl Validate for HexBinary16 {}

hex_binary_serde!(HexBinary16, u16, 2);

/// A 32-bit field encoded as a hex string (8 hex characters max). Where
/// applicable, bit 0, or the least significant bit, goes on the right. Note that
/// hexBinary requires pairs of hex characters, so an odd number of characters
/// requires a leading "0".
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, DefaultYaSerde)]
pub struct HexBinary32(pub u32);

impl Validate for HexBinary32 {}

hex_binary_serde!(HexBinary32, u32, 4);

/// A 48-bit field encoded as a hex string (12 hex characters max). Where
/// applicable, bit 0, or the least significant bit, goes on the right. Note that
/// hexBinary requires pairs of hex characters, so an odd number of characters
/// requires a leading "0".
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, DefaultYaSerde)]
pub struct HexBinary48(pub u64);

impl Validate for HexBinary48 {
    fn validate(&self) -> Result<(), String> {
        let a = &self.0;
        if a > &281474976710656 {
            Err(format!("Validation error: invalid value! \nExpected: 0 <= 281474976710656.\nActual: 0 == {}", a))
        } else {
            Ok(())
        }
    }
}

hex_binary_serde!(HexBinary48, u64, 6);

/// A 64-bit field encoded as a hex string (16 hex characters max). Where
/// applicable, bit 0, or the least significant bit, goes on the right. Note that
/// hexBinary requires pairs of hex characters, so an odd number of characters
/// requires a leading "0".
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, DefaultYaSerde)]
pub struct HexBinary64(pub u64);

impl Validate for HexBinary64 {}

hex_binary_serde!(HexBinary64, u64, 8);

/// A 128-bit field encoded as a hex string (32 hex characters max). Where
/// applicable, bit 0, or the least significant bit, goes on the right. Note that
/// hexBinary requires pairs of hex characters, so an odd number of characters
/// requires a leading "0".
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, DefaultYaSerde)]
pub struct HexBinary128(pub u128);

impl Validate for HexBinary128 {}

hex_binary_serde!(HexBinary128, u128, 16);

/// A 160-bit field encoded as a hex string (40 hex characters).
///
/// Unlike the types above this is an identifier rather than a number, so it
/// renders as all 40 digits even when the leading octets are zero.
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, DefaultYaSerde)]
pub struct HexBinary160(pub [u8; 20]); // TODO: Can this use a Cow?

impl Validate for HexBinary160 {}

impl Index<usize> for HexBinary160 {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl AsRef<[u8]> for HexBinary160 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for HexBinary160 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02X}")?;
        }
        Ok(())
    }
}

impl FromStr for HexBinary160 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_hex_binary_bytes(s).map(HexBinary160)
    }
}

#[derive(Default, Hash, PartialEq, Eq, Debug, Clone, DefaultYaSerde)]
pub struct LFDI(pub HexBinary160);

impl Validate for LFDI {}

impl fmt::Display for LFDI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hexstring = format!("{}", self.0);
        write!(
            f,
            "{}",
            hexstring
                .chars()
                .enumerate()
                .flat_map(|(i, c)| {
                    if i > 0 && i % 4 == 0 { Some('-') } else { None }
                        .into_iter()
                        .chain(Some(c))
                })
                .collect::<String>()
        )
    }
}

impl FromStr for LFDI {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('-', "");
        Ok(LFDI(HexBinary160::from_str(&s)?))
    }
}

/// Character string of max length 6. In order to limit internal storage,
/// implementations SHALL reduce the length of strings using multi-byte
/// characters so that the string may be stored using "maxLength" octets in the
/// given encoding.
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, PrimitiveYaSerde)]
pub struct String6(pub String);

impl Validate for String6 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 6 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 6 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

/// Character string of max length 16. In order to limit internal storage,
/// implementations SHALL reduce the length of strings using multi-byte
/// characters so that the string may be stored using "maxLength" octets in the
/// given encoding.
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, PrimitiveYaSerde)]
pub struct String16(pub String);

impl Validate for String16 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 16 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 16 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

/// Character string of max length 20. In order to limit internal storage,
/// implementations SHALL reduce the length of strings using multi-byte
/// characters so that the string may be stored using "maxLength" octets in the
/// given encoding.
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, PrimitiveYaSerde)]
pub struct String20(pub String);

impl Validate for String20 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 20 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 20 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

/// Character string of max length 32. In order to limit internal storage,
/// implementations SHALL reduce the length of strings using multi-byte
/// characters so that the string may be stored using "maxLength" octets in the
/// given encoding.
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, PrimitiveYaSerde)]
pub struct String32(pub String);

impl Validate for String32 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 32 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 32 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

/// Character string of max length 42. In order to limit internal storage,
/// implementations SHALL reduce the length of strings using multi-byte
/// characters so that the string may be stored using "maxLength" octets in the
/// given encoding.
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, PrimitiveYaSerde)]
pub struct String42(pub String);

impl Validate for String42 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 42 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 42 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

/// Character string of max length 192. For all string types, in order to limit
/// internal storage, implementations SHALL reduce the length of strings using
/// multi-byte characters so that the string may be stored using "maxLength"
/// octets in the given encoding.
#[derive(Default, Hash, PartialEq, Eq, Debug, Clone, PrimitiveYaSerde)]
pub struct String192(pub String);

impl Validate for String192 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 192 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 192 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

/// Asserts `rendered` is in the lexical space of `xs:hexBinary`,
/// `([0-9a-fA-F]{2})*`, and uppercase as the XSD canonical form.
#[cfg(test)]
pub(crate) fn assert_conformant(rendered: &str) {
    assert!(
        !rendered.is_empty(),
        "hexBinary value must not be empty, got {rendered:?}"
    );
    assert!(
        rendered.len().is_multiple_of(2),
        "hexBinary values SHALL have an even number of digits, got {rendered:?}"
    );
    assert!(
        rendered
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_lowercase()),
        "hexBinary values are uppercase hex digits with no prefix, got {rendered:?}"
    );
}

/// Generates the hexBinary lexical-space tests for a type.
macro_rules! hex_binary_tests {
    ($shape:ident, $name:ident, $inner:ty, $octets:literal) => {
        #[test]
        fn $shape() {
            let mut values = vec![0 as $inner];
            for shift in 0..($octets * 8) {
                // A lone set bit, and every bit below it set, at each position.
                let bit = (1 as $inner) << shift;
                values.push(bit);
                values.push(bit | (bit - 1));
            }

            for value in values {
                let orig = $name(value);
                let rendered = orig.to_string();
                assert_conformant(&rendered);
                assert!(
                    rendered.len() <= $octets * 2,
                    "{} rendered wider than {} octets: {rendered:?}",
                    stringify!($name),
                    $octets
                );
                assert_eq!(Ok(orig), $name::from_str(&rendered));
                // Peers may pad to the type's full width.
                let padded = format!("{:0>width$}", rendered, width = $octets * 2);
                assert_eq!(Ok(orig), $name::from_str(&padded));
                let round_trip: $name = deserialize(&serialize(&orig).unwrap()).unwrap();
                assert_eq!(orig, round_trip);
            }
        }
    };
}

hex_binary_tests!(hexbinary8_conformance, HexBinary8, u8, 1);
hex_binary_tests!(hexbinary16_conformance, HexBinary16, u16, 2);
hex_binary_tests!(hexbinary32_conformance, HexBinary32, u32, 4);
hex_binary_tests!(hexbinary48_conformance, HexBinary48, u64, 6);
hex_binary_tests!(hexbinary64_conformance, HexBinary64, u64, 8);
hex_binary_tests!(hexbinary128_conformance, HexBinary128, u128, 16);

#[test]
fn hexbinary_is_minimal_and_unprefixed() {
    // Regression: these rendered via `{:#0Nx?}`, emitting a lowercase,
    // `0x`-prefixed string that is not hexBinary at all.
    assert_eq!("3F", HexBinary8(0x3F).to_string());
    assert_eq!("00", HexBinary16(0).to_string());
    assert_eq!("3F", HexBinary16(0x3F).to_string());
    assert_eq!("0100", HexBinary16(0x100).to_string());
    assert_eq!("0F4240", HexBinary32(1_000_000).to_string());
    assert_eq!("0ED30F5A0000", HexBinary48(0x0ED3_0F5A_0000).to_string());
    assert_eq!("00", HexBinary128(0).to_string());
}

#[test]
fn hexbinary_accepts_conformant_input() {
    // Regression: `FromStr` sliced `&s[2..]` to strip the `0x` its own
    // `Display` emitted, rejecting `3F` and panicking on `0`.
    assert_eq!(Ok(HexBinary8(0x3F)), HexBinary8::from_str("3F"));
    assert_eq!(Ok(HexBinary8(0x3F)), HexBinary8::from_str("3f"));
    assert_eq!(Ok(HexBinary8(0)), HexBinary8::from_str("0"));
    assert_eq!(Ok(HexBinary128(1)), HexBinary128::from_str("01"));
    assert_eq!(
        Ok(HexBinary48(0x0ED3_0F5A_0000)),
        HexBinary48::from_str("0ED30F5A0000")
    );
}

#[test]
fn hexbinary_accepts_legacy_prefixed_input() {
    // sep2_common <= 0.2.0 emitted a `0x` prefix.
    assert_eq!(Ok(HexBinary8(0x3F)), HexBinary8::from_str("0x3f"));
    assert_eq!(
        Ok(HexBinary64(0)),
        HexBinary64::from_str("0x0000000000000000")
    );
    assert_eq!(
        Ok(HexBinary64(0x0DEA_DBEE)),
        HexBinary64::from_str("DEADBEE")
    );
}

#[test]
fn hexbinary_rejects_uninterpretable_input() {
    assert!(HexBinary8::from_str("").is_err());
    assert!(HexBinary8::from_str("GG").is_err());
    // `from_str_radix` accepts a leading sign; hexBinary does not.
    assert!(HexBinary8::from_str("+1").is_err());
    assert!(HexBinary8::from_str("100").is_err());
    assert!(HexBinary16::from_str("10000").is_err());
    assert_eq!(Ok(HexBinary8(1)), HexBinary8::from_str("0001"));
}

#[test]
fn hexbinary160_is_fixed_width() {
    assert_eq!(
        "0000000000000000000000000000000000000000",
        HexBinary160::default().to_string()
    );
    assert_eq!(
        "00000000000000000000000000000000C0FFEE00",
        HexBinary160::from_str("C0FFEE00").unwrap().to_string()
    );
}

#[test]
fn hexbinary160_rejects_oversized_input() {
    // Regression: `from_str` left-padded to 40 characters without checking the
    // input was not already longer, then indexed past the end of its `[u8; 20]`.
    assert!(HexBinary160::from_str(&"A".repeat(41)).is_err());
    assert!(HexBinary160::from_str(&"F".repeat(40)).is_ok());
    assert_eq!(
        Ok(HexBinary160::default()),
        HexBinary160::from_str(&"0".repeat(64))
    );
}

#[test]
fn default_hexbinary8() {
    let orig = HexBinary8::default();
    let new: HexBinary8 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary16() {
    let orig = HexBinary16::default();
    let new: HexBinary16 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary32() {
    let orig = HexBinary32::default();
    let new: HexBinary32 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary48() {
    let orig = HexBinary48::default();
    let new: HexBinary48 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary64() {
    let orig = HexBinary64::default();
    let new: HexBinary64 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary128() {
    let orig = HexBinary128::default();
    let new: HexBinary128 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary160() {
    let orig = HexBinary160::default();
    let new: HexBinary160 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_lfdi() {
    let orig = LFDI::default();
    let new: LFDI = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn example_hexbinary160() {
    let orig: HexBinary160 = HexBinary160::from_str("C0FFEE00").unwrap();
    let new = orig.to_string();
    assert_eq!(orig, HexBinary160::from_str(&new).unwrap());
}

#[test]
fn example_lfdi() {
    let orig: LFDI = LFDI::from_str("C0FFEE00").unwrap();
    let new = orig.to_string();
    assert_eq!(orig, LFDI::from_str(&new).unwrap());
}

#[test]
fn default_uint8() {
    let orig = Uint8::default();
    let new: Uint8 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_uint16() {
    let orig = Uint16::default();
    let new: Uint16 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_uint32() {
    let orig = Uint32::default();
    let new: Uint32 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_uint40() {
    let orig = Uint40::default();
    let new: Uint40 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_uint48() {
    let orig = Uint48::default();
    let new: Uint48 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_uint64() {
    let orig = Uint64::default();
    let new: Uint64 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_int8() {
    let orig = Int8::default();
    let new: Int8 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_int16() {
    let orig = Int16::default();
    let new: Int16 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_int32() {
    let orig = Int32::default();
    let new: Int32 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_int48() {
    let orig = Int48::default();
    let new: Int48 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_int64() {
    let orig = Int64::default();
    let new: Int64 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_string6() {
    let orig = String6::default();
    let new: String6 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_string16() {
    let orig = String16::default();
    let new: String16 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_string20() {
    let orig = String20::default();
    let new: String20 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_string32() {
    let orig = String32::default();
    let new: String32 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_string42() {
    let orig = String42::default();
    let new: String42 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_string192() {
    let orig = String192::default();
    let new: String192 = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}
