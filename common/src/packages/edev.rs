use crate::traits::{
    SEAbstractDevice, SEList, SEResource, SESubscribableList, SESubscribableResource, Validate,
};
use sep2_common_derive::{
    SEAbstractDevice, SEList, SEResource, SESubscribableList, SESubscribableResource,
};

use yaserde::{YaDeserialize, YaSerialize};

#[cfg(feature = "conn_point")]
use super::conn_point::ConnectionPointLink;

use super::{
    links::{
        ConfigurationLink, DERListLink, DeviceInformationLink, DeviceStatusLink, FileStatusLink,
        FlowReservationRequestListLink, FlowReservationResponseListLink,
        FunctionSetAssignmentsListLink, IPInterfaceListLink, LoadShedAvailabilityListLink,
        LogEventListLink, PowerStatusLink, RegistrationLink, SubscriptionListLink, TimeLink,
    },
    primitives::{HexBinary160, Int16, Uint16, Uint32, Uint8},
    types::{
        DeviceCategoryType, PINType, PowerOfTenMultiplierType, SFDIType, SubscribableType, TimeType,
    },
};

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SESubscribableResource,
    SEResource,
    SEAbstractDevice,
)]
#[yaserde(rename = "AbstractDevice")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AbstractDevice {
    #[yaserde(rename = "ConfigurationLink")]
    pub configuration_link: Option<ConfigurationLink>,

    #[yaserde(rename = "DERListLink")]
    pub der_list_link: Option<DERListLink>,

    /// This field is for use in devices that can adjust energy usage (e.g.,
    /// demand response, distributed energy resources). For devices that do not
    /// respond to EndDeviceControls or DERControls (for instance, an ESI), this
    /// field should not have any bits set.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: Option<DeviceCategoryType>,

    #[yaserde(rename = "DeviceInformationLink")]
    pub device_information_link: Option<DeviceInformationLink>,

    #[yaserde(rename = "DeviceStatusLink")]
    pub device_status_link: Option<DeviceStatusLink>,

    #[yaserde(rename = "FileStatusLink")]
    pub file_status_link: Option<FileStatusLink>,

    #[yaserde(rename = "IPInterfaceListLink")]
    pub ip_interface_list_link: Option<IPInterfaceListLink>,

    /// Long form of device identifier. See the Security section for additional
    /// details.
    #[yaserde(rename = "lFDI")]
    pub lfdi: Option<HexBinary160>,

    #[yaserde(rename = "LoadShedAvailabilityListLink")]
    pub load_shed_availability_list_link: Option<LoadShedAvailabilityListLink>,

    #[yaserde(rename = "LogEventListLink")]
    pub log_event_list_link: Option<LogEventListLink>,

    #[yaserde(rename = "PowerStatusLink")]
    pub power_status_link: Option<PowerStatusLink>,

    /// Short form of device identifier, WITH the checksum digit. See the
    /// Security section for additional details.
    #[yaserde(rename = "sFDI")]
    pub sfdi: SFDIType,

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

impl Validate for AbstractDevice {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "DeviceStatus")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceStatus {
    /// The time at which the reported values were recorded.
    #[yaserde(rename = "changedTime")]
    pub changed_time: TimeType,

    /// The number of times that the device has been turned on: Count of "device
    /// on" times, since the last time the counter was reset
    #[yaserde(rename = "onCount")]
    pub on_count: Option<Uint16>,

    /// Device operational state:
    /// 0 - Not applicable / Unknown
    /// 1 - Not operating
    /// 2 - Operating
    /// 3 - Starting up
    /// 4 - Shutting down
    /// 5 - At disconnect level
    /// 6 - kW ramping
    /// 7 - kVar ramping
    #[yaserde(rename = "opState")]
    pub op_state: Option<OpState>,

    /// Total time device has operated: re-settable: Accumulated time in seconds
    /// since the last time the counter was reset.
    #[yaserde(rename = "opTime")]
    pub op_time: Option<Uint32>,

