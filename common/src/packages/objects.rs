use xsd_parser::generator::validator::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

// TODO Ethan: Temporary import all

use crate::packages::primitives::*;
use crate::packages::xsd::*;

use super::traits::*;

// Current status information relevant to a specific object. The Status object
// is used to indicate the current status of an Event. Devices can read the
// containing resource (e.g. TextMessage) to get the most up to date status of
// the event. Devices can also subscribe to a specific resource instance to get
// updates when any of its attributes change, including the Status object.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EventStatus {
    // Field representing the current status type.
    // 0 = Scheduled
    // This status indicates that the event has been scheduled and the event has
    // not yet started. The server SHALL set the event to this status when the
    // event is first scheduled and persist until the event has become active or
    // has been cancelled. For events with a start time less than or equal to
    // the current time, this status SHALL never be indicated, the event SHALL
    // start with a status of “Active”.
    // 1 = Active
    // This status indicates that the event is currently active. The server
    // SHALL set the event to this status when the event reaches its earliest
    // Effective Start Time.
    // 2 = Cancelled
    // When events are cancelled, the Status.dateTime attribute SHALL be set to
    // the time the cancellation occurred, which cannot be in the future. The
    // server is responsible for maintaining the cancelled event in its
    // collection for the duration of the original event, or until the server
    // has run out of space and needs to store a new event. Client devices SHALL
    // be aware of Cancelled events, determine if the Cancelled event applies to
    // them, and cancel the event immediately if applicable.
    // 3 = Cancelled with Randomization
    // The server is responsible for maintaining the cancelled event in its
    // collection for the duration of the Effective Scheduled Period. Client
    // devices SHALL be aware of Cancelled with Randomization events, determine
    // if the Cancelled event applies to them, and cancel the event immediately,
    // using the larger of (absolute value of randomizeStart) and (absolute
    // value of randomizeDuration) as the end randomization, in seconds. This
    // Status.type SHALL NOT be used with "regular" Events, only with
    // specializations of RandomizableEvent.
    // 4 = Superseded
    // Events marked as Superseded by servers are events that may have been
    // replaced by new events from the same program that target the exact same
    // set of deviceCategory's (if applicable) AND DERControl controls (e.g.,
    // opModTargetW) (if applicable) and overlap for a given period of time.
    // Servers SHALL mark an event as Superseded at the earliest Effective Start
    // Time of the overlapping event. Servers are responsible for maintaining
    // the Superseded event in their collection for the duration of the
    // Effective Scheduled Period.
    // Client devices encountering a Superseded event SHALL terminate execution
    // of the event immediately and commence execution of the new event
    // immediately, unless the current time is within the start randomization
    // window of the superseded event, in which case the client SHALL obey the
    // start randomization of the new event. This Status.type SHALL NOT be used
    // with TextMessage, since multiple text messages can be active.
    // All other values reserved.
    #[yaserde(rename = "currentStatus")]
    pub current_status: Uint8,

    // The dateTime attribute will provide a timestamp of when the current
    // status was defined. dateTime MUST be set to the time at which the status
    // change occurred, not a time in the future or past.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    // Set to true by a server of this event if there are events that overlap
    // this event in time and also overlap in some, but not all,
    // deviceCategory's (if applicable) AND DERControl controls (e.g.,
    // opModTargetW) (if applicable) in the same function set instance.
    #[yaserde(rename = "potentiallySuperseded")]
    pub potentially_superseded: bool,

    // Indicates the time that the potentiallySuperseded flag was set.
    #[yaserde(rename = "potentiallySupersededTime")]
    pub potentially_superseded_time: Option<TimeType>,

    // The Reason attribute allows a Service provider to provide a textual
    // explanation of the status.
    #[yaserde(rename = "reason")]
    pub reason: Option<String192>,
}

impl Validate for EventStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Event {
    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl SEEvent for Event {}
impl SERespondableSubscribableIdentifiedObject for Event {}
impl SERespondableResource for Event {}
impl SEResource for Event {}
impl Validate for Event {}

// Contains information about the nature of an error if a request could not be
// completed successfully.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Error {
    // Contains the number of seconds the client SHOULD wait before retrying the
    // request.
    #[yaserde(rename = "maxRetryDuration")]
    pub max_retry_duration: Option<Uint16>,

    // Code indicating the reason for failure.
    // 0 - Invalid request format
    // 1 - Invalid request values (e.g. invalid threshold values)
    // 2 - Resource limit reached
    // 3 - Conditional subscription field not supported
    // 4 - Maximum request frequency exceeded
    // All other values reserved
    #[yaserde(rename = "reasonCode")]
    pub reason_code: Uint16,
}

