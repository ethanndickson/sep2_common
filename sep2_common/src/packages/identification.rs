use bitflags::bitflags;
use sep2_common_derive::{
    SEIdentifiedObject, SEList, SEResource, SERespondableIdentifiedObject, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SEResponse, SESubscribableIdentifiedObject,
    SESubscribableList, SESubscribableResource,
};
use sepserde::{HexBinaryYaSerde, YaDeserialize, YaSerialize};

use super::{
    objects::EventStatusType,
    primitives::{HexBinary160, String32, Uint32},
    types::{MRIDType, SubscribableType, TimeType, VersionType},
};

use crate::traits::{
    SEIdentifiedObject, SEList, SEResource, SERespondableIdentifiedObject, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SEResponse, SESubscribableIdentifiedObject,
    SESubscribableList, SESubscribableResource, Validate,
};

/// A resource is an addressable unit of information, either a collection (List)
/// or instance of an object (identifiedObject, or simply, Resource)
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "Resource")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Resource {
    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Resource {}

/// The Response object is the generic response data
/// repository which is extended for specific function sets.
#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResponse, SEResource,
)]
#[yaserde(rename = "Response")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Response {
    /// The createdDateTime field contains the date and time when the
    /// acknowledgement/status occurred in the client. The client will provide
    /// the timestamp to ensure the proper time is captured in case the response
    /// is delayed in reaching the server (server receipt time would not be the
    /// same as the actual confirmation time). The time reported from the client
    /// should be relative to the time server indicated by the
    /// FunctionSetAssignment that also indicated the event resource; if no
    /// FunctionSetAssignment exists, the time of the server where the event
    /// resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    /// Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    /// The status field contains the acknowledgement or status. Each event type
    /// (DRLC, DER, Price, or Text) can return different status information (e.g.
    /// an Acknowledge will be returned for a Price event where a DRLC event can
    /// return Event Received, Event Started, and Event Completed). The Status
    /// field value definitions are defined in Table 27: Response Types by
    /// Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<ResponseStatus>,

    /// The subject field provides a method to match the response with the
    /// originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: MRIDType,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for Response {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Response {
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
        // Secondary Key - endDeviceLFDI (ascending)
        self.end_device_lfdi.cmp(&other.end_device_lfdi)
    }
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
/// Defined in Table 27: Response Types by Function Set
/// Can be created from a [`EventStatusType`] for the purpose of reading Event resources
pub enum ResponseStatus {
    #[default]
    EventReceived = 1,
    EventStarted = 2,
    EventCompleted = 3,
    EventOptOut = 4,
    EventOptIn = 5,
    EventCancelled = 6,
    EventSuperseded = 7,
    EventPartialOptOut = 8,
    EventPartialOptIn = 9,
    EventCompletedNoUserParticipation = 10,
    EventAcknowledge = 11,
    EventNoDisplay = 12,
    EventAbortedServer = 13,
    EventAbortedProgram = 14,
    EventNotApplicable = 252,
    EventInvalid = 253,
    EventExpired = 254,
}

impl From<EventStatusType> for ResponseStatus {
    fn from(value: EventStatusType) -> Self {
        match value {
            EventStatusType::Scheduled => ResponseStatus::EventReceived,
            EventStatusType::Active => ResponseStatus::EventStarted,
            EventStatusType::Cancelled => ResponseStatus::EventCancelled,
            EventStatusType::CancelledRandom => ResponseStatus::EventCancelled,
            EventStatusType::Superseded => ResponseStatus::EventSuperseded,
        }
    }
}

impl Validate for Response {}

/// Container to hold a collection of object instances or references.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "List")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct List {
    /// This field is OOS since it is different in every child type
    /// but is required for our SEList trait implementation
    #[yaserde(skip_serializing)]
    pub contents: Vec<String>,

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

impl Validate for List {}

/// Links provide a reference, via URI, to another resource.
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "Link")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Link {
    /// A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for Link {}

/// ListLinks provide a reference, via URI, to a List.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ListLink {
    /// Indicates the total number of items in the referenced list. This
    /// attribute SHALL be present if the href is a local or relative URI. This
    /// attribute SHOULD NOT be present if the href is a remote or absolute URI,
    /// as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    /// A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}
impl Validate for ListLink {}

/// This is a root class to provide common naming attributes for all classes needing naming attributes
#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "IdentifiedObject")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IdentifiedObject {
    /// The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    /// The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    /// Contains the version number of the object. See the type definition for
    /// details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}
impl Validate for IdentifiedObject {}

/// A Resource to which a Response can be requested.
#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SERespondableResource,
    SEResource,
)]
#[yaserde(rename = "RespondableResource")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RespondableResource {
    /// A reference to the response resource address (URI). Required on a
    /// response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    /// Indicates whether or not a response is required upon receipt, creation or
    /// update of this resource. Responses shall be posted to the collection
    /// specified in "replyTo".
    /// If the resource has a deviceCategory field, devices that match one or
    /// more of the device types indicated in deviceCategory SHALL respond
    /// according to the rules listed below. If the category does not match, the
    /// device SHALL NOT respond. If the resource does not have a deviceCategory
    /// field, a device receiving the resource SHALL respond according to the
    /// rules listed below.
    /// Value encoded as hex according to the following bit assignments, any
    /// combination is possible.
    /// See Table 27 for the list of appropriate Response status codes to be sent
    /// for these purposes.
    /// 0 - End device shall indicate that message was received
    /// 1 - End device shall indicate specific response.
    /// 2 - End user / customer response is required.
    /// All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<ResponseRequired>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

