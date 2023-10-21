//! Module of traits representing IEEE 2030.5 types used as base types.
//!
//! All IEEE 2030.5 top-level types are either a Resource.
//! Since the spec does not use multiple-inheritance, there is redundancy in the inheritance tree. We have this removed this redundancy for clarity & usability.
//! Each of these traits can be derived provided an attribute with the expected type (as per the specification, not these traits) exists.
use std::panic::RefUnwindSafe;

use yaserde::{YaDeserialize, YaSerialize};

use crate::packages::{
    identification::{ResponseRequired, ResponseStatus},
    objects::EventStatus,
    primitives::{HexBinary160, String32, Uint32},
    types::{
        DateTimeInterval, MRIDType, OneHourRangeType, SubscribableType, TimeType, VersionType,
    },
};

#[cfg(feature = "metering_mirror")]
use crate::packages::{
    primitives::{HexBinary16, Int48},
    types::{ConsumptionBlockType, RoleFlagsType, ServiceKind, TOUType},
};

#[cfg(feature = "billing")]
use crate::packages::links::{BillingReadingSetListLink, ReadingTypeLink};

#[cfg(feature = "fsa")]
use crate::packages::links::{
    CustomerAccountListLink, DERProgramListLink, DemandResponseProgramListLink, FileListLink,
    MessagingProgramListLink, PrepaymentListLink, ResponseSetListLink, TariffProfileListLink,
    TimeLink, UsagePointListLink,
};

#[cfg(feature = "edev")]
use crate::packages::{
    links::{
        ConfigurationLink, DERListLink, DeviceInformationLink, DeviceStatusLink, FileStatusLink,
        IPInterfaceListLink, LoadShedAvailabilityListLink, LogEventListLink, PowerStatusLink,
    },
    types::{DeviceCategoryType, SFDIType},
};

/// Supertrait for all base traits; implemented by all IEEE 2030.5 types present in the XSD
pub trait SEType:
    YaSerialize + YaDeserialize + Validate + RefUnwindSafe + Send + Sync + 'static
{
}

impl<T> SEType for T where
    T: YaSerialize + YaDeserialize + Validate + RefUnwindSafe + Send + Sync + 'static
{
}

/// A top-level IEEE 2030.5 Resource.
/// An IEEE 2030.5 Server exposes resources.
/// IEEE 2030.5 Clients retrieve, update, create and delete resources on servers.
/// Implemented by all types whose base type is [`Resource`]
///
/// The `RefUnwindSafe` auto-trait bound is to gracefully handle our XML library (xml-rs) unavoidably panicking and performing a stack unwind.
/// There is no reason for any Resource to use interior mutability, thus this bound is reasonable.
///
/// [`Resource`]: super::packages::identification::Resource
pub trait SEResource: SEType {
    fn href(&self) -> Option<&str>;
}

/// Implemented by all types whose base type is [`Response`]
///
/// [`Response`]: super::packages::identification::Response
pub trait SEResponse: SEResource {
    /// Create a Response.
    ///
    /// DRLC Responses [`DrResponse`] contain additional fields,
    /// which default to `None` when created via this function.
    fn new(
        creation_time: TimeType,
        lfdi: HexBinary160,
        mrid: MRIDType,
        status: ResponseStatus,
    ) -> Self;
    fn created_date_time(&self) -> Option<TimeType>;
    fn end_device_lfdi(&self) -> &HexBinary160;
    fn status(&self) -> Option<ResponseStatus>;
    fn subject(&self) -> &MRIDType;
}

/// Implemented by all types whose base type is [`IdentifiedObject`]
///
/// [`IdentifiedObject`]: super::packages::identification::IdentifiedObject
pub trait SEIdentifiedObject: SEResource {
    fn mrid(&self) -> &MRIDType;
    fn description(&self) -> Option<&String32>;
    fn version(&self) -> Option<VersionType>;
}

/// Implemented by all types whose base type is [`RespondableResource`]
///
/// [`RespondableResource`]: super::packages::identification::RespondableResource
pub trait SERespondableResource: SEResource {
    fn reply_to(&self) -> Option<&str>;
    fn response_required(&self) -> Option<ResponseRequired>;
}

/// Implemented by all types whose base type is [`SubscriptionBase`]
///
/// [`SubscriptionBase`]: super::packages::pubsub::SubscriptionBase
#[cfg(feature = "pubsub")]
pub trait SESubscriptionBase: SEResource {
    fn subscribed_resource(&self) -> &str;
}

/// Implemented by all types whose base type is [`SubscribableResource`]
///
/// [`SubscribableResource`]: super::packages::identification::SubscribableResource
pub trait SESubscribableResource: SEResource {
    fn subscribable(&self) -> Option<SubscribableType>;
}

/// Implemented by all types whose base type is [`RespondableIdentifiedObject`]
///
/// [`RespondableIdentifiedObject`]: super::packages::identification::RespondableIdentifiedObject
pub trait SERespondableIdentifiedObject: SERespondableResource + SEIdentifiedObject {}

/// Implemented by all types whose base type is [`RespondableSubscribableIdentifiedObject`]
///
/// [`RespondableSubscribableIdentifiedObject`]: super::packages::identification::RespondableSubscribableIdentifiedObject
pub trait SERespondableSubscribableIdentifiedObject:
    SERespondableResource + SESubscribableResource + SEIdentifiedObject
{
}

/// Implemented by all types whose base type is [`SubscribableIdentifiedObject`]
///
/// [`SubscribableIdentifiedObject`]: super::packages::identification::SubscribableIdentifiedObject
pub trait SESubscribableIdentifiedObject: SESubscribableResource + SEIdentifiedObject {}

