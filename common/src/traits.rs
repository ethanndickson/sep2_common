use std::panic::RefUnwindSafe;

use yaserde::{YaDeserialize, YaSerialize};

use crate::packages::{
    identification::{ResponseRequired, ResponseStatus},
    links::{
        BillingReadingSetListLink, ConfigurationLink, CustomerAccountListLink, DERListLink,
        DERProgramListLink, DemandResponseProgramListLink, DeviceInformationLink, DeviceStatusLink,
        FileListLink, FileStatusLink, IPInterfaceListLink, LoadShedAvailabilityListLink,
        LogEventListLink, MessagingProgramListLink, PowerStatusLink, PrepaymentListLink,
        ReadingTypeLink, ResponseSetListLink, TariffProfileListLink, TimeLink, UsagePointListLink,
    },
    objects::EventStatus,
    primitives::{HexBinary16, HexBinary160, Int48, String32, Uint32},
    types::{
        ConsumptionBlockType, DateTimeInterval, DeviceCategoryType, MRIDType, OneHourRangeType,
        RoleFlagsType, SFDIType, ServiceKind, SubscribableType, TimeType, Toutype, VersionType,
    },
};

// All IEEE 2030.5 top-level types are either a Resource, or a Link to a Resource
// Since the spec does not use multiple-inheritance, there is redundancy in the inheritance tree. We have this removed this redundancy for clarity & usability.
// Each of these traits can be derived provided an attribute with the expected type (as per the specification, not these traits) exists.

/// A top-level IEEE 2030.5 Resource.
/// An IEEE 2030.5 Server exposes resources.
/// IEEE 2030.5 Clients retrieve, update, create and delete resources on servers.
/// Implemented by all types whose base type is [`Resource`]
///
/// The `RefUnwindSafe` auto-trait bound is to gracefully handle our XML library (xml-rs) unavoidably panicking and performing a stack unwind.
/// There is no reason for any Resource to use interior mutability, thus this bound is reasonable.
///
/// [`Resource`]: super::packages::identification::Resource
#[cfg(feature = "common")]
pub trait SEResource:
    YaSerialize + YaDeserialize + Default + PartialEq + Eq + Clone + Validate + RefUnwindSafe
{
    fn href(&self) -> Option<&str>;
}

/// An IEEE 2030.5 Representation of a link to a resource.
/// "Links provide a reference, via URI, to another resource."
/// These are not top-level resources.
/// Implemented by all types whose base type is [`Link`]
///
/// [`Link`]: super::packages::identification::Link
#[cfg(feature = "common")]
pub trait SELink:
    YaSerialize + YaDeserialize + Default + PartialEq + Eq + Clone + Validate
{
    fn href(&self) -> &str;
}

#[cfg(feature = "common")]
pub trait SEListLink: SELink {
    fn all(&self) -> Option<Uint32>;
}

/// Implemented by all types whose base type is [`Response`]
///
/// [`Response`]: super::packages::identification::Response
#[cfg(feature = "common")]
pub trait SEResponse: SEResource {
    fn created_date_time(&self) -> Option<TimeType>;
    fn end_device_lfdi(&self) -> &HexBinary160;
    fn status(&self) -> Option<ResponseStatus>;
    fn subject(&self) -> &MRIDType;
}

/// Implemented by all types whose base type is [`IdentifiedObject`]
///
/// [`IdentifiedObject`]: super::packages::identification::IdentifiedObject
#[cfg(feature = "common")]
pub trait SEIdentifiedObject: SEResource {
    fn mrid(&self) -> &MRIDType;
    fn description(&self) -> Option<&String32>;
    fn version(&self) -> Option<VersionType>;
}

/// Implemented by all types whose base type is [`RespondableResource`]
///
/// [`RespondableResource`]: super::packages::identification::RespondableResource
#[cfg(feature = "common")]
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
#[cfg(feature = "common")]
pub trait SESubscribableResource: SEResource {
    fn subscribable(&self) -> Option<SubscribableType>;
}

/// Implemented by all types whose base type is [`RespondableIdentifiedObject`]
///
/// [`RespondableIdentifiedObject`]: super::packages::identification::RespondableIdentifiedObject
#[cfg(feature = "common")]
pub trait SERespondableIdentifiedObject: SERespondableResource + SEIdentifiedObject {}

/// Implemented by all types whose base type is [`RespondableSubscribableIdentifiedObject`]
///
/// [`RespondableSubscribableIdentifiedObject`]: super::packages::identification::RespondableSubscribableIdentifiedObject
#[cfg(feature = "common")]
pub trait SERespondableSubscribableIdentifiedObject:
    SERespondableResource + SESubscribableResource + SEIdentifiedObject
{
}

/// Implemented by all types whose base type is [`SubscribableIdentifiedObject`]
///
/// [`SubscribableIdentifiedObject`]: super::packages::identification::SubscribableIdentifiedObject
#[cfg(feature = "common")]
pub trait SESubscribableIdentifiedObject: SESubscribableResource + SEIdentifiedObject {}

/// Implemented by all types whose base type is [`Event`]
///
/// [`Event`]: super::packages::objects::Event
#[cfg(feature = "common")]
pub trait SEEvent: SERespondableSubscribableIdentifiedObject {
    fn creation_time(&self) -> TimeType;
    fn event_status(&self) -> &EventStatus;
    fn interval(&self) -> &DateTimeInterval;
}

/// Implemented by all types whose base type is [`RandomizableEvent`]
///
/// [`RandomizableEvent`]: super::packages::objects::RandomizableEvent
#[cfg(feature = "common")]
pub trait SERandomizableEvent: SEEvent {
    fn randomize_duration(&self) -> Option<OneHourRangeType>;
    fn randomize_start(&self) -> Option<OneHourRangeType>;
}

/// Implemented by all types whose base type is [`List`]
///
/// [`List`]: super::packages::identification::List
#[cfg(feature = "common")]
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
    // Remove an item from the contained list, maintaining invariants
    fn remove(&mut self, idx: usize) -> Self::Inner {
        *self.all_mut() = Uint32(self.all().get() - 1);
        self.list_mut().remove(idx)
    }
}

/// Implemented by all types whose base type is [`SubscribableList`]
///
/// [`SubscribableList`]: super::packages::identification::SubscribableList
#[cfg(feature = "common")]
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
#[cfg(feature = "edev")]
pub trait SEMeterReadingBase: SEIdentifiedObject {
    // Does not extend IdentifiedObject further
}

/// Implemented by all types whose base type is [`ReadingBase`]
#[cfg(feature = "metering_mirror")]
pub trait SEReadingBase: SEResource {
    fn consumption_block(&self) -> Option<ConsumptionBlockType>;
    fn quality_flags(&self) -> Option<HexBinary16>;
    fn time_period(&self) -> Option<&DateTimeInterval>;
    fn tou_tier(&self) -> Option<Toutype>;
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
