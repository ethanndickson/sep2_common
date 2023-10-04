use crate::traits::{SEList, SEResource, Validate};
use sep2_common_derive::{SEList, SEResource};

use super::{
    links::{
        IPAddrListLink, LlinterfaceListLink, NeighborListLink, RplinstanceListLink,
        RplsourceRoutesListLink,
    },
    primitives::{
        HexBinary128, HexBinary64, HexBinary8, Int64, String16, String192, Uint16, Uint32, Uint8,
    },
};
use yaserde::{YaDeserialize, YaSerialize};

/// Contains 802.15.4 link layer specific attributes.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "IEEE_802_15_4")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Ieee802154 {
    /// As defined by IEEE 802.15.4
    #[yaserde(rename = "capabilityInfo")]
    pub capability_info: HexBinary8,

    #[yaserde(rename = "NeighborListLink")]
    pub neighbor_list_link: Option<NeighborListLink>,

    /// As defined by IEEE 802.15.4
    #[yaserde(rename = "shortAddress")]
    pub short_address: Uint16,
}

impl Validate for Ieee802154 {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "IPAddr")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IPAddr {
    /// An IP address value.
    #[yaserde(rename = "address")]
    pub address: HexBinary128,

    #[yaserde(rename = "RPLInstanceListLink")]
    pub rpl_instance_list_link: Option<RplinstanceListLink>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for IPAddr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IPAddr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - address (ascending)
        self.address.cmp(&other.address)
    }
}

impl Validate for IPAddr {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "IPAddrList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IPAddrList {
    #[yaserde(rename = "IPAddr")]
    pub ip_addr: Vec<IPAddr>,

    /// The number specifying "all" of the items in the list. Required on a
    /// response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    /// Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for IPAddrList {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "IPInterface")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IPInterface {
    /// Use rules from [RFC 2863].
    #[yaserde(rename = "ifDescr")]
    pub if_descr: Option<String192>,

    /// Use rules from [RFC 2863].
    #[yaserde(rename = "ifHighSpeed")]
    pub if_high_speed: Option<Uint32>,

    /// Use rules from [RFC 2863].
    #[yaserde(rename = "ifInBroadcastPkts")]
    pub if_in_broadcast_pkts: Option<Uint32>,

    /// Use rules from [RFC 2863].
    #[yaserde(rename = "ifIndex")]
    pub if_index: Option<Uint32>,

    /// Use rules from [RFC 2863]. Can be thought of as Input Datagrams
    /// Discarded.
    #[yaserde(rename = "ifInDiscards")]
    pub if_in_discards: Option<Uint32>,

    /// Use rules from [RFC 2863].
    #[yaserde(rename = "ifInErrors")]
    pub if_in_errors: Option<Uint32>,

    /// Use rules from [RFC 2863]. Can be thought of as Multicast Datagrams
    /// Received.
    #[yaserde(rename = "ifInMulticastPkts")]
    pub if_in_multicast_pkts: Option<Uint32>,

    /// Use rules from [RFC 2863]. Can be thought of as Bytes Received.
    #[yaserde(rename = "ifInOctets")]
    pub if_in_octets: Option<Uint32>,

    /// Use rules from [RFC 2863]. Can be thought of as Datagrams Received.
    #[yaserde(rename = "ifInUcastPkts")]
    pub if_in_ucast_pkts: Option<Uint32>,

    /// Use rules from [RFC 2863]. Can be thought of as Datagrams with Unknown
    /// Protocol Received.
    #[yaserde(rename = "ifInUnknownProtos")]
    pub if_in_unknown_protos: Option<Uint32>,

    /// Use rules from [RFC 2863].
    #[yaserde(rename = "ifMtu")]
    pub if_mtu: Option<Uint32>,

    /// Use rules from [RFC 2863].
    #[yaserde(rename = "ifName")]
    pub if_name: Option<String16>,

    /// Use rules and assignments from [RFC 2863].
    #[yaserde(rename = "ifOperStatus")]
    pub if_oper_status: Option<Uint8>,

    /// Use rules from [RFC 2863]. Can be thought of as Broadcast Datagrams Sent.
    #[yaserde(rename = "ifOutBroadcastPkts")]
    pub if_out_broadcast_pkts: Option<Uint32>,

    /// Use rules from [RFC 2863]. Can be thought of as Output Datagrams
    /// Discarded.
    #[yaserde(rename = "ifOutDiscards")]
    pub if_out_discards: Option<Uint32>,

    /// Use rules from [RFC 2863].
    #[yaserde(rename = "ifOutErrors")]
    pub if_out_errors: Option<Uint32>,

    /// Use rules from [RFC 2863]. Can be thought of as Multicast Datagrams Sent.
    #[yaserde(rename = "ifOutMulticastPkts")]
    pub if_out_multicast_pkts: Option<Uint32>,

