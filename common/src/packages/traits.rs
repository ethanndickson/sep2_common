use yaserde::{YaDeserialize, YaSerialize};

use super::{
    objects::EventStatus,
    primitives::{HexBinary8, String32},
    xsd::{DateTimeInterval, Mridtype, SubscribableType, TimeType, VersionType},
};

// All IEEE 2030.5 top-level types are either a Resource, or a Link to a Resource
pub trait SEResource: YaSerialize + YaDeserialize + Default + PartialEq + Clone {}
pub trait SELink: YaSerialize + YaDeserialize + Default + PartialEq + Clone {}
pub trait SEResponse: SEResource {}
pub trait SEIdentifiedObject: SEResource {}
pub trait SERespondableResource: SEResource {
    fn reply_to(&self) -> Option<&str>;
    fn response_required(&self) -> Option<HexBinary8>;
}
pub trait SESubscriptionBase: SEResource {}
pub trait SESubscribableResource: SEResource {}
pub trait SERespondableIdentifiedObject: SERespondableResource {}
pub trait SERespondableSubscribableIdentifiedObject: SERespondableResource {}
pub trait SESubscribableIdentifiedObject: SESubscribableResource {
    fn mrid(&self) -> &Mridtype;
    fn description(&self) -> Option<&String32>;
    fn version(&self) -> Option<VersionType>;
    fn subscribable(&self) -> Option<SubscribableType>;
}
pub trait SEEvent: SERespondableSubscribableIdentifiedObject {
    fn creation_time(&self) -> TimeType;
    fn event_status(&self) -> &EventStatus;
    fn interval(&self) -> &DateTimeInterval;
}
pub trait SERandomizableEvent: SEEvent {}

pub trait SEListLink: SELink {}

pub trait SEList: SEResource {}

pub trait SESubscribableList: SESubscribableResource {}

pub trait SEFunctionSetAssignmentsBase: SEResource {}

pub trait SEAbstractDevice: SESubscribableResource {}

pub trait SEMeterReadingBase: SEIdentifiedObject {}

pub trait SEReadingBase: SEResource {}

pub trait SEReadingSetBase: SEIdentifiedObject {}

pub trait SEUsagePointBase: SEIdentifiedObject {}

pub trait SEBillingMeterReadingBase: SEMeterReadingBase {}
