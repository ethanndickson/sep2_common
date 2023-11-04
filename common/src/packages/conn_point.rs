//! ConnectionPoint Package
//!
//! Extension forming part of the Australian Common Smart Inverter Profile
//!
//! NOT used outside of Australian CSIP

use crate::traits::{SEResource, Validate};
use sep2_common_derive::SEResource;

use yaserde::{YaDeserialize, YaSerialize};

use super::{identification::Link, primitives::String32};

/// The intent of the Connection Point registration extension is to provide
/// the capability for SEP2 servers to offer a consistent mechanism to provide
/// additional information about the existence and location of a newly registered
/// device on the network.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "ConnectionPoint")]
#[yaserde(prefix = "csipaus", namespace = "csipaus: https://csipaus.org/ns")]
pub struct ConnectionPoint {
    /// The ConnectionPoint identifier.
    /// By default, the National Metering identifier (NMI) is used to identify
    /// the network location of the device in this attribute.
    #[yaserde(rename = "connectionPointId")]
    #[yaserde(prefix = "csipaus", namespace = "csipaus: https://csipaus.org/ns")]
    pub connection_pointid: String32,
    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

/// SHALL contain a Link to an instance of ConnectionPoint
pub type ConnectionPointLink = Link;

impl Validate for ConnectionPoint {}

#[cfg(test)]
use {
    super::edev::EndDevice,
    crate::{deserialize, serialize},
};

#[test]
fn default_connectionpoint() {
    let orig = ConnectionPoint::default();
    let new: ConnectionPoint = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn example_connectionpoint() {
    let orig = r#"<csipaus:ConnectionPoint xmlns:csipaus="https://csipaus.org/ns">
  <csipaus:connectionPointId>1234567890</csipaus:connectionPointId>
</csipaus:ConnectionPoint>"#;
    let new_de: ConnectionPoint = deserialize(orig).unwrap();
    let new_se = serialize(&new_de).unwrap();
    assert_eq!(orig, new_se);
}

#[test]
fn connectionpoint_edev() {
    let expected = r#"<EndDevice xmlns="urn:ieee:std:2030.5:ns">
  <changedTime>0</changedTime>
  <csipaus:ConnectionPointLink  href="/edev/1/cp" />
  <sFDI>0</sFDI>
</EndDevice>"#;
    let mut edev = EndDevice::default();
    edev.connection_point_link = Some(ConnectionPointLink {
        href: "/edev/1/cp".to_owned(),
    });
    assert_eq!(expected, serialize(&edev).unwrap())
}
