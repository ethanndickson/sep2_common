#![allow(non_snake_case)]
use std::str::FromStr;

use common::config::YASERDE_CFG;
use common::packages::primitives::*;
use yaserde::de::from_str;
use yaserde::ser::to_string_with_config;

#[test]
fn default_HexBinary8() {
    let orig = HexBinary8::default();
    let new: HexBinary8 = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_HexBinary16() {
    let orig = HexBinary16::default();
    let new: HexBinary16 = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_HexBinary32() {
    let orig = HexBinary32::default();
    let new: HexBinary32 = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_HexBinary48() {
    let orig = HexBinary48::default();
    let new: HexBinary48 = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_HexBinary64() {
    let orig = HexBinary64::default();
    let new: HexBinary64 = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_HexBinary128() {
    let orig = HexBinary128::default();
    let new: HexBinary128 = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_HexBinary160() {
    let orig = HexBinary160::default();
    let new: HexBinary160 = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LFDI() {
    let orig = LFDI::default();
    let new: LFDI = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    println!("{new}");
    assert_eq!(orig, new);
}

#[test]
fn example_HexBinary160() {
    let orig: HexBinary160 = HexBinary160::from_str("C0FFEE00").unwrap();
    let new = orig.to_string();
    assert_eq!(orig, HexBinary160::from_str(&new).unwrap());
}

#[test]
fn example_LFDI() {
    let orig: LFDI = LFDI::from_str("C0FFEE00").unwrap();
    let new = orig.to_string();
    assert_eq!(orig, LFDI::from_str(&new).unwrap());
}