impl Validate for Error {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RandomizableEvent {
    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval duration, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeDuration")]
    pub randomize_duration: Option<OneHourRangeType>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval start time, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeStart")]
    pub randomize_start: Option<OneHourRangeType>,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl SEEvent for RandomizableEvent {}
impl SERespondableSubscribableIdentifiedObject for RandomizableEvent {}
impl SERespondableResource for RandomizableEvent {}
impl SEResource for RandomizableEvent {}
impl Validate for RandomizableEvent {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Dercontrol {
    #[yaserde(rename = "DERControlBase")]
    pub der_control_base: DercontrolBase,

    // Specifies the bitmap indicating the categories of devices that SHOULD
    // respond. Devices SHOULD ignore events that do not indicate their device
    // category. If not present, all devices SHOULD respond.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: Option<DeviceCategoryType>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval duration, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeDuration")]
    pub randomize_duration: Option<OneHourRangeType>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval start time, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeStart")]
    pub randomize_start: Option<OneHourRangeType>,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl SERandomizableEvent for Dercontrol {}
impl SEEvent for Dercontrol {}
impl SERespondableSubscribableIdentifiedObject for Dercontrol {}
impl SERespondableResource for Dercontrol {}
impl SEResource for Dercontrol {}
impl Validate for Dercontrol {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TextMessage {
    // Indicates the human-readable name of the publisher of the message
    #[yaserde(rename = "originator")]
    pub originator: Option<String20>,

    // The priority is used to inform the client of the priority of the
    // particular message. Devices with constrained or limited resources for
    // displaying Messages should use this attribute to determine how to handle
    // displaying currently active Messages (e.g. if a device uses a scrolling
    // method with a single Message viewable at a time it MAY want to push a low
    // priority Message to the background and bring a newly received higher
    // priority Message to the foreground).
    #[yaserde(rename = "priority")]
    pub priority: PriorityType,

    // The textMessage attribute contains the actual UTF-8 encoded text to be
    // displayed in conjunction with the messageLength attribute which contains
    // the overall length of the textMessage attribute. Clients and servers
    // SHALL support a reception of a Message of 100 bytes in length. Messages
    // that exceed the clients display size will be left to the client to choose
    // what method to handle the message (truncation, scrolling, etc.).
    #[yaserde(rename = "textMessage")]
    pub text_message: String,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl SEEvent for TextMessage {}
impl SERespondableSubscribableIdentifiedObject for TextMessage {}
impl SERespondableResource for TextMessage {}
impl SEResource for TextMessage {}
impl Validate for TextMessage {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TimeTariffInterval {
    #[yaserde(rename = "ConsumptionTariffIntervalListLink")]
    pub consumption_tariff_interval_list_link: Option<ConsumptionTariffIntervalListLink>,

    // Indicates the time of use tier related to the reading. If not specified,
    // is assumed to be "0 - N/A".
    #[yaserde(rename = "touTier")]
    pub tou_tier: Toutype,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval duration, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeDuration")]
    pub randomize_duration: Option<OneHourRangeType>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval start time, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeStart")]
    pub randomize_start: Option<OneHourRangeType>,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl SERandomizableEvent for TimeTariffInterval {}
impl SEEvent for TimeTariffInterval {}
impl SERespondableSubscribableIdentifiedObject for TimeTariffInterval {}
impl SERespondableResource for TimeTariffInterval {}
impl SEResource for TimeTariffInterval {}
impl Validate for TimeTariffInterval {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceControl {
    #[yaserde(rename = "ApplianceLoadReduction")]
    pub appliance_load_reduction: Option<ApplianceLoadReduction>,

    // Specifies the bitmap indicating the categories of devices that SHOULD
    // respond. Devices SHOULD ignore events that do not indicate their device
    // category.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: DeviceCategoryType,

    // A flag to indicate if the EndDeviceControl is considered a mandatory
    // event as defined by the service provider issuing the EndDeviceControl.
    // The drProgramMandatory flag alerts the client/user that they will be
    // subject to penalty or ineligibility based on the service provider’s
    // program rules for that deviceCategory.
    #[yaserde(rename = "drProgramMandatory")]
    pub dr_program_mandatory: bool,

    #[yaserde(rename = "DutyCycle")]
    pub duty_cycle: Option<DutyCycle>,

    // Indicates that the event intends to increase consumption. A value of true
    // indicates the intention to increase usage value, and a value of false
    // indicates the intention to decrease usage.
    #[yaserde(rename = "loadShiftForward")]
    pub load_shift_forward: bool,

    #[yaserde(rename = "Offset")]
    pub offset: Option<Offset>,

    // The overrideDuration attribute provides a duration, in seconds, for which
    // a client device is allowed to override this EndDeviceControl and still
    // meet the contractual agreement with a service provider without opting
    // out. If overrideDuration is not specified, then it SHALL default to 0.
    #[yaserde(rename = "overrideDuration")]
    pub override_duration: Option<Uint16>,

    #[yaserde(rename = "SetPoint")]
    pub set_point: Option<SetPoint>,

