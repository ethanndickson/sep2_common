use core::fmt;
use num_bigint::BigUint;
use std::str::FromStr;

use xsd_macro_utils::{UtilsDefaultSerde, UtilsTupleIo};
use xsd_parser::generator::validator::Validate;

// Can we switch to type aliases?
// pub type Int8 = i8;
// pub type Uint8 = u8;
// pub type Int16 = i16;
// pub type Uint16 = u16;
// pub type Int32 = i32;
// pub type Uint32 = u32;
// pub type Int48 = i64;
// pub type Int64 = i64;
// pub type Uint48 = u64;
// pub type Uint64 = u64;
// pub type Uint40 = Uint64;
// pub type Uint128 = u128;

// Unsigned integer, max inclusive 255 (2^8-1)
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint8(pub u8);

impl Validate for Uint8 {}
// Unsigned integer, max inclusive 65535 (2^16-1)
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint16(pub u16);

impl Validate for Uint16 {}
// Unsigned integer, max inclusive 4294967295 (2^32-1)
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint32(pub u32);

impl Validate for Uint32 {}
// Unsigned integer, max inclusive 1099511627775 (2^40-1)
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint40(pub u64);

impl Validate for Uint40 {
    fn validate(&self) -> Result<(), String> {
        if self.0 > "281474976710655".parse::<u64>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 281474976710655.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

// Unsigned integer, max inclusive 281474976710655 (2^48-1)
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint48(pub u64);

impl Validate for Uint48 {
    fn validate(&self) -> Result<(), String> {
        if self.0 > "281474976710655".parse::<u64>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 281474976710655.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

// Unsigned integer, max inclusive 18446744073709551615 (2^64-1)
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint64(pub u64);

impl Validate for Uint64 {}
// Signed integer, min -128 max +127
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Int8(pub i8);

impl Validate for Int8 {}
// Signed integer, min -32768 max +32767
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Int16(pub i16);

impl Validate for Int16 {}
// Signed integer, max inclusive 2147483647 (2^31), min inclusive -2147483647
// (same as xs:int)
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Int32(pub i32);

impl Validate for Int32 {}
// Signed integer, max inclusive 140737488355328 (2^47), min inclusive
// -140737488355328
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Int48(pub i64);

impl Validate for Int48 {
    fn validate(&self) -> Result<(), String> {
        if self.0 > "140737488355328".parse::<i64>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 140737488355328.\nActual: 0 == {}", self.0));
        }
        if self.0 < "-140737488355328".parse::<i64>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= -140737488355328.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

// Signed integer, max inclusive 9223372036854775807 (2^63), min inclusive
// -9223372036854775808 (same as xs:long)
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Int64(pub i64);

impl Validate for Int64 {}

// An 8-bit field encoded as a hex string (2 hex characters). Where applicable,
// bit 0, or the least significant bit, goes on the right. Note that hexBinary
// requires pairs of hex characters, so an odd number of characters requires a
// leading "0".
#[derive(Default, PartialEq, Debug, UtilsDefaultSerde)]
pub struct HexBinary8(pub Uint8);

impl Validate for HexBinary8 {}

impl fmt::Display for HexBinary8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary8(a) = self;
        let Uint8(a) = a;
        write!(f, "{:#04x?}", a)
    }
}

impl FromStr for HexBinary8 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBinary8(Uint8(
            u8::from_str_radix(&s[2..], 16).map_err(|_| "could not parse hexbinary8")?,
        )))
    }
}

// A 16-bit field encoded as a hex string (4 hex characters max). Where
// applicable, bit 0, or the least significant bit, goes on the right. Note that
// hexBinary requires pairs of hex characters, so an odd number of characters
// requires a leading "0".
#[derive(Default, PartialEq, Debug, UtilsDefaultSerde)]
pub struct HexBinary16(pub Uint16);

impl Validate for HexBinary16 {}

impl fmt::Display for HexBinary16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary16(a) = self;
        let Uint16(a) = a;
        write!(f, "{:#06x?}", a)
    }
}

impl FromStr for HexBinary16 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBinary16(Uint16(
            u16::from_str_radix(&s[2..], 16).map_err(|_| "could not parse hexbinary16")?,
        )))
    }
}

