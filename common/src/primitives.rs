/*
 * Described in detail in section B.2.3.5.
 * Specifically, it contains  type for = aliases
 * primitive, concrete data types
 *  (e.g. Int64 is the type i64 in rust)
 * All types can be serialised and deserialised using the serde crate
 */
use arrayvec::ArrayString;
use core::fmt;
use num_bigint::BigUint;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::io::ErrorKind;

pub type Int8 = i8;
pub type UInt8 = u8;
pub type Int16 = i16;
pub type UInt16 = u16;
pub type Int32 = i32;
pub type UInt32 = u32;
pub type Int48 = i64;
pub type Int64 = i64;
pub type UInt48 = u64;
pub type UInt64 = u64;
pub type UInt40 = UInt64;
pub type UInt128 = u128;

// HexBinary8
// Spec says:
/*
 * Where applicable, bit 0, or the least significant bit, goes on the right.
 * Note that hexBinary requires pairs of hex characters, so an odd number of characters
 * requires a leading “0”.
 */
// While this would make printing the characters prettier, implementing this functionality for
// is neither required nor particuarly important to the functioning of the protocol, so this
// implementation simply prints the full length of the HexBinary in hexacimal, padded with '0'
// Implementing the neater version (eg. 0x123 -> 0x0123) is left as a future optimisation
#[derive(
    Serialize, Deserialize, Default, Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord,
)]
pub struct HexBinary8(UInt8);

impl AsRef<UInt8> for HexBinary8 {
    fn as_ref(&self) -> &UInt8 {
        &self.0
    }
}

impl fmt::Display for HexBinary8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary8(a) = self;
        write!(f, "{:#04x}", a)
    }
}

#[derive(
    Serialize, Deserialize, Default, Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord,
)]
pub struct HexBinary16(UInt16);

impl AsRef<UInt16> for HexBinary16 {
    fn as_ref(&self) -> &UInt16 {
        &self.0
    }
}

impl fmt::Display for HexBinary16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary16(a) = self;
        write!(f, "{:#06x}", a)
    }
}

#[derive(
    Serialize, Deserialize, Default, Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord,
)]
pub struct HexBinary32(UInt32);

impl AsRef<UInt32> for HexBinary32 {
    fn as_ref(&self) -> &UInt32 {
        &self.0
    }
}

impl fmt::Display for HexBinary32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary32(a) = self;
        write!(f, "{:#010x}", a)
    }
}

#[derive(
    Serialize, Deserialize, Default, Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord,
)]
pub struct HexBinary48(UInt64);

impl AsRef<UInt64> for HexBinary48 {
    fn as_ref(&self) -> &UInt64 {
        &self.0
    }
}

impl fmt::Display for HexBinary48 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary48(a) = self;
        write!(f, "{:#014x}", a)
    }
}

#[derive(
    Serialize, Deserialize, Default, Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord,
)]
pub struct HexBinary64(UInt64);

impl AsRef<UInt64> for HexBinary64 {
    fn as_ref(&self) -> &UInt64 {
        &self.0
    }
}

impl fmt::Display for HexBinary64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary64(a) = self;
        write!(f, "{:#018x}", a)
    }
}

#[derive(
    Serialize, Deserialize, Default, Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord,
)]
pub struct HexBinary128(UInt128);

impl AsRef<UInt128> for HexBinary128 {
    fn as_ref(&self) -> &UInt128 {
        &self.0
    }
}

impl fmt::Display for HexBinary128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary128(a) = self;
        write!(f, "{:#034x}", a)
    }
}

#[derive(Default, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct HexBinary160(BigUint);

impl HexBinary160 {
    fn new(num: Vec<UInt8>) -> Result<HexBinary160, ErrorKind> {
        // Error checking
        if num.len() > 20 {
            Err(ErrorKind::InvalidInput)
        } else {
            Ok(HexBinary160(BigUint::from_bytes_be(&num)))
        }
    }
}

impl fmt::Display for HexBinary160 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary160(a) = self;
        write!(f, "{:#042x}", a)
    }
}

// spec says:
/*
 * In order to limit internal storage, implementations SHALL reduce
 * the length of strings using multi-byte characters so that the
 * string may be stored using “maxLength” octets in the given encoding.
 */
// we are assuming maxLength is the numeric suffix to `String` and octet means 8 bits,
// so this implies that the number suffix is the number of elements, not bytes, in the string.
// For the sake of simplicity, we are ignoring the fact that Strings allow for any valid
// Unicode character, including those greater than 8 bits. The space savings on single byte
// characters and rarity of 4 byte characters should make up for it.
// ArrayStrings are used to allow String methods, and enforce a max size.

// Optional Optimisation actually implmenting this requirement of using multi-byte characters is left as a future
// optimisation

#[derive(Clone, Debug, Default)]
pub struct String6(ArrayString<6>);
impl String6 {
    const MAX_LEN: usize = 6;
    // would like to use generic errors here, but that will be done during the
    // Great Error Refactor (GER)
    fn new(input: &str) -> Result<Self, ErrorKind> {
        if input.len() > Self::MAX_LEN {
            Err(ErrorKind::InvalidInput)
        } else {
            Ok(String6(ArrayString::from(input).unwrap()))
        }
    }
}

struct String6Visitor;

impl Serialize for String6 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0.as_str())
    }
}