    #[yaserde(rename = "TargetReduction")]
    pub target_reduction: Option<TargetReduction>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval duration, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeDuration")]
    pub randomize_duration: Option<OneHourRangeType>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval start time, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeStart")]
    pub randomize_start: Option<OneHourRangeType>,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl SERandomizableEvent for EndDeviceControl {}
impl SEEvent for EndDeviceControl {}
impl SERespondableSubscribableIdentifiedObject for EndDeviceControl {}
impl SERespondableResource for EndDeviceControl {}
impl SEResource for EndDeviceControl {}
impl Validate for EndDeviceControl {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DemandResponseProgram {
    #[yaserde(rename = "ActiveEndDeviceControlListLink")]
    pub active_end_device_control_list_link: Option<ActiveEndDeviceControlListLink>,

    // This attribute allows program providers to specify the requested
    // granularity of updates to LoadShedAvailability sheddablePercent. If not
    // present, or set to 0, then updates to LoadShedAvailability SHALL NOT be
    // provided. If present and greater than zero, then clients SHALL provide
    // their LoadShedAvailability if it has not previously been provided, and
    // thereafter if the difference between the previously provided value and
    // the current value of LoadShedAvailability sheddablePercent is greater
    // than availabilityUpdatePercentChangeThreshold.
    #[yaserde(rename = "availabilityUpdatePercentChangeThreshold")]
    pub availability_update_percent_change_threshold: Option<PerCent>,

    // This attribute allows program providers to specify the requested
    // granularity of updates to LoadShedAvailability sheddablePower. If not
    // present, or set to 0, then updates to LoadShedAvailability SHALL NOT be
    // provided. If present and greater than zero, then clients SHALL provide
    // their LoadShedAvailability if it has not previously been provided, and
    // thereafter if the difference between the previously provided value and
    // the current value of LoadShedAvailability sheddablePower is greater than
    // availabilityUpdatePowerChangeThreshold.
    #[yaserde(rename = "availabilityUpdatePowerChangeThreshold")]
    pub availability_update_power_change_threshold: Option<ActivePower>,

    #[yaserde(rename = "EndDeviceControlListLink")]
    pub end_device_control_list_link: Option<EndDeviceControlListLink>,

    // Indicates the relative primacy of the provider of this program.
    #[yaserde(rename = "primacy")]
    pub primacy: PrimacyType,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

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

impl SEIdentifiedObject for DemandResponseProgram {}
impl SEResource for DemandResponseProgram {}
impl Validate for DemandResponseProgram {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TariffProfile {
    // The currency code indicating the currency for this TariffProfile.
    #[yaserde(rename = "currency")]
    pub currency: Option<CurrencyCode>,

    // Indicates the power of ten multiplier for the price attribute.
    #[yaserde(rename = "pricePowerOfTenMultiplier")]
    pub price_power_of_ten_multiplier: Option<PowerOfTenMultiplierType>,

    // Indicates the relative primacy of the provider of this program.
    #[yaserde(rename = "primacy")]
    pub primacy: PrimacyType,

    // The rate code for this tariff profile. Provided by the Pricing service
    // provider per its internal business needs and practices and provides a
    // method to identify the specific rate code for the TariffProfile instance.
    // This would typically not be communicated to the user except to facilitate
    // troubleshooting due to its service provider-specific technical nature.
    #[yaserde(rename = "rateCode")]
    pub rate_code: Option<String20>,

    #[yaserde(rename = "RateComponentListLink")]
    pub rate_component_list_link: Option<RateComponentListLink>,

    // The kind of service provided by this usage point.
    #[yaserde(rename = "serviceCategoryKind")]
    pub service_category_kind: ServiceKind,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

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

impl SEIdentifiedObject for TariffProfile {}
impl SEResource for TariffProfile {}
impl Validate for TariffProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MessagingProgram {
    #[yaserde(rename = "ActiveTextMessageListLink")]
    pub active_text_message_list_link: Option<ActiveTextMessageListLink>,

    // Indicates the language and region of the messages in this collection.
    #[yaserde(rename = "locale")]
    pub locale: LocaleType,

    // Indicates the relative primacy of the provider of this program.
    #[yaserde(rename = "primacy")]
    pub primacy: PrimacyType,

    #[yaserde(rename = "TextMessageListLink")]
    pub text_message_list_link: Option<TextMessageListLink>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl SESubscribableIdentifiedObject for MessagingProgram {}
impl SESubscribableResource for MessagingProgram {}
impl SEResource for MessagingProgram {}
impl Validate for MessagingProgram {}

#[derive(Default, Clone, Copy, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum PrimacyType {
    #[default]
    InHomeEnergyManagementSystem = 0,
    ContractedPremisesServiceProvider = 1,
    NonContractualServiceProvider = 2,
}

impl Validate for PrimacyType {}