use std::fmt::Display;

use sep2_common_derive::{
    SEEvent, SEIdentifiedObject, SEResource, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SESubscribableResource,
};

use crate::traits::{
    SEEvent, SEIdentifiedObject, SEResource, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SESubscribableResource, Validate,
};

use sepserde::{YaDeserialize, YaSerialize};

use super::{
    identification::ResponseRequired,
    primitives::{String192, String32, Uint16},
    types::{
        DateTimeInterval, MRIDType, OneHourRangeType, SubscribableType, TimeType, VersionType,
    },
};

/// Current status information relevant to a specific object. The Status object
/// is used to indicate the current status of an Event. Devices can read the
/// containing resource (e.g. TextMessage) to get the most up to date status of
/// the event. Devices can also subscribe to a specific resource instance to get
/// updates when any of its attributes change, including the Status object.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "EventStatus")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EventStatus {
    /// Field representing the current status type.
    /// 0 = Scheduled
    /// This status indicates that the event has been scheduled and the event has
    /// not yet started. The server SHALL set the event to this status when the
    /// event is first scheduled and persist until the event has become active or
    /// has been cancelled. For events with a start time less than or equal to
    /// the current time, this status SHALL never be indicated, the event SHALL
    /// start with a status of “Active”.
    /// 1 = Active
    /// This status indicates that the event is currently active. The server
    /// SHALL set the event to this status when the event reaches its earliest
    /// Effective Start Time.
    /// 2 = Cancelled
    /// When events are cancelled, the Status.dateTime attribute SHALL be set to
    /// the time the cancellation occurred, which cannot be in the future. The
    /// server is responsible for maintaining the cancelled event in its
    /// collection for the duration of the original event, or until the server
    /// has run out of space and needs to store a new event. Client devices SHALL
    /// be aware of Cancelled events, determine if the Cancelled event applies to
    /// them, and cancel the event immediately if applicable.
    /// 3 = Cancelled with Randomization
    /// The server is responsible for maintaining the cancelled event in its
    /// collection for the duration of the Effective Scheduled Period. Client
    /// devices SHALL be aware of Cancelled with Randomization events, determine
    /// if the Cancelled event applies to them, and cancel the event immediately,
    /// using the larger of (absolute value of randomizeStart) and (absolute
    /// value of randomizeDuration) as the end randomization, in seconds. This
    /// Status.type SHALL NOT be used with "regular" Events, only with
    /// specializations of RandomizableEvent.
    /// 4 = Superseded
    /// Events marked as Superseded by servers are events that may have been
    /// replaced by new events from the same program that target the exact same
    /// set of deviceCategory's (if applicable) AND DERControl controls (e.g.,
    /// opModTargetW) (if applicable) and overlap for a given period of time.
    /// Servers SHALL mark an event as Superseded at the earliest Effective Start
    /// Time of the overlapping event. Servers are responsible for maintaining
    /// the Superseded event in their collection for the duration of the
    /// Effective Scheduled Period.
    /// Client devices encountering a Superseded event SHALL terminate execution
    /// of the event immediately and commence execution of the new event
    /// immediately, unless the current time is within the start randomization
    /// window of the superseded event, in which case the client SHALL obey the
    /// start randomization of the new event. This Status.type SHALL NOT be used
    /// with TextMessage, since multiple text messages can be active.
    /// All other values reserved.
    #[yaserde(rename = "currentStatus")]
    pub current_status: EventStatusType,

    /// The dateTime attribute will provide a timestamp of when the current
    /// status was defined. dateTime MUST be set to the time at which the status
    /// change occurred, not a time in the future or past.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    /// Set to true by a server of this event if there are events that overlap
    /// this event in time and also overlap in some, but not all,
    /// deviceCategory's (if applicable) AND DERControl controls (e.g.,
    /// opModTargetW) (if applicable) in the same function set instance.
    #[yaserde(rename = "potentiallySuperseded")]
    pub potentially_superseded: bool,

    /// Indicates the time that the potentiallySuperseded flag was set.
    #[yaserde(rename = "potentiallySupersededTime")]
    pub potentially_superseded_time: Option<TimeType>,

    /// The Reason attribute allows a Service provider to provide a textual
    /// explanation of the status.
    #[yaserde(rename = "reason")]
    pub reason: Option<String192>,
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum EventStatusType {
    #[default]
    Scheduled,
    Active,
    Cancelled,
    CancelledRandom,
    Superseded,
}

impl Validate for EventStatus {}

/// An Event indicates information that applies to a particular period of time. Events SHALL be executed relative to the time of the server, as described in the Time function set section 11.1.
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
#[yaserde(rename = "Event")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Event {
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

impl Validate for Event {}

/// Contains information about the nature of an error if a request could not be
/// completed successfully.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "Error")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Error {
    /// Contains the number of seconds the client SHOULD wait before retrying the
    /// request.
    #[yaserde(rename = "maxRetryDuration")]
    pub max_retry_duration: Option<Uint16>,

    /// Code indicating the reason for failure.
    /// 0 - Invalid request format
    /// 1 - Invalid request values (e.g. invalid threshold values)
    /// 2 - Resource limit reached
    /// 3 - Conditional subscription field not supported
    /// 4 - Maximum request frequency exceeded
    /// All other values reserved
    #[yaserde(rename = "reasonCode")]
    pub reason_code: ErrorReason,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.max_retry_duration {
            Some(d) => write!(
                f,
                "Request Error - Max Retry Duration: {} - Reason: {}",
                d, self.reason_code
            ),
            None => write!(f, "Request Error - Reason: {}", self.reason_code),
        }
    }
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u16)]
pub enum ErrorReason {
    #[default]
    InvalidRequestFormat = 0,
    InvalidRequestValues = 1,
    ResourceLimitReached = 2,
    SubscriptionNotSupported = 3,
    MaximumRequestFrequency = 4,
}

impl Display for ErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorReason::InvalidRequestFormat => write!(f, "Invalid request format"),
            ErrorReason::InvalidRequestValues => {
                write!(f, "Invalid request values (e.g. invalid threshold values)")
            }
            ErrorReason::ResourceLimitReached => write!(f, "Resource limit reached"),
            ErrorReason::SubscriptionNotSupported => {
                write!(f, "Conditional subscription field not supported")
            }
            ErrorReason::MaximumRequestFrequency => write!(f, "Maximum request frequency exceeded"),
        }
    }
}
impl Validate for Error {}

/// An Event that can indicate time ranges over which the start time
/// and duration SHALL be randomized.
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
#[yaserde(rename = "RandomizableEvent")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RandomizableEvent {
    /// Number of seconds boundary inside which a random value must be selected
    /// to be applied to the associated interval duration, to avoid sudden
    /// synchronized demand changes. If related to price level changes, sign may
    /// be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    /// default.
    #[yaserde(rename = "randomizeDuration")]
    pub randomize_duration: Option<OneHourRangeType>,

    /// Number of seconds boundary inside which a random value must be selected
    /// to be applied to the associated interval start time, to avoid sudden
    /// synchronized demand changes. If related to price level changes, sign may
    /// be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    /// default.
    #[yaserde(rename = "randomizeStart")]
    pub randomize_start: Option<OneHourRangeType>,

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

impl Validate for RandomizableEvent {}
