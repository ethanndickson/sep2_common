use common_derive::{SEIdentifiedObject, SEList, SEResource, SEResponse};
// File auto-generated using xsd-parser-rs & IEEE 2030.5 sep-ordered-dep.xsd
use xsd_parser::generator::validator::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

// TODO Ethan: Temporary import all
use crate::packages::primitives::*;
use crate::packages::xsd::*;

use super::identification::ResponseStatus;
use super::traits::*;
use super::types::{Mridtype, TimeType, UnitType, VersionType};

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResponse, SEResource,
)]
#[yaserde(rename = "DERControlResponse")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DercontrolResponse {
    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<ResponseStatus>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DercontrolResponse {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResponse, SEResource,
)]
#[yaserde(rename = "FlowReservationResponseResponse")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationResponseResponse {
    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<ResponseStatus>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FlowReservationResponseResponse {}

// Specifies the value of the TargetReduction applied by the device.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "AppliedTargetReduction")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AppliedTargetReduction {
    // Enumerated field representing the type of reduction requested.
    #[yaserde(rename = "type")]
    pub _type: UnitType,

    // Indicates the requested amount of the relevant commodity to be reduced.
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for AppliedTargetReduction {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResponse, SEResource,
)]
#[yaserde(rename = "DrResponse")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DrResponse {
    #[yaserde(rename = "ApplianceLoadReduction")]
    pub appliance_load_reduction: Option<ApplianceLoadReduction>,

    #[yaserde(rename = "AppliedTargetReduction")]
    pub applied_target_reduction: Option<AppliedTargetReduction>,

    #[yaserde(rename = "DutyCycle")]
    pub duty_cycle: Option<DutyCycle>,

    #[yaserde(rename = "Offset")]
    pub offset: Option<Offset>,

    // Indicates the amount of time, in seconds, that the client partially
    // opts-out during the demand response event. When overriding within the
    // allowed override duration, the client SHALL send a partial opt-out
    // (Response status code 8) for partial opt-out upon completion, with the
    // total time the event was overridden (this attribute) populated. The
    // client SHALL send a no participation status response (status type 10) if
    // the user partially opts-out for longer than
    // EndDeviceControl.overrideDuration.
    #[yaserde(rename = "overrideDuration")]
    pub override_duration: Option<Uint16>,

    #[yaserde(rename = "SetPoint")]
    pub set_point: Option<SetPoint>,

    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<ResponseStatus>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DrResponse {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResponse, SEResource,
)]
#[yaserde(rename = "PriceResponse")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PriceResponse {
    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<ResponseStatus>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for PriceResponse {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "ResponseList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ResponseList<T: SEResponse> {
    #[yaserde(rename = "Response")]
    pub response: Vec<T>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl<T: SEResponse> Validate for ResponseList<T> {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "ResponseSet")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ResponseSet {
    #[yaserde(rename = "ResponseListLink")]
    pub response_list_link: Option<ResponseListLink>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for ResponseSet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ResponseSet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for ResponseSet {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "ResponseSetList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ResponseSetList {
    #[yaserde(rename = "ResponseSet")]
    pub response_set: Vec<ResponseSet>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ResponseSetList {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResponse, SEResource,
)]
#[yaserde(rename = "TextResponse")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TextResponse {
    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<ResponseStatus>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for TextResponse {}