// A 32-bit field encoded as a hex string (8 hex characters max). Where
// applicable, bit 0, or the least significant bit, goes on the right. Note that
// hexBinary requires pairs of hex characters, so an odd number of characters
// requires a leading "0".
#[derive(Default, PartialEq, Debug, UtilsDefaultSerde)]
pub struct HexBinary32(pub Uint32);

impl Validate for HexBinary32 {}

impl fmt::Display for HexBinary32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary32(a) = self;
        let Uint32(a) = a;
        write!(f, "{:#010x?}", a)
    }
}

impl FromStr for HexBinary32 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBinary32(Uint32(
            u32::from_str_radix(&s[2..], 16).map_err(|_| "could not parse hexbinary32")?,
        )))
    }
}

// A 48-bit field encoded as a hex string (12 hex characters max). Where
// applicable, bit 0, or the least significant bit, goes on the right. Note that
// hexBinary requires pairs of hex characters, so an odd number of characters
// requires a leading "0".
#[derive(Default, PartialEq, Debug, UtilsDefaultSerde)]
pub struct HexBinary48(pub Uint64);

impl Validate for HexBinary48 {
    fn validate(&self) -> Result<(), String> {
        // TODO: Validate value is 48 bits
        todo!()
    }
}

impl fmt::Display for HexBinary48 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary48(a) = self;
        let Uint64(a) = a;
        write!(f, "{:#014x?}", a)
    }
}

impl FromStr for HexBinary48 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBinary48(Uint64(
            u64::from_str_radix(&s[2..], 16).map_err(|_| "could not parse hexbinary48")?,
        )))
    }
}

// A 64-bit field encoded as a hex string (16 hex characters max). Where
// applicable, bit 0, or the least significant bit, goes on the right. Note that
// hexBinary requires pairs of hex characters, so an odd number of characters
// requires a leading "0".
#[derive(Default, PartialEq, Debug, UtilsDefaultSerde)]
pub struct HexBinary64(pub Uint64);

impl Validate for HexBinary64 {}

impl fmt::Display for HexBinary64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary64(a) = self;
        let Uint64(a) = a;
        write!(f, "{:#018x?}", a)
    }
}

impl FromStr for HexBinary64 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HexBinary64(Uint64(
            u64::from_str_radix(&s[2..], 16).map_err(|_| "could not parse hexbinary48")?,
        )))
    }
}

// A 128-bit field encoded as a hex string (32 hex characters max). Where
// applicable, bit 0, or the least significant bit, goes on the right. Note that
// hexBinary requires pairs of hex characters, so an odd number of characters
// requires a leading "0".
#[derive(Default, PartialEq, Debug, UtilsDefaultSerde)]

pub struct HexBinary128(pub u128);

impl Validate for HexBinary128 {}

impl fmt::Display for HexBinary128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary128(a) = self;
        write!(f, "{:#034x?}", a)
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

#[derive(Default, PartialEq, Debug, UtilsDefaultSerde)]
pub struct HexBinary160(BigUint);

impl Validate for HexBinary160 {}

impl fmt::Display for HexBinary160 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary160(a) = self;
        write!(f, "{:#042x?}", a)
    }
}

impl FromStr for HexBinary160 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

// Character string of max length 6. In order to limit internal storage,
// implementations SHALL reduce the length of strings using multi-byte
// characters so that the string may be stored using "maxLength" octets in the
// given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
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

// Character string of max length 16. In order to limit internal storage,
// implementations SHALL reduce the length of strings using multi-byte
// characters so that the string may be stored using "maxLength" octets in the
// given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
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

// Character string of max length 20. In order to limit internal storage,
// implementations SHALL reduce the length of strings using multi-byte
// characters so that the string may be stored using "maxLength" octets in the
// given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
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

// Character string of max length 32. In order to limit internal storage,
// implementations SHALL reduce the length of strings using multi-byte
// characters so that the string may be stored using "maxLength" octets in the
// given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
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

// Character string of max length 42. In order to limit internal storage,
// implementations SHALL reduce the length of strings using multi-byte
// characters so that the string may be stored using "maxLength" octets in the
// given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
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

// Character string of max length 192. For all string types, in order to limit
// internal storage, implementations SHALL reduce the length of strings using
// multi-byte characters so that the string may be stored using "maxLength"
// octets in the given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
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