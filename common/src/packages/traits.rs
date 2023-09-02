use yaserde::{YaDeserialize, YaSerialize};

use super::{
    identification::ResponseStatus,
    objects::EventStatus,
    primitives::{HexBinary16, HexBinary160, HexBinary8, Int48, String32, Uint32},
    types::{
        ConsumptionBlockType, DateTimeInterval, DeviceCategoryType, Mridtype, OneHourRangeType,
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
pub trait SEResource: YaSerialize + YaDeserialize + Default + PartialEq + Clone {
    fn href(&self) -> Option<&str>;
}

/// An IEEE 2030.5 Representation of a link to a resource.
/// "Links provide a reference, via URI, to another resource."
/// These are not top-level resources.
pub trait SELink: YaSerialize + YaDeserialize + Default + PartialEq + Clone {
    fn href(&self) -> &str;
}

pub trait SEListLink: SELink {
    fn all(&self) -> Option<Uint32>;
}
pub trait SEResponse: SEResource {
    fn created_date_time(&self) -> Option<TimeType>;
    fn end_device_lfdi(&self) -> &HexBinary160;
    fn status(&self) -> Option<ResponseStatus>;
    fn subject(&self) -> &Mridtype;
}

pub trait SEIdentifiedObject: SEResource {
    fn mrid(&self) -> &Mridtype;
    fn description(&self) -> Option<&String32>;
    fn version(&self) -> Option<VersionType>;
}

pub trait SERespondableResource: SEResource {
    fn reply_to(&self) -> Option<&str>;
    fn response_required(&self) -> Option<HexBinary8>;
}

pub trait SESubscriptionBase: SEResource {
    fn subscribed_resource(&self) -> &str;
}

pub trait SESubscribableResource: SEResource {
    fn subscribable(&self) -> Option<SubscribableType>;
}

pub trait SERespondableIdentifiedObject: SERespondableResource {
    fn mrid(&self) -> &Mridtype;
    fn description(&self) -> Option<&String32>;
    fn version(&self) -> Option<VersionType>;
}

pub trait SERespondableSubscribableIdentifiedObject: SERespondableResource {
    fn mrid(&self) -> &Mridtype;
    fn description(&self) -> Option<&String32>;
    fn version(&self) -> Option<VersionType>;
    fn subscribable(&self) -> Option<SubscribableType>;
}

pub trait SESubscribableIdentifiedObject: SESubscribableResource {
    fn mrid(&self) -> &Mridtype;
    fn description(&self) -> Option<&String32>;
    fn version(&self) -> Option<VersionType>;
}

pub trait SEEvent: SERespondableSubscribableIdentifiedObject {
    fn creation_time(&self) -> TimeType;
    fn event_status(&self) -> &EventStatus;
    fn interval(&self) -> &DateTimeInterval;
}

pub trait SERandomizableEvent: SEEvent {
    fn randomize_duration(&self) -> Option<OneHourRangeType>;
    fn randomize_start(&self) -> Option<OneHourRangeType>;
}

pub trait SEList: SEResource {
    fn all(&self) -> Uint32;
    fn results(&self) -> Uint32;
}

pub trait SESubscribableList: SESubscribableResource {
    fn all(&self) -> Uint32;
    fn results(&self) -> Uint32;
}

pub trait SEFunctionSetAssignmentsBase: SEResource {
    fn customer_account_list_link(&self) -> Option<&CustomerAccountListLink>;
    fn demand_response_program_list_link(&self) -> Option<&DemandResponseProgramListLink>;
    fn der_program_list_link(&self) -> Option<&DerprogramListLink>;
    fn file_list_link(&self) -> Option<&FileListLink>;
    fn messaging_program_list_link(&self) -> Option<&MessagingProgramListLink>;
    fn prepayment_list_link(&self) -> Option<&PrepaymentListLink>;
    fn response_set_list_link(&self) -> Option<&ResponseSetListLink>;
    fn tariff_profile_list_link(&self) -> Option<&TariffProfileListLink>;
    fn time_link(&self) -> Option<&TimeLink>;
    fn usage_point_list_link(&self) -> Option<&UsagePointListLink>;
}

pub trait SEAbstractDevice: SESubscribableResource {
    fn configuration_link(&self) -> Option<&ConfigurationLink>;
    fn der_list_link(&self) -> Option<&DerlistLink>;
    fn device_category(&self) -> Option<DeviceCategoryType>;
    fn device_information_link(&self) -> Option<&DeviceInformationLink>;
    fn device_status_link(&self) -> Option<&DeviceStatusLink>;
    fn file_status_link(&self) -> Option<&FileStatusLink>;
    fn ip_interface_list_link(&self) -> Option<&IpinterfaceListLink>;
    fn lfdi(&self) -> Option<&HexBinary160>;
    fn load_shed_availability_list_link(&self) -> Option<&LoadShedAvailabilityListLink>;
    fn log_event_list_link(&self) -> Option<&LogEventListLink>;
    fn power_status_link(&self) -> Option<&PowerStatusLink>;
    fn sfdi(&self) -> SFDIType;
    fn subscribable(&self) -> Option<&SubscribableType>;
}

pub trait SEMeterReadingBase: SEIdentifiedObject {
    // Does not extend IdentifiedObject further
}

pub trait SEReadingBase: SEResource {
    fn consumption_block(&self) -> Option<ConsumptionBlockType>;
    fn quality_flags(&self) -> Option<HexBinary16>;
    fn time_period(&self) -> Option<&DateTimeInterval>;
    fn tou_tier(&self) -> Option<Toutype>;
    fn value(&self) -> Option<Int48>;
}

pub trait SEReadingSetBase: SEIdentifiedObject {
    fn time_period(&self) -> &DateTimeInterval;
}

pub trait SEUsagePointBase: SEIdentifiedObject {
    fn role_flags(&self) -> RoleFlagsType;
    fn service_category_kind(&self) -> ServiceKind;
}

pub trait SEBillingMeterReadingBase: SEMeterReadingBase {
    fn billing_reading_set_list_link(&self) -> Option<&BillingReadingSetListLink>;
    fn reading_type_link(&self) -> Option<&ReadingTypeLink>;
}
