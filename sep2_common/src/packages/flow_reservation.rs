use crate::traits::{
    SEEvent, SEIdentifiedObject, SEList, SEResource, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SESubscribableList, SESubscribableResource,
    Validate,
};
use sep2_common_derive::{
    SEEvent, SEIdentifiedObject, SEList, SEResource, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SESubscribableList, SESubscribableResource,
};

use super::{
    der::ActivePower,
    identification::ResponseRequired,
    objects::EventStatus,
    primitives::{String32, Uint16, Uint32},
    types::{
        DateTimeInterval, MRIDType, SignedRealEnergy, SubscribableType, TimeType, VersionType,
    },
};
use sepserde::{YaDeserialize, YaSerialize};

/// The RequestStatus object is used to indicate the current status of a Flow
/// Reservation Request.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "RequestStatus")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RequestStatus {
    /// The dateTime attribute will provide a timestamp of when the request
    /// status was set. dateTime MUST be set to the time at which the status
    /// change occurred, not a time in the future or past.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    /// Field representing the request status type.
    /// 0 = Requested
    /// 1 = Cancelled
    /// All other values reserved.
    #[yaserde(rename = "requestStatus")]
    pub request_status: RequestStatusType,
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum RequestStatusType {
    #[default]
    Requested = 0,
    Cancelled = 1,
}

impl Validate for RequestStatus {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "FlowReservationRequest")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationRequest {
    /// The time at which the request was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    /// A value that is calculated by the storage device that defines the minimum
    /// duration, in seconds, that it will take to complete the actual flow
    /// transaction, including any ramp times and conditioning times, if
    /// applicable.
    #[yaserde(rename = "durationRequested")]
    pub duration_requested: Option<Uint16>,

    /// Indicates the total amount of energy, in Watt-Hours, requested to be
    /// transferred between the storage device and the electric power system.
    /// Positive values indicate charging and negative values indicate
    /// discharging. This sign convention is different than for the DER function
    /// where discharging is positive. Note that the energyRequestNow attribute
    /// in the PowerStatus Object must always represent a charging solution and
    /// it is not allowed to have a negative value.
    #[yaserde(rename = "energyRequested")]
    pub energy_requested: SignedRealEnergy,

    /// The time window during which the flow reservation is needed. For example,
    /// if an electric vehicle is set with a 7:00 AM time charge is needed, and
    /// price drops to the lowest tier at 11:00 PM, then this window would likely
    /// be from 11:00 PM until 7:00 AM.
    #[yaserde(rename = "intervalRequested")]
    pub interval_requested: DateTimeInterval,

    /// Indicates the sustained level of power, in Watts, that is requested. For
    /// charging this is calculated by the storage device and it represents the
    /// charging system capability (which for an electric vehicle must also
    /// account for any power limitations due to the EVSE control pilot). For
    /// discharging, a lower value than the inverter capability can be used as a
    /// target.
    #[yaserde(rename = "powerRequested")]
    pub power_requested: ActivePower,

    #[yaserde(rename = "RequestStatus")]
    pub request_status: RequestStatus,

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

impl PartialOrd for FlowReservationRequest {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FlowReservationRequest {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - interval.start (ascending)
        match self
            .interval_requested
            .start
            .cmp(&other.interval_requested.start)
        {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - creationTime (descending)
        match self.creation_time.cmp(&other.creation_time).reverse() {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Tertiary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for FlowReservationRequest {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "FlowReservationRequestList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationRequestList {
    #[yaserde(rename = "FlowReservationRequest")]
    pub flow_reservation_request: Vec<FlowReservationRequest>,

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

impl Validate for FlowReservationRequestList {}

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
#[yaserde(rename = "FlowReservationResponse")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationResponse {
    /// Indicates the amount of energy available.
    #[yaserde(rename = "energyAvailable")]
    pub energy_available: SignedRealEnergy,

    /// Indicates the amount of power available.
    #[yaserde(rename = "powerAvailable")]
    pub power_available: ActivePower,

    /// The subject field provides a method to match the response with the
    /// originating event. It is populated with the mRID of the corresponding
    /// FlowReservationRequest object.
    #[yaserde(rename = "subject")]
    pub subject: MRIDType,

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

impl PartialOrd for FlowReservationResponse {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FlowReservationResponse {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - interval.start (ascending)
        match self.interval.start.cmp(&other.interval.start) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - creationTime (descending)
        match self.creation_time.cmp(&other.creation_time).reverse() {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Tertiary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for FlowReservationResponse {}

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
#[yaserde(rename = "FlowReservationResponseList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationResponseList {
    #[yaserde(rename = "FlowReservationResponse")]
    pub flow_reservation_response: Vec<FlowReservationResponse>,

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

impl Validate for FlowReservationResponseList {}
