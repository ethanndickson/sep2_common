use yaserde::{YaDeserialize, YaSerialize};

pub trait SEResource: YaSerialize + YaDeserialize + Default + PartialEq + Clone {}
pub trait SEResponse: SEResource {}
pub trait SEIdentifiedObject: SEResource {}
pub trait SERespondableResource: SEResource {}
pub trait SESubscriptionBase: SEResource {}
pub trait SESubscribableResource: SEResource {}
pub trait SERespondableIdentifiedObject: SERespondableResource {}
pub trait SERespondableSubscribableIdentifiedObject: SERespondableResource {}
pub trait SESubscribableIdentifiedObject: SESubscribableResource {}
pub trait SEEvent: SERespondableSubscribableIdentifiedObject {}
pub trait SERandomizableEvent: SEEvent {}

pub trait SELink: YaSerialize + YaDeserialize {}
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
