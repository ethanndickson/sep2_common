use yaserde::{YaDeserialize, YaSerialize};

use crate::packages::{
    identification::{ResponseRequired, ResponseStatus},
    objects::EventStatus,
    primitives::{HexBinary16, HexBinary160, Int48, String32, Uint32},
    types::{
        ConsumptionBlockType, DateTimeInterval, DeviceCategoryType, MRIDType, OneHourRangeType,
        RoleFlagsType, SFDIType, ServiceKind, SubscribableType, TimeType, Toutype, VersionType,
    },
    // TODO: Temporary import all
    xsd::*,
};

// All IEEE 2030.5 top-level types are either a Resource, or a Link to a Resource
// Since the spec does not use multiple-inheritance, there is redundancy in the inheritance tree. We may be able to remove this in the future.
// Each of these traits can be derived provided an attribute with the expected type (as per the specification, not these traits) exists.

/// A top-level IEEE 2030.5 Resource.
/// An IEEE 2030.5 Server exposes resources.
/// IEEE 2030.5 Clients retrieve, update, create and delete resources on servers.
/// Implemented by all types whose base type is [`Resource`]
///
/// [`Resource`]: super::identification::Resource
pub trait SEResource:
    YaSerialize + YaDeserialize + Default + PartialEq + Eq + Clone + Validate
{
    fn href(&self) -> Option<&str>;
}

/// An IEEE 2030.5 Representation of a link to a resource.
/// "Links provide a reference, via URI, to another resource."
/// These are not top-level resources.
/// Implemented by all types whose base type is [`Link`]
///
/// [`Link`]: super::identification::Link
pub trait SELink: YaSerialize + YaDeserialize + Default + PartialEq + Eq + Clone {
    fn href(&self) -> &str;
}

pub trait SEListLink: SELink {
    fn all(&self) -> Option<Uint32>;
}

/// Implemented by all types whose base type is [`Response`]
///
/// [`Response`]: super::identification::Response
pub trait SEResponse: SEResource {
    fn created_date_time(&self) -> Option<TimeType>;
    fn end_device_lfdi(&self) -> &HexBinary160;
    fn status(&self) -> Option<ResponseStatus>;
    fn subject(&self) -> &MRIDType;
}

/// Implemented by all types whose base type is [`IdentifiedObject`]
///
/// [`IdentifiedObject`]: super::identification::IdentifiedObject
pub trait SEIdentifiedObject: SEResource {
    fn mrid(&self) -> &MRIDType;
    fn description(&self) -> Option<&String32>;
    fn version(&self) -> Option<VersionType>;
}

/// Implemented by all types whose base type is [`RespondableResource`]
///
/// [`RespondableResource`]: super::identification::RespondableResource
pub trait SERespondableResource: SEResource {
    fn reply_to(&self) -> Option<&str>;
    fn response_required(&self) -> Option<ResponseRequired>;
}

/// Implemented by all types whose base type is [`SubscriptionBase`]
///
/// [`SubscriptionBase`]: super::pubsub::SubscriptionBase
pub trait SESubscriptionBase: SEResource {
    fn subscribed_resource(&self) -> &str;
}

/// Implemented by all types whose base type is [`SubscribableResource`]
///
/// [`SubscribableResource`]: super::identification::SubscribableResource
pub trait SESubscribableResource: SEResource {
    fn subscribable(&self) -> Option<SubscribableType>;
}

/// Implemented by all types whose base type is [`RespondableIdentifiedObject`]
///
/// [`RespondableIdentifiedObject`]: super::identification::RespondableIdentifiedObject
pub trait SERespondableIdentifiedObject: SERespondableResource {
    fn mrid(&self) -> &MRIDType;
    fn description(&self) -> Option<&String32>;
    fn version(&self) -> Option<VersionType>;
}

/// Implemented by all types whose base type is [`RespondableSubscribableIdentifiedObject`]
///
/// [`RespondableSubscribableIdentifiedObject`]: super::identification::RespondableSubscribableIdentifiedObject
pub trait SERespondableSubscribableIdentifiedObject: SERespondableResource {
    fn mrid(&self) -> &MRIDType;
    fn description(&self) -> Option<&String32>;
    fn version(&self) -> Option<VersionType>;
    fn subscribable(&self) -> Option<SubscribableType>;
}

/// Implemented by all types whose base type is [`SubscribableIdentifiedObject`]
///
/// [`SubscribableIdentifiedObject`]: super::identification::SubscribableIdentifiedObject
pub trait SESubscribableIdentifiedObject: SESubscribableResource {
    fn mrid(&self) -> &MRIDType;
    fn description(&self) -> Option<&String32>;
    fn version(&self) -> Option<VersionType>;
}

/// Implemented by all types whose base type is [`Event`]
///
/// [`Event`]: super::objects::Event
pub trait SEEvent: SERespondableSubscribableIdentifiedObject {
    fn creation_time(&self) -> TimeType;
    fn event_status(&self) -> &EventStatus;
    fn interval(&self) -> &DateTimeInterval;
}

/// Implemented by all types whose base type is [`RandomizableEvent`]
///
/// [`RandomizableEvent`]: super::objects::RandomizableEvent
pub trait SERandomizableEvent: SEEvent {
    fn randomize_duration(&self) -> Option<OneHourRangeType>;
    fn randomize_start(&self) -> Option<OneHourRangeType>;
}

/// Implemented by all types whose base type is [`List`]
///
/// [`List`]: super::identification::List
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
/// [`SubscribableList`]: super::identification::SubscribableList
pub trait SESubscribableList: SESubscribableResource {
    fn all(&self) -> Uint32;
    fn results(&self) -> Uint32;
}

/// Implemented by all types whose base type is [`FunctionSetAssignmentsBase`]
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
    fn subscribable(&self) -> Option<&SubscribableType>;
}

/// Implemented by all types whose base type is [`MeterReadingBase`]
pub trait SEMeterReadingBase: SEIdentifiedObject {
    // Does not extend IdentifiedObject further
}

/// Implemented by all types whose base type is [`ReadingBase`]
pub trait SEReadingBase: SEResource {
    fn consumption_block(&self) -> Option<ConsumptionBlockType>;
    fn quality_flags(&self) -> Option<HexBinary16>;
    fn time_period(&self) -> Option<&DateTimeInterval>;
    fn tou_tier(&self) -> Option<Toutype>;
    fn value(&self) -> Option<Int48>;
}

/// Implemented by all types whose base type is [`ReadingSetBase`]
pub trait SEReadingSetBase: SEIdentifiedObject {
    fn time_period(&self) -> &DateTimeInterval;
}

/// Implemented by all types whose base type is [`UsagePointBase`]
pub trait SEUsagePointBase: SEIdentifiedObject {
    fn role_flags(&self) -> RoleFlagsType;
    fn service_category_kind(&self) -> ServiceKind;
}

/// Implemented by all types whose base type is [`SEBillingMeterReadingBase`]
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