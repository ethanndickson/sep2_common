use crate::traits::{SEList, SEResource, Validate};
use sep2_common_derive::{SEList, SEResource};

use super::{
    der::ActivePower,
    links::SupportedLocaleListLink,
    power_status::PowerSourceType,
    primitives::{HexBinary160, String32, Uint32},
    types::{GPSLocationType, LocaleType, PENType, RealEnergy, TimeType},
};
use bitflags::bitflags;
use yaserde::{HexBinaryYaSerde, YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "DeviceInformation")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceInformation {
    #[yaserde(rename = "DRLCCapabilities")]
    pub drlc_capabilities: Option<DRLCCapabilities>,

    /// Bitmap indicating the function sets used by the device as a client.
    /// 0 - Device Capability
    /// 1 - Self Device Resource
    /// 2 - End Device Resource
    /// 3 - Function Set Assignments
    /// 4 - Subscription/Notification Mechanism
    /// 5 - Response
    /// 6 - Time
    /// 7 - Device Information
    /// 8 - Power Status
    /// 9 - Network Status
    /// 10 - Log Event
    /// 11 - Configuration Resource
    /// 12 - Software Download
    /// 13 - DRLC
    /// 14 - Metering
    /// 15 - Pricing
    /// 16 - Messaging
    /// 17 - Billing
    /// 18 - Prepayment
    /// 19 - Flow Reservation
    /// 20 - DER Control
    #[yaserde(rename = "functionsImplemented")]
    pub functions_implemented: Option<FunctionSetsImplemented>,

    /// GPS location of this device.
    #[yaserde(rename = "gpsLocation")]
    pub gps_location: Option<GPSLocationType>,

    /// Long form device identifier. See the Security section for full details.
    #[yaserde(rename = "lFDI")]
    pub lfdi: HexBinary160,

    /// Date/time of manufacture
    #[yaserde(rename = "mfDate")]
    pub mf_date: TimeType,

    /// Manufacturer hardware version
    #[yaserde(rename = "mfHwVer")]
    pub mf_hw_ver: String32,

    /// The manufacturer's IANA Enterprise Number.
    #[yaserde(rename = "mfID")]
    pub mf_id: PENType,

    /// Manufacturer dependent information related to the manufacture of this
    /// device
    #[yaserde(rename = "mfInfo")]
    pub mf_info: Option<String32>,

    /// Manufacturer's model number
    #[yaserde(rename = "mfModel")]
    pub mf_model: String32,

    /// Manufacturer assigned serial number
    #[yaserde(rename = "mfSerNum")]
    pub mf_ser_num: String32,

    // Primary source of power.
    #[yaserde(rename = "primaryPower")]
    pub primary_power: PowerSourceType,

    // Secondary source of power
    #[yaserde(rename = "secondaryPower")]
    pub secondary_power: PowerSourceType,

    #[yaserde(rename = "SupportedLocaleListLink")]
    pub supported_locale_list_link: Option<SupportedLocaleListLink>,

    /// Activation date/time of currently running software
    #[yaserde(rename = "swActTime")]
    pub sw_act_time: TimeType,

    /// Currently running software version
    #[yaserde(rename = "swVer")]
    pub sw_ver: String32,

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

bitflags! {
    #[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, HexBinaryYaSerde)]
    pub struct FunctionSetsImplemented: u64 { // HexBinary64 (for some reason)
        const DeviceCapability = 1;
        const SelfDeviceResource = 2;
        const EndDeviceResource = 4;
        const FunctionSetAssignments = 8;
        const SubscriptionNotificationMechanism = 16;
        const Response = 32;
        const Time = 64;
        const DeviceInformation = 128;
        const PowerStatus = 256;
        const NetworkStatus = 512;
        const LogEvent = 1024;
        const ConfigurationResource = 2048;
        const SoftwareDownload = 4096;
        const DRLC = 8192;
        const Metering = 16384;
        const Pricing = 32768;
        const Messaging = 65536;
        const Billing = 131072;
        const Prepayment = 262144;
        const FlowReservation = 524288;
        const DERControl = 1048576;
    }
}

impl Validate for DeviceInformation {}

/// Contains information about the static capabilities of the device, to allow
/// service providers to know what types of functions are supported, what the
/// normal operating ranges and limits are, and other similar information, in
/// order to provide better suggestions of applicable programs to receive the
/// maximum benefit.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "DRLCCapabilities")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DRLCCapabilities {
    /// The average hourly energy usage when in normal operating mode.
    #[yaserde(rename = "averageEnergy")]
    pub average_energy: RealEnergy,

    /// The maximum demand rating of this end device.
    #[yaserde(rename = "maxDemand")]
    pub max_demand: ActivePower,

    /// Bitmap indicating the DRLC options implemented by the device.
    /// 0 - Target reduction (kWh)
    /// 1 - Target reduction (kW)
    /// 2 - Target reduction (Watts)
    /// 3 - Target reduction (Cubic Meters)
    /// 4 - Target reduction (Cubic Feet)
    /// 5 - Target reduction (US Gallons)
    /// 6 - Target reduction (Imperial Gallons)
    /// 7 - Target reduction (BTUs)
    /// 8 - Target reduction (Liters)
    /// 9 - Target reduction (kPA (gauge))
    /// 10 - Target reduction (kPA (absolute))
    /// 11 - Target reduction (Mega Joule)
    /// 12 - Target reduction (Unitless)
    /// 13-15 - Reserved
    /// 16 - Temperature set point
    /// 17 - Temperature offset
    /// 18 - Duty cycle
    /// 19 - Load adjustment percentage
    /// 20 - Appliance load reduction
    /// 21-31 - Reserved
    #[yaserde(rename = "optionsImplemented")]
    pub options_implemented: DRLCOptions,
}

bitflags! {
    #[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, HexBinaryYaSerde)]
    pub struct DRLCOptions: u32 { // HexBinary32
        const TargetReductionKwh = 1;
        const TargetReductionKw = 2;
        const TargetReductionWatts = 4;
        const TargetReductionCubicMeters = 8;
        const TargetReductionCubicFeet = 16;
        const TargetReductionUsGallons = 32;
        const TargetReductionImperialGallons = 64;
        const TargetReductionBtus = 128;
        const TargetReductionLiters = 256;
        const TargetReductionKpaGauge = 512;
        const TargetReductionKpaAbsolute = 1024;
        const TargetReductionMegaJoule = 2048;
        const TargetReductionUnitless = 4096;
        const TemperatureSetPoint = 65536;
        const TemperatureOffset = 131072;
        const DutyCycle = 262144;
        const LoadAdjustmentPercentage = 524288;
        const ApplianceLoadReduction = 1048576;
    }
}

impl Validate for DRLCCapabilities {}

#[derive(
    Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, YaSerialize, YaDeserialize, SEResource,
)]
#[yaserde(rename = "SupportedLocale")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupportedLocale {
    /// The code for a locale that is supported
    #[yaserde(rename = "locale")]
    pub locale: LocaleType,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for SupportedLocale {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "SupportedLocaleList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupportedLocaleList {
    #[yaserde(rename = "SupportedLocale")]
    pub supported_locale: Vec<SupportedLocale>,

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

impl Validate for SupportedLocaleList {}