    #[yaserde(rename = "Temperature")]
    pub temperature: Vec<Temperature>,

    #[yaserde(rename = "TimeLink")]
    pub time_link: Option<TimeLink>,

    /// The default polling rate for this function set (this resource and all
    /// resources below), in seconds. If not specified, a default of 900 seconds
    /// (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    /// this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DeviceStatus {}

#[allow(non_camel_case_types)]
#[derive(
    Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, YaSerialize, YaDeserialize,
)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum OpState {
    #[default]
    /// Or Unknown
    NotApplicable = 0,
    NotOperating = 1,
    Operating = 2,
    StartingUp = 3,
    ShuttingDown = 4,
    AtDisconnectLevel = 5,
    kWRamping = 6,
    kVarRamping = 7,
}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEAbstractDevice,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "EndDevice")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDevice {
    /// The time at which this resource was last modified or created.
    #[yaserde(rename = "changedTime")]
    pub changed_time: TimeType,

    /// This attribute indicates whether or not an EndDevice is enabled, or
    /// registered, on the server. If a server sets this attribute to false, the
    /// device is no longer registered. It should be noted that servers can
    /// delete EndDevice instances, but using this attribute for some time is
    /// more convenient for clients.
    #[yaserde(rename = "enabled")]
    pub enabled: Option<bool>,

    #[yaserde(rename = "FlowReservationRequestListLink")]
    pub flow_reservation_request_list_link: Option<FlowReservationRequestListLink>,

    #[yaserde(rename = "FlowReservationResponseListLink")]
    pub flow_reservation_response_list_link: Option<FlowReservationResponseListLink>,

    #[yaserde(rename = "FunctionSetAssignmentsListLink")]
    pub function_set_assignments_list_link: Option<FunctionSetAssignmentsListLink>,

    /// POST rate, or how often EndDevice and subordinate resources should be
    /// POSTed, in seconds. A client MAY indicate a preferred postRate when
    /// POSTing EndDevice. A server MAY add or modify postRate to indicate its
    /// preferred posting rate.
    #[yaserde(rename = "postRate")]
    pub post_rate: Option<Uint32>,

    #[yaserde(rename = "RegistrationLink")]
    pub registration_link: Option<RegistrationLink>,

    #[yaserde(rename = "SubscriptionListLink")]
    pub subscription_list_link: Option<SubscriptionListLink>,

    #[yaserde(rename = "ConfigurationLink")]
    pub configuration_link: Option<ConfigurationLink>,

    #[yaserde(rename = "DERListLink")]
    pub der_list_link: Option<DERListLink>,

    /// This field is for use in devices that can adjust energy usage (e.g.,
    /// demand response, distributed energy resources). For devices that do not
    /// respond to EndDeviceControls or DERControls (for instance, an ESI), this
    /// field should not have any bits set.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: Option<DeviceCategoryType>,

    #[yaserde(rename = "DeviceInformationLink")]
    pub device_information_link: Option<DeviceInformationLink>,

    #[yaserde(rename = "DeviceStatusLink")]
    pub device_status_link: Option<DeviceStatusLink>,

    #[yaserde(rename = "FileStatusLink")]
    pub file_status_link: Option<FileStatusLink>,

    #[yaserde(rename = "IPInterfaceListLink")]
    pub ip_interface_list_link: Option<IPInterfaceListLink>,

    /// Long form of device identifier. See the Security section for additional
    /// details.
    #[yaserde(rename = "lFDI")]
    pub lfdi: Option<HexBinary160>,

    #[yaserde(rename = "LoadShedAvailabilityListLink")]
    pub load_shed_availability_list_link: Option<LoadShedAvailabilityListLink>,

    #[yaserde(rename = "LogEventListLink")]
    pub log_event_list_link: Option<LogEventListLink>,

    #[yaserde(rename = "PowerStatusLink")]
    pub power_status_link: Option<PowerStatusLink>,

    #[yaserde(prefix = "csipaus", namespace = "csipaus: https://csipaus.org/ns")]
    #[yaserde(rename = "ConnectionPointLink ")]
    #[cfg(feature = "conn_point")]
    pub connection_point_link: Option<ConnectionPointLink>,

    /// Short form of device identifier, WITH the checksum digit. See the
    /// Security section for additional details.
    #[yaserde(rename = "sFDI")]
    pub sfdi: SFDIType,

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

impl PartialOrd for EndDevice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EndDevice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - changedTime (descending)
        match self.changed_time.cmp(&other.changed_time).reverse() {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - SFDI (ascending)
        match self.sfdi.cmp(&other.sfdi) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Tertiary Key - href (ascending)
        self.href.cmp(&other.href)
    }
}

