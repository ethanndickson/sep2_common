use crate::traits::{SEList, SEResource, SESubscribableList, SESubscribableResource, Validate};
use sep2_common_derive::{SEList, SEResource, SESubscribableList, SESubscribableResource};

use super::{
    primitives::{String32, Uint16, Uint32, Uint8},
    types::{PENType, SubscribableType, TimeType},
};
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "LogEvent")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LogEvent {
    /// The date and time that the event occurred.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: TimeType,

    /// Human readable text that MAY be used to transmit additional details about
    /// the event. A host MAY remove this field when received.
    #[yaserde(rename = "details")]
    pub details: Option<String32>,

    /// May be used to transmit additional details about the event.
    #[yaserde(rename = "extendedData")]
    pub extended_data: Option<Uint32>,

    /// If the profileID indicates this is IEEE 2030.5, the functionSet is
    /// defined by IEEE 2030.5 and SHALL be one of the values from the table
    /// below (IEEE 2030.5 function set identifiers). If the profileID is
    /// anything else, the functionSet is defined by the identified profile.
    /// 0 General (not specific to a function set)
    /// 1 Publish and Subscribe
    /// 2 End Device
    /// 3 Function Set Assignment
    /// 4 Response
    /// 5 Demand Response and Load Control
    /// 6 Metering
    /// 7 Pricing
    /// 8 Messaging
    /// 9 Billing
    /// 10 Prepayment
    /// 11 Distributed Energy Resources
    /// 12 Time
    /// 13 Software Download
    /// 14 Device Information
    /// 15 Power Status
    /// 16 Network Status
    /// 17 Log Event List
    /// 18 Configuration
    /// 19 Security
    /// All other values are reserved.
    #[yaserde(rename = "functionSet")]
    pub function_set: FunctionSets,

    /// An 8 bit unsigned integer. logEventCodes are scoped to a profile and a
    /// function set. If the profile is IEEE 2030.5, the logEventCode is defined
    /// by IEEE 2030.5 within one of the function sets of IEEE 2030.5. If the
    /// profile is anything else, the logEventCode is defined by the specified
    /// profile.
    #[yaserde(rename = "logEventCode")]
    pub log_event_code: Uint8,

    /// This 16-bit value, combined with createdDateTime, profileID, and
    /// logEventPEN, should provide a reasonable level of uniqueness.
    #[yaserde(rename = "logEventID")]
    pub log_event_id: Uint16,

    /// The Private Enterprise Number(PEN) of the entity that defined the
    /// profileID, functionSet, and logEventCode of the logEvent. IEEE
    /// 2030.5-assigned logEventCodes SHALL use the IEEE 2030.5 PEN. Combinations
    /// of profileID, functionSet, and logEventCode SHALL have unique meaning
    /// within a logEventPEN and are defined by the owner of the PEN.
    #[yaserde(rename = "logEventPEN")]
    pub log_event_pen: PENType,

    /// The profileID identifies which profile (HA, BA, SE, etc) defines the
    /// following event information.
    /// 0 Not profile specific.
    /// 1 Vendor Defined
    /// 2 IEEE 2030.5
    /// 3 Home Automation
    /// 4 Building Automation
    /// All other values are reserved.
    #[yaserde(rename = "profileID")]
    pub profile_id: ProfileID,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for LogEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LogEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - createdDateTime (descending)
        match self
            .created_date_time
            .cmp(&other.created_date_time)
            .reverse()
        {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - LogEventID (descending)
        self.log_event_id.cmp(&other.log_event_id).reverse()
    }
}

impl Validate for LogEvent {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum FunctionSets {
    #[default]
    General = 0,
    PubSub = 1,
    EndDevice = 2,
    FSA = 3,
    Response = 4,
    DRLC = 5,
    Metering = 6,
    Pricing = 7,
    Messaging = 8,
    Billing = 9,
    Prepayment = 10,
    DER = 11,
    Time = 12,
    SoftwareDownload = 13,
    DeviceInformation = 14,
    PowerStatus = 15,
    NetworkStatus = 16,
    LogEventList = 17,
    Configuration = 18,
    Security = 19,
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum ProfileID {
    #[default]
    NonSpecific = 0,
    VendorDefined = 1,
    IEEE20305 = 2,
    HomeAutomation = 3,
    BuildingAutomation = 4,
}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SESubscribableList,
    SEList,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "LogEventList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LogEventList {
    #[yaserde(rename = "LogEvent")]
    pub log_event: Vec<LogEvent>,

    /// The default polling rate for this function set (this resource and all
    /// resources below), in seconds. If not specified, a default of 900 seconds
    /// (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    /// this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    /// The number specifying "all" of the items in the list. Required on GET,
    /// ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    /// Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    /// Indicates whether or not subscriptions are supported for this resource,
    /// and whether or not conditional (thresholds) are supported. If not
    /// specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for LogEventList {}