// can't make this generic as Value must be concrete.
impl<'de> Visitor<'de> for String6Visitor {
    type Value = String6;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected string with maximum length of 6 characters")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String6(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(&v).unwrap(),
            ))
        }
    }

    // This is the function Deserialize runs. All other functions are not required/flavour
    // visit_string can be written triviall by using exactly the same body as bellow,
    // but borrowing "v" and using the appropriate function signature.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String6(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(v).unwrap(),
            ))
        }
    }
}

// In keeping with KISS, trying other tomfoolery to work around the innevitable
// boilerplate is not worth the trouble at this point.
// We will need to get used to macro's and use them to derive all this stuff.
impl<'de> Deserialize<'de> for String6 {
    fn deserialize<D>(deserializer: D) -> Result<String6, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(String6Visitor)
    }
}

#[derive(Clone, Debug, Default)]
pub struct String16(ArrayString<16>);
impl String16 {
    const MAX_LEN: usize = 16;

    fn new(input: &str) -> Result<Self, ErrorKind> {
        if input.len() > Self::MAX_LEN {
            Err(ErrorKind::InvalidInput)
        } else {
            Ok(String16(ArrayString::from(input).unwrap()))
        }
    }
}

struct String16Visitor;

impl Serialize for String16 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0.as_str())
    }
}

// can't make this generic as Value must be concrete.
impl<'de> Visitor<'de> for String16Visitor {
    type Value = String16;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected string with maximum length of 6 characters")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String16(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(&v).unwrap(),
            ))
        }
    }

    // This is the function Deserialize runs. All other functions are not required/flavour
    // visit_string can be written triviall by using exactly the same body as bellow,
    // but borrowing "v" and using the appropriate function signature.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String16(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(v).unwrap(),
            ))
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct String20(ArrayString<20>);
impl String20 {
    const MAX_LEN: usize = 20;

    fn new(input: &str) -> Result<Self, ErrorKind> {
        if input.len() > Self::MAX_LEN {
            Err(ErrorKind::InvalidInput)
        } else {
            Ok(String20(ArrayString::from(input).unwrap()))
        }
    }
}

struct String20Visitor;

impl Serialize for String20 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0.as_str())
    }
}

// can't make this generic as Value must be concrete.
impl<'de> Visitor<'de> for String20Visitor {
    type Value = String20;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected string with maximum length of 6 characters")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String20(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(&v).unwrap(),
            ))
        }
    }

    // This is the function Deserialize runs. All other functions are not required/flavour
    // visit_string can be written triviall by using exactly the same body as bellow,
    // but borrowing "v" and using the appropriate function signature.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String20(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(v).unwrap(),
            ))
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct String32(ArrayString<32>);
impl String32 {
    const MAX_LEN: usize = 32;

    fn new(input: &str) -> Result<Self, ErrorKind> {
        if input.len() > Self::MAX_LEN {
            Err(ErrorKind::InvalidInput)
        } else {
            Ok(String32(ArrayString::from(input).unwrap()))
        }
    }
}

struct String32Visitor;

impl Serialize for String32 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0.as_str())
    }
}

// can't make this generic as Value must be concrete.
impl<'de> Visitor<'de> for String32Visitor {
    type Value = String32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected string with maximum length of 6 characters")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String32(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(&v).unwrap(),
            ))
        }
    }

    // This is the function Deserialize runs. All other functions are not required/flavour
    // visit_string can be written triviall by using exactly the same body as bellow,
    // but borrowing "v" and using the appropriate function signature.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String32(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(v).unwrap(),
            ))
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct String42(ArrayString<42>);
impl String42 {
    const MAX_LEN: usize = 42;

    fn new(input: &str) -> Result<Self, ErrorKind> {
        if input.len() > Self::MAX_LEN {
            Err(ErrorKind::InvalidInput)
        } else {
            Ok(String42(ArrayString::from(input).unwrap()))
        }
    }
}

struct String42Visitor;

impl Serialize for String42 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0.as_str())
    }
}

// can't make this generic as Value must be concrete.
impl<'de> Visitor<'de> for String42Visitor {
    type Value = String42;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected string with maximum length of 6 characters")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String42(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(&v).unwrap(),
            ))
        }
    }

    // This is the function Deserialize runs. All other functions are not required/flavour
    // visit_string can be written triviall by using exactly the same body as bellow,
    // but borrowing "v" and using the appropriate function signature.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String42(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(v).unwrap(),
            ))
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct String192(ArrayString<192>);
impl String192 {
    const MAX_LEN: usize = 192;

    fn new(input: &str) -> Result<Self, ErrorKind> {
        if input.len() > Self::MAX_LEN {
            Err(ErrorKind::InvalidInput)
        } else {
            Ok(String192(ArrayString::from(input).unwrap()))
        }
    }
}

struct String192Visitor;

impl Serialize for String192 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0.as_str())
    }
}

// can't make this generic as Value must be concrete.
impl<'de> Visitor<'de> for String192Visitor {
    type Value = String192;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected string with maximum length of 6 characters")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String192(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(&v).unwrap(),
            ))
        }
    }

    // This is the function Deserialize runs. All other functions are not required/flavour
    // visit_string can be written triviall by using exactly the same body as bellow,
    // but borrowing "v" and using the appropriate function signature.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() > Self::Value::MAX_LEN {
            Err(E::invalid_length(Self::Value::MAX_LEN, &self))
        } else {
            Ok(String192(
                ArrayString::<{ Self::Value::MAX_LEN }>::from(v).unwrap(),
            ))
        }
    }
}