    /// Use rules from [RFC 2863]. Can be thought of as Bytes Sent.
    #[yaserde(rename = "ifOutOctets")]
    pub if_out_octets: Option<Uint32>,

    /// Use rules from [RFC 2863]. Can be thought of as Datagrams Sent.
    #[yaserde(rename = "ifOutUcastPkts")]
    pub if_out_ucast_pkts: Option<Uint32>,

    /// Use rules from [RFC 2863].
    #[yaserde(rename = "ifPromiscuousMode")]
    pub if_promiscuous_mode: Option<bool>,

    /// Use rules from [RFC 2863].
    #[yaserde(rename = "ifSpeed")]
    pub if_speed: Option<Uint32>,

    /// Use rules and assignments from [RFC 2863].
    #[yaserde(rename = "ifType")]
    pub if_type: Option<Uint16>,

    #[yaserde(rename = "IPAddrListLink")]
    pub ip_addr_list_link: Option<IPAddrListLink>,

    /// Similar to ifLastChange in [RFC 2863].
    #[yaserde(rename = "lastResetTime")]
    pub last_reset_time: Option<Int64>,

    /// The date/time of the reported status.
    #[yaserde(rename = "lastUpdatedTime")]
    pub last_updated_time: Option<Int64>,

    #[yaserde(rename = "LLInterfaceListLink")]
    pub ll_interface_list_link: Option<LlinterfaceListLink>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for IPInterface {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IPInterface {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - ifIndex (ascending)
        self.if_index.cmp(&other.if_index)
    }
}

impl Validate for IPInterface {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "IPInterfaceList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IPInterfaceList {
    #[yaserde(rename = "IPInterface")]
    pub ip_interface: Vec<IPInterface>,

    /// The default polling rate for this function set (this resource and all
    /// resources below), in seconds. If not specified, a default of 900 seconds
    /// (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    /// this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    /// The number specifying "all" of the items in the list. Required on a
    /// response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    /// Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for IPInterfaceList {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "LLInterface")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Llinterface {
    /// Contains the number of CRC errors since reset.
    #[yaserde(rename = "CRCerrors")]
    pub cr_cerrors: Uint32,

    /// Contains the EUI-64 of the link layer interface. 48 bit MAC addresses
    /// SHALL be changed into an EUI-64 using the method defined in [RFC 4291],
    /// Appendix A. (The method is to insert "0xFFFE" as described in the
    /// reference.)
    #[yaserde(rename = "EUI64")]
    pub eui64: HexBinary64,

    #[yaserde(rename = "IEEE_802_15_4")]
    pub ieee_802_15_4: Option<Ieee802154>,

    /// Specifies the type of link layer interface associated with the
    /// IPInterface. Values are below.
    /// 0 = Unspecified
    /// 1 = IEEE 802.3 (Ethernet)
    /// 2 = IEEE 802.11 (WLAN)
    /// 3 = IEEE 802.15 (PAN)
    /// 4 = IEEE 1901 (PLC)
    /// All other values reserved.
    #[yaserde(rename = "linkLayerType")]
    pub link_layer_type: Uint8,

    /// Number of times an ACK was not received for a frame transmitted (when ACK
    /// was requested).
    #[yaserde(rename = "LLAckNotRx")]
    pub ll_ack_not_rx: Option<Uint32>,

    /// Number of times CSMA failed.
    #[yaserde(rename = "LLCSMAFail")]
    pub llcsma_fail: Option<Uint32>,

    /// Number of dropped receive frames.
    #[yaserde(rename = "LLFramesDropRx")]
    pub ll_frames_drop_rx: Option<Uint32>,

    /// Number of dropped transmit frames.
    #[yaserde(rename = "LLFramesDropTx")]
    pub ll_frames_drop_tx: Option<Uint32>,

    /// Number of link layer frames received.
    #[yaserde(rename = "LLFramesRx")]
    pub ll_frames_rx: Option<Uint32>,

    /// Number of link layer frames transmitted.
    #[yaserde(rename = "LLFramesTx")]
    pub ll_frames_tx: Option<Uint32>,

    /// Number of times access to media failed.
    #[yaserde(rename = "LLMediaAccessFail")]
    pub ll_media_access_fail: Option<Uint32>,

    /// Number of Bytes received.
    #[yaserde(rename = "LLOctetsRx")]
    pub ll_octets_rx: Option<Uint32>,

    /// Number of Bytes transmitted.
    #[yaserde(rename = "LLOctetsTx")]
    pub ll_octets_tx: Option<Uint32>,

    /// Number of MAC transmit retries.
    #[yaserde(rename = "LLRetryCount")]
    pub ll_retry_count: Option<Uint32>,

    /// Number of receive security errors.
    #[yaserde(rename = "LLSecurityErrorRx")]
    pub ll_security_error_rx: Option<Uint32>,