/// Implemented by all types whose base type is [`Event`]
///
/// [`Event`]: super::packages::objects::Event
pub trait SEEvent: SERespondableSubscribableIdentifiedObject {
    fn creation_time(&self) -> TimeType;
    fn event_status(&self) -> &EventStatus;
    fn interval(&self) -> &DateTimeInterval;
}

/// Implemented by all types whose base type is [`RandomizableEvent`]
///
/// [`RandomizableEvent`]: super::packages::objects::RandomizableEvent
pub trait SERandomizableEvent: SEEvent {
    fn randomize_duration(&self) -> Option<OneHourRangeType>;
    fn randomize_start(&self) -> Option<OneHourRangeType>;
}

/// Implemented by all types whose base type is [`List`]
///
/// [`List`]: super::packages::identification::List
pub trait SEList: SEResource {
    type Inner: Ord;
    fn all(&self) -> Uint32;
    fn all_mut(&mut self) -> &mut Uint32;
    fn results(&self) -> Uint32;
    fn results_mut(&mut self) -> &mut Uint32;
    fn list_as_slice(&self) -> &[Self::Inner];
    fn list_mut(&mut self) -> &mut Vec<Self::Inner>;

    /// Add an item to the contained list, maintaining invariants
    fn push(&mut self, item: Self::Inner) {
        self.list_mut().push(item);
        self.list_mut().sort();
        *self.all_mut() = Uint32(self.all().get() + 1);
    }
    ///Remove an item from the contained list, maintaining invariants
    fn remove(&mut self, idx: usize) -> Self::Inner {
        *self.all_mut() = Uint32(self.all().get() - 1);
        self.list_mut().remove(idx)
    }
}

/// Implemented by all types whose base type is [`SubscribableList`]
///
/// [`SubscribableList`]: super::packages::identification::SubscribableList
pub trait SESubscribableList: SESubscribableResource + SEList {}

/// Implemented by all types whose base type is [`FunctionSetAssignmentsBase`]
#[cfg(feature = "fsa")]
pub trait SEFunctionSetAssignmentsBase: SEResource {
    fn customer_account_list_link(&self) -> Option<&CustomerAccountListLink>;
    fn demand_response_program_list_link(&self) -> Option<&DemandResponseProgramListLink>;
    fn der_program_list_link(&self) -> Option<&DERProgramListLink>;
    fn file_list_link(&self) -> Option<&FileListLink>;
    fn messaging_program_list_link(&self) -> Option<&MessagingProgramListLink>;
    fn prepayment_list_link(&self) -> Option<&PrepaymentListLink>;
    fn response_set_list_link(&self) -> Option<&ResponseSetListLink>;
    fn tariff_profile_list_link(&self) -> Option<&TariffProfileListLink>;
    fn time_link(&self) -> Option<&TimeLink>;
    fn usage_point_list_link(&self) -> Option<&UsagePointListLink>;
}

/// Implemented by all types whose base type is [`AbstractDevice`]
#[cfg(feature = "edev")]
pub trait SEAbstractDevice: SESubscribableResource {
    fn configuration_link(&self) -> Option<&ConfigurationLink>;
    fn der_list_link(&self) -> Option<&DERListLink>;
    fn device_category(&self) -> Option<DeviceCategoryType>;
    fn device_information_link(&self) -> Option<&DeviceInformationLink>;
    fn device_status_link(&self) -> Option<&DeviceStatusLink>;
    fn file_status_link(&self) -> Option<&FileStatusLink>;
    fn ip_interface_list_link(&self) -> Option<&IPInterfaceListLink>;
    fn lfdi(&self) -> Option<&HexBinary160>;
    fn load_shed_availability_list_link(&self) -> Option<&LoadShedAvailabilityListLink>;
    fn log_event_list_link(&self) -> Option<&LogEventListLink>;
    fn power_status_link(&self) -> Option<&PowerStatusLink>;
    fn sfdi(&self) -> SFDIType;
}

/// Implemented by all types whose base type is [`MeterReadingBase`]
#[cfg(feature = "metering_mirror")]
pub trait SEMeterReadingBase: SEIdentifiedObject {
    // Does not extend IdentifiedObject further
}

/// Implemented by all types whose base type is [`ReadingBase`]
#[cfg(feature = "metering_mirror")]
pub trait SEReadingBase: SEResource {
    fn consumption_block(&self) -> Option<ConsumptionBlockType>;
    fn quality_flags(&self) -> Option<HexBinary16>;
    fn time_period(&self) -> Option<&DateTimeInterval>;
    fn tou_tier(&self) -> Option<TOUType>;
    fn value(&self) -> Option<Int48>;
}

/// Implemented by all types whose base type is [`ReadingSetBase`]
#[cfg(feature = "metering_mirror")]
pub trait SEReadingSetBase: SEIdentifiedObject {
    fn time_period(&self) -> &DateTimeInterval;
}

/// Implemented by all types whose base type is [`UsagePointBase`]
#[cfg(feature = "metering_mirror")]
pub trait SEUsagePointBase: SEIdentifiedObject {
    fn role_flags(&self) -> RoleFlagsType;
    fn service_category_kind(&self) -> ServiceKind;
}

/// Implemented by all types whose base type is [`SEBillingMeterReadingBase`]
#[cfg(feature = "billing")]
pub trait SEBillingMeterReadingBase: SEMeterReadingBase {
    fn billing_reading_set_list_link(&self) -> Option<&BillingReadingSetListLink>;
    fn reading_type_link(&self) -> Option<&ReadingTypeLink>;
}

/// A trait that may be used in future to check invariants on IEEE 2030.5 data types
pub trait Validate {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