bitflags! {
    #[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, HexBinaryYaSerde)]
    pub struct ResponseRequired: u8 { // HexBinary8
        const MessageReceived = 1;
        const SpecificResponse = 2;
        const ResponseRequired = 4;
    }
}

impl Validate for RespondableResource {}

/// An IdentifiedObject to which a Response can be requested.
#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SERespondableIdentifiedObject,
    SEIdentifiedObject,
    SERespondableResource,
    SEResource,
)]
#[yaserde(rename = "RespondableIdentifiedObject")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RespondableIdentifiedObject {
    /// The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    /// The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    /// Contains the version number of the object. See the type definition for
    /// details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    /// A reference to the response resource address (URI). Required on a
    /// response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    /// Indicates whether or not a response is required upon receipt, creation or
    /// update of this resource. Responses shall be posted to the collection
    /// specified in "replyTo".
    /// If the resource has a deviceCategory field, devices that match one or
    /// more of the device types indicated in deviceCategory SHALL respond
    /// according to the rules listed below. If the category does not match, the
    /// device SHALL NOT respond. If the resource does not have a deviceCategory
    /// field, a device receiving the resource SHALL respond according to the
    /// rules listed below.
    /// Value encoded as hex according to the following bit assignments, any
    /// combination is possible.
    /// See Table 27 for the list of appropriate Response status codes to be sent
    /// for these purposes.
    /// 0 - End device shall indicate that message was received
    /// 1 - End device shall indicate specific response.
    /// 2 - End user / customer response is required.
    /// All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<ResponseRequired>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RespondableIdentifiedObject {}

/// An IdentifiedObject to which a Response can be requested.
#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SERespondableSubscribableIdentifiedObject,
    SESubscribableResource,
    SEIdentifiedObject,
    SERespondableResource,
    SEResource,
)]
#[yaserde(rename = "RespondableSubscribableIdentifiedObject")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RespondableSubscribableIdentifiedObject {
    /// The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    /// The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    /// Contains the version number of the object. See the type definition for
    /// details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    /// Indicates whether or not subscriptions are supported for this resource,
    /// and whether or not conditional (thresholds) are supported. If not
    /// specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    /// A reference to the response resource address (URI). Required on a
    /// response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    /// Indicates whether or not a response is required upon receipt, creation or
    /// update of this resource. Responses shall be posted to the collection
    /// specified in "replyTo".
    /// If the resource has a deviceCategory field, devices that match one or
    /// more of the device types indicated in deviceCategory SHALL respond
    /// according to the rules listed below. If the category does not match, the
    /// device SHALL NOT respond. If the resource does not have a deviceCategory
    /// field, a device receiving the resource SHALL respond according to the
    /// rules listed below.
    /// Value encoded as hex according to the following bit assignments, any
    /// combination is possible.
    /// See Table 27 for the list of appropriate Response status codes to be sent
    /// for these purposes.
    /// 0 - End device shall indicate that message was received
    /// 1 - End device shall indicate specific response.
    /// 2 - End user / customer response is required.
    /// All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<ResponseRequired>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RespondableSubscribableIdentifiedObject {}

/// A Resource to which a Subscription can be requested.
#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "SubscribableResource")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscribableResource {
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

impl Validate for SubscribableResource {}

/// A List to which a Subscription can be requested.
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
#[yaserde(rename = "SubscribableList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscribableList {
    /// This field is OOS since it is different in every child type
    /// but is required for our SEList trait implementation
    #[yaserde(skip_serializing)]
    pub contents: Vec<String>,

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

impl Validate for SubscribableList {}

/// An IdentifiedObject to which a Response can be requested.
#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SESubscribableIdentifiedObject,
    SEIdentifiedObject,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "SubscribableIdentifiedObject")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscribableIdentifiedObject {
    /// The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    /// The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    /// Contains the version number of the object. See the type definition for
    /// details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

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

impl Validate for SubscribableIdentifiedObject {}