    #[yaserde(rename = "loWPAN")]
    pub lo_wpan: Option<LoWPAN>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for Llinterface {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Llinterface {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - EUI64 (ascending)
        self.eui64.cmp(&other.eui64)
    }
}

impl Validate for Llinterface {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "LLInterfaceList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LlinterfaceList {
    #[yaserde(rename = "LLInterface")]
    pub ll_interface: Vec<Llinterface>,

    /// The number specifying "all" of the items in the list. Required on a
    /// response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    /// Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for LlinterfaceList {}

/// Contains information specific to 6LoWPAN.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "loWPAN")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LoWPAN {
    /// Number of Bytes received
    #[yaserde(rename = "octetsRx")]
    pub octets_rx: Option<Uint32>,

    /// Number of Bytes transmitted
    #[yaserde(rename = "octetsTx")]
    pub octets_tx: Option<Uint32>,

    /// Number of packets received
    #[yaserde(rename = "packetsRx")]
    pub packets_rx: Uint32,

    /// Number of packets transmitted
    #[yaserde(rename = "packetsTx")]
    pub packets_tx: Uint32,

    /// Number of errors receiving fragments
    #[yaserde(rename = "rxFragError")]
    pub rx_frag_error: Uint32,
}

impl Validate for LoWPAN {}

#[derive(
    Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, YaSerialize, YaDeserialize, SEResource,
)]
#[yaserde(rename = "Neighbor")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Neighbor {
    /// True if the neighbor is a child.
    #[yaserde(rename = "isChild")]
    pub is_child: bool,

    /// The quality of the link, as defined by 802.15.4
    #[yaserde(rename = "linkQuality")]
    pub link_quality: Uint8,

    /// As defined by IEEE 802.15.4
    #[yaserde(rename = "shortAddress")]
    pub short_address: Uint16,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Neighbor {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "NeighborList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct NeighborList {
    #[yaserde(rename = "Neighbor")]
    pub neighbor: Vec<Neighbor>,

    /// The number specifying "all" of the items in the list. Required on a
    /// response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    /// Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for NeighborList {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "RPLInstance")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Rplinstance {
    /// See [RFC 6550].
    #[yaserde(rename = "DODAGid")]
    pub doda_gid: Uint8,

    /// See [RFC 6550].
    #[yaserde(rename = "DODAGroot")]
    pub doda_groot: bool,

    /// See [RFC 6550].
    #[yaserde(rename = "flags")]
    pub flags: Uint8,

    /// See [RFC 6550].
    #[yaserde(rename = "groundedFlag")]
    pub grounded_flag: bool,

    /// See [RFC 6550].
    #[yaserde(rename = "MOP")]
    pub mop: Uint8,

    /// See [RFC 6550].
    #[yaserde(rename = "PRF")]
    pub prf: Uint8,

    /// See [RFC 6550].
    #[yaserde(rename = "rank")]
    pub rank: Uint16,

    /// See [RFC 6550].
    #[yaserde(rename = "RPLInstanceID")]
    pub rpl_instance_id: Uint8,

    #[yaserde(rename = "RPLSourceRoutesListLink")]
    pub rpl_source_routes_list_link: Option<RplsourceRoutesListLink>,

    /// See [RFC 6550].
    #[yaserde(rename = "versionNumber")]
    pub version_number: Uint8,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for Rplinstance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rplinstance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - DODAGid (ascending)
        match self.doda_gid.cmp(&other.doda_gid) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - RPLInstanceID (ascending)
        self.rpl_instance_id.cmp(&other.rpl_instance_id)
    }
}

impl Validate for Rplinstance {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "RPLInstanceList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RplinstanceList {
    #[yaserde(rename = "RPLInstance")]
    pub rpl_instance: Vec<Rplinstance>,

    /// The number specifying "all" of the items in the list. Required on a
    /// response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    /// Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RplinstanceList {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "RPLSourceRoutes")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RplsourceRoutes {
    /// See [RFC 6554].
    #[yaserde(rename = "DestAddress")]
    pub dest_address: HexBinary128,

    /// See [RFC 6554].
    #[yaserde(rename = "SourceRoute")]
    pub source_route: HexBinary128,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for RplsourceRoutes {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RplsourceRoutes {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - SourceRoute (ascending)
        match self.source_route.cmp(&other.source_route) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - DestAddress (ascending)
        self.dest_address.cmp(&other.dest_address)
    }
}

impl Validate for RplsourceRoutes {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "RPLSourceRoutesList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RplsourceRoutesList {
    #[yaserde(rename = "RPLSourceRoutes")]
    pub rpl_source_routes: Vec<RplsourceRoutes>,

    /// The number specifying "all" of the items in the list. Required on a
    /// response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    /// Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RplsourceRoutesList {}