impl Validate for EndDevice {}

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
#[yaserde(rename = "EndDeviceList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceList {
    #[yaserde(rename = "EndDevice")]
    pub end_device: Vec<EndDevice>,

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

impl Validate for EndDeviceList {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "Registration")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Registration {
    /// Contains the time at which this registration was created, by which
    /// clients MAY prioritize information providers with the most recent
    /// registrations, when no additional direction from the consumer is
    /// available.
    #[yaserde(rename = "dateTimeRegistered")]
    pub date_time_registered: TimeType,

    /// Contains the registration PIN number associated with the device,
    /// including the checksum digit.
    #[yaserde(rename = "pIN")]
    pub pin: PINType,

    /// The default polling rate for this function set (this resource and all
    /// resources below), in seconds. If not specified, a default of 900 seconds
    /// (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    /// this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Registration {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEAbstractDevice,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "SelfDevice")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SelfDevice {
    /// The default polling rate for this function set (this resource and all
    /// resources below), in seconds. If not specified, a default of 900 seconds
    /// (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    /// this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    #[yaserde(rename = "ConfigurationLink")]
    pub configuration_link: Option<ConfigurationLink>,

    #[yaserde(rename = "DERListLink")]
    pub der_list_link: Option<DERListLink>,

    /// This field is for use in devices that can adjust energy usage (e.g.,
    /// demand response, distributed energy resources). For devices that do not
    /// respond to EndDeviceControls or DERControls (for instance, an ESI), this
    /// field should not have any bits set.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: Option<DeviceCategoryType>,

    #[yaserde(rename = "DeviceInformationLink")]
    pub device_information_link: Option<DeviceInformationLink>,

    #[yaserde(rename = "DeviceStatusLink")]
    pub device_status_link: Option<DeviceStatusLink>,

    #[yaserde(rename = "FileStatusLink")]
    pub file_status_link: Option<FileStatusLink>,

    #[yaserde(rename = "IPInterfaceListLink")]
    pub ip_interface_list_link: Option<IPInterfaceListLink>,

    /// Long form of device identifier. See the Security section for additional
    /// details.
    #[yaserde(rename = "lFDI")]
    pub lfdi: Option<HexBinary160>,

    #[yaserde(rename = "LoadShedAvailabilityListLink")]
    pub load_shed_availability_list_link: Option<LoadShedAvailabilityListLink>,

    #[yaserde(rename = "LogEventListLink")]
    pub log_event_list_link: Option<LogEventListLink>,

    #[yaserde(rename = "PowerStatusLink")]
    pub power_status_link: Option<PowerStatusLink>,

    /// Short form of device identifier, WITH the checksum digit. See the
    /// Security section for additional details.
    #[yaserde(rename = "sFDI")]
    pub sfdi: SFDIType,

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

impl Validate for SelfDevice {}

/// Specification of a temperature.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "Temperature")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Temperature {
    /// Multiplier for 'unit'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    /// The subject of the temperature measurement
    /// 0 - Enclosure
    /// 1 - Transformer
    /// 2 - HeatSink
    #[yaserde(rename = "subject")]
    pub subject: Uint8,

    /// Value in Degrees Celsius (uom 23).
    #[yaserde(rename = "value")]
    pub value: Int16,
}

impl Validate for Temperature {}
