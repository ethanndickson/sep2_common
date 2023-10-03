use crate::traits::{SEList, SEResource, SESubscribableResource, Validate};
use sep2_common_derive::{SEList, SEResource, SESubscribableResource};

use super::{
    identification::{Link, ListLink},
    primitives::{Int32, String32, Uint32},
    types::{DstRuleType, LocaleType, SubscribableType, TimeOffsetType, TimeType},
};
use yaserde::{YaDeserialize, YaSerialize};

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
)]
#[yaserde(rename = "Configuration")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Configuration {
    // [RFC 4646] identifier of the language-region currently in use.
    #[yaserde(rename = "currentLocale")]
    pub current_locale: LocaleType,

    #[yaserde(rename = "PowerConfiguration")]
    pub power_configuration: Option<PowerConfiguration>,

    #[yaserde(rename = "PriceResponseCfgListLink")]
    pub price_response_cfg_list_link: Option<ListLink>,

    #[yaserde(rename = "TimeConfiguration")]
    pub time_configuration: Option<TimeConfiguration>,

    // User assigned, convenience name used for network browsing displays, etc.
    // Example "My Thermostat"
    #[yaserde(rename = "userDeviceName")]
    pub user_device_name: String32,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

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

impl Validate for Configuration {}

// Contains configuration related to the device's power sources
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "PowerConfiguration")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PowerConfiguration {
    // Time/Date at which battery was installed,
    #[yaserde(rename = "batteryInstallTime")]
    pub battery_install_time: Option<TimeType>,

    // In context of the PowerStatus resource, this is the value of
    // EstimatedTimeRemaining below which BatteryStatus "low" is indicated and
    // the PS_LOW_BATTERY is raised.
    #[yaserde(rename = "lowChargeThreshold")]
    pub low_charge_threshold: Option<Uint32>,
}

impl Validate for PowerConfiguration {}

#[derive(
    Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, YaSerialize, YaDeserialize, SEResource,
)]
#[yaserde(rename = "PriceResponseCfg")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PriceResponseCfg {
    // Price responsive clients acting upon the associated RateComponent SHOULD
    // consume the associated commodity while the price is less than this
    // threshold.
    #[yaserde(rename = "consumeThreshold")]
    pub consume_threshold: Int32,

    // Price responsive clients acting upon the associated RateComponent SHOULD
    // reduce consumption to the maximum extent possible while the price is
    // greater than this threshold.
    #[yaserde(rename = "maxReductionThreshold")]
    pub max_reduction_threshold: Int32,

    #[yaserde(rename = "RateComponentLink")]
    pub rate_component_link: Link,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for PriceResponseCfg {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "PriceResponseCfgList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PriceResponseCfgList {
    #[yaserde(rename = "PriceResponseCfg")]
    pub price_response_cfg: Vec<PriceResponseCfg>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for PriceResponseCfgList {}

// Contains attributes related to the configuration of the time service.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "TimeConfiguration")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TimeConfiguration {
    // Rule to calculate end of daylight savings time in the current year.
    // Result of dstEndRule must be greater than result of dstStartRule.
    #[yaserde(rename = "dstEndRule")]
    pub dst_end_rule: DstRuleType,

    // Daylight savings time offset from local standard time.
    #[yaserde(rename = "dstOffset")]
    pub dst_offset: TimeOffsetType,

    // Rule to calculate start of daylight savings time in the current year.
    // Result of dstEndRule must be greater than result of dstStartRule.
    #[yaserde(rename = "dstStartRule")]
    pub dst_start_rule: DstRuleType,

    // Local time zone offset from UTCTime. Does not include any daylight
    // savings time offsets.
    #[yaserde(rename = "tzOffset")]
    pub tz_offset: TimeOffsetType,
}

impl Validate for TimeConfiguration {}
