use crate::traits::{
    SEEvent, SEIdentifiedObject, SEList, SEResource, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SESubscribableIdentifiedObject, SESubscribableList,
    SESubscribableResource, Validate,
};
use sep2_common_derive::{
    SEEvent, SEIdentifiedObject, SEList, SEResource, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SESubscribableIdentifiedObject, SESubscribableList,
    SESubscribableResource,
};

use super::{
    identification::ResponseRequired,
    links::{ActiveTextMessageListLink, TextMessageListLink},
    objects::EventStatus,
    primitives::{String20, String32, Uint32},
    types::{
        DateTimeInterval, LocaleType, MRIDType, PrimacyType, SubscribableType, TimeType,
        VersionType,
    },
};
use yaserde::{YaDeserialize, YaSerialize};

/// Provides a container for collections of text messages.
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
#[yaserde(rename = "MessagingProgram")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MessagingProgram {
    #[yaserde(rename = "ActiveTextMessageListLink")]
    pub active_text_message_list_link: Option<ActiveTextMessageListLink>,

    /// Indicates the language and region of the messages in this collection.
    #[yaserde(rename = "locale")]
    pub locale: LocaleType,

    /// Indicates the relative primacy of the provider of this program.
    #[yaserde(rename = "primacy")]
    pub primacy: PrimacyType,

    #[yaserde(rename = "TextMessageListLink")]
    pub text_message_list_link: Option<TextMessageListLink>,

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

impl PartialOrd for MessagingProgram {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MessagingProgram {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - mRID (descending)
        self.mrid.cmp(&other.mrid)
    }
}

impl Validate for MessagingProgram {}

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
#[yaserde(rename = "MessagingProgramList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MessagingProgramList {
    #[yaserde(rename = "MessagingProgram")]
    pub messaging_program: Vec<MessagingProgram>,

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

impl Validate for MessagingProgramList {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "PriorityType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum PriorityType {
    #[default]
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
    // 4-255 RESERVED
}

impl Validate for PriorityType {}

/// Text message such as a notification.
#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEEvent,
    SERespondableSubscribableIdentifiedObject,
    SEIdentifiedObject,
    SESubscribableResource,
    SERespondableResource,
    SEResource,
)]
#[yaserde(rename = "TextMessage")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TextMessage {
    /// Indicates the human-readable name of the publisher of the message
    #[yaserde(rename = "originator")]
    pub originator: Option<String20>,

    /// The priority is used to inform the client of the priority of the
    /// particular message. Devices with constrained or limited resources for
    /// displaying Messages should use this attribute to determine how to handle
    /// displaying currently active Messages (e.g. if a device uses a scrolling
    /// method with a single Message viewable at a time it MAY want to push a low
    /// priority Message to the background and bring a newly received higher
    /// priority Message to the foreground).
    #[yaserde(rename = "priority")]
    pub priority: PriorityType,

    /// The textMessage attribute contains the actual UTF-8 encoded text to be
    /// displayed in conjunction with the messageLength attribute which contains
    /// the overall length of the textMessage attribute. Clients and servers
    /// SHALL support a reception of a Message of 100 bytes in length. Messages
    /// that exceed the clients display size will be left to the client to choose
    /// what method to handle the message (truncation, scrolling, etc.).
    #[yaserde(rename = "textMessage")]
    pub text_message: String,

    /// The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    /// The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

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

impl PartialOrd for TextMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TextMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - mRID (ascending)
        match self.interval.start.cmp(&other.interval.start) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - createdDateTime (descending)
        match self.creation_time.cmp(&other.creation_time).reverse() {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Tertiary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for TextMessage {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "TextMessageList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TextMessageList {
    #[yaserde(rename = "TextMessage")]
    pub text_message: Vec<TextMessage>,

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

impl Validate for TextMessageList {}
