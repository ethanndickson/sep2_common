use core::fmt;
use std::{
    ops::{Deref, Index},
    str::FromStr,
};

use yaserde::{DefaultYaSerde, PrimitiveYaSerde};

#[cfg(test)]
use yaserde::{de::from_str, ser::to_string};

use crate::traits::Validate;

/// We purposefully don't use type aliases, as our procedural macros cannot determine whether a type is a primitive using an alias to it
/// This means types that are just primitive aliases cannot be used without these primitive newtypes.
/// We require newtypes for non-standard integer types regardless.

/// Unsigned integer, max inclusive 255 (2^8-1)
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Uint8(pub u8);

impl Deref for Uint8 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Uint8 {}
/// Unsigned integer, max inclusive 65535 (2^16-1)
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Uint16(pub u16);

impl Deref for Uint16 {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Uint16 {}
/// Unsigned integer, max inclusive 4294967295 (2^32-1)
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Uint32(pub u32);

impl Uint32 {
    pub fn get(&self) -> u32 {
        self.0
    }
}

impl Validate for Uint32 {}
/// Unsigned integer, max inclusive 1099511627775 (2^40-1)
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
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
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
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
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Uint64(pub u64);

impl Deref for Uint64 {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Uint64 {}
/// Signed integer, min -128 max +127
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Int8(pub i8);

impl Deref for Int8 {
    type Target = i8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Validate for Int8 {}
/// Signed integer, min -32768 max +32767
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
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
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
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
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
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
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, PrimitiveYaSerde)]
pub struct Int64(pub i64);

impl Int64 {
    pub fn get(&self) -> i64 {
        self.0
    }
}

impl Validate for Int64 {}

/// An 8-bit field encoded as a hex string (2 hex characters). Where applicable,
/// bit 0, or the least significant bit, goes on the right. Note that hexBinary
/// requires pairs of hex characters, so an odd number of characters requires a
/// leading "0".
#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, DefaultYaSerde)]
pub struct HexBinary8(pub u8);

impl Validate for HexBinary8 {}

impl fmt::Display for HexBinary8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#04x?}", self.0)
    }
}

impl FromStr for HexBinary8 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBinary8(
            u8::from_str_radix(&s[2..], 16).map_err(|_| "could not parse hexbinary8")?,
        ))
    }
}

/// A 16-bit field encoded as a hex string (4 hex characters max). Where
/// applicable, bit 0, or the least significant bit, goes on the right. Note that
/// hexBinary requires pairs of hex characters, so an odd number of characters
/// requires a leading "0".
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, DefaultYaSerde)]
pub struct HexBinary16(pub u16);

impl Validate for HexBinary16 {}

impl fmt::Display for HexBinary16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#06x?}", self.0)
    }
}

impl FromStr for HexBinary16 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBinary16(
            u16::from_str_radix(&s[2..], 16).map_err(|_| "could not parse hexbinary16")?,
        ))
    }
}

/// A 32-bit field encoded as a hex string (8 hex characters max). Where
/// applicable, bit 0, or the least significant bit, goes on the right. Note that
/// hexBinary requires pairs of hex characters, so an odd number of characters
/// requires a leading "0".
#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, DefaultYaSerde)]
pub struct HexBinary32(pub u32);

impl Validate for HexBinary32 {}

impl fmt::Display for HexBinary32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#010x?}", self.0)
    }
}

impl FromStr for HexBinary32 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBinary32(
            u32::from_str_radix(&s[2..], 16).map_err(|_| "could not parse hexbinary32")?,
        ))
    }
}

/// A 48-bit field encoded as a hex string (12 hex characters max). Where
/// applicable, bit 0, or the least significant bit, goes on the right. Note that
/// hexBinary requires pairs of hex characters, so an odd number of characters
/// requires a leading "0".
#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, DefaultYaSerde)]
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

impl fmt::Display for HexBinary48 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = &self.0;
        write!(f, "{:#014x?}", a)
    }
}

impl FromStr for HexBinary48 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBinary48(
            u64::from_str_radix(&s[2..], 16).map_err(|_| "could not parse hexbinary48")?,
        ))
    }
}

/// A 64-bit field encoded as a hex string (16 hex characters max). Where
/// applicable, bit 0, or the least significant bit, goes on the right. Note that
/// hexBinary requires pairs of hex characters, so an odd number of characters
/// requires a leading "0".
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, DefaultYaSerde)]
pub struct HexBinary64(pub u64);

impl Validate for HexBinary64 {}

impl fmt::Display for HexBinary64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = &self.0;
        write!(f, "{:#018x?}", a)
    }
}

impl FromStr for HexBinary64 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBinary64(
            u64::from_str_radix(&s[2..], 16).map_err(|_| "could not parse hexbinary48")?,
        ))
    }
}

/// A 128-bit field encoded as a hex string (32 hex characters max). Where
/// applicable, bit 0, or the least significant bit, goes on the right. Note that
/// hexBinary requires pairs of hex characters, so an odd number of characters
/// requires a leading "0".
#[derive(Default, Hash, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, DefaultYaSerde)]

pub struct HexBinary128(pub u128);

impl Validate for HexBinary128 {}

impl fmt::Display for HexBinary128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#034x?}", self.0)
    }
}

impl FromStr for HexBinary128 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBinary128(
            u128::from_str_radix(&s[2..], 16).map_err(|_| "could not parse hexbinary128")?,
        ))
    }
}

#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, DefaultYaSerde)]
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
        let a = self
            .0
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{a}")
    }
}

impl FromStr for HexBinary160 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            return Err("Hex string must be even".to_owned());
        }
        let mut out: [u8; 20] = [0; 20];
        let s = format!("{:0>40}", s);
        for i in (0..s.len()).step_by(2) {
            let hex_pair = &s[i..(i + 2)];
            if let Ok(byte) = u8::from_str_radix(hex_pair, 16) {
                out[i / 2] = byte;
            } else {
                return Err("Invalid Hex String".to_owned());
            }
        }

        Ok(HexBinary160(out))
    }
}

#[derive(Default, PartialEq, Eq, Debug, Clone, DefaultYaSerde)]
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
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, PrimitiveYaSerde)]
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
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, PrimitiveYaSerde)]
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
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, PrimitiveYaSerde)]
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
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, PrimitiveYaSerde)]
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
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, PrimitiveYaSerde)]
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
#[derive(Default, PartialEq, Eq, Debug, Clone, PrimitiveYaSerde)]
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

#[test]
fn default_hexbinary8() {
    let orig = HexBinary8::default();
    let new: HexBinary8 = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary16() {
    let orig = HexBinary16::default();
    let new: HexBinary16 = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary32() {
    let orig = HexBinary32::default();
    let new: HexBinary32 = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary48() {
    let orig = HexBinary48::default();
    let new: HexBinary48 = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary64() {
    let orig = HexBinary64::default();
    let new: HexBinary64 = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary128() {
    let orig = HexBinary128::default();
    let new: HexBinary128 = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_hexbinary160() {
    let orig = HexBinary160::default();
    let new: HexBinary160 = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_lfdi() {
    let orig = LFDI::default();
    let new: LFDI = from_str(&to_string(&orig).unwrap()).unwrap();
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
