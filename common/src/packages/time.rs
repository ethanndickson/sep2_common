use crate::traits::{SEResource, Validate};
use sep2_common_derive::SEResource;

use yaserde::{YaDeserialize, YaSerialize};

use super::{
    primitives::Uint32,
    types::{TimeOffsetType, TimeType},
};

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "Time")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Time {
    // The current time, in the format defined by TimeType.
    #[yaserde(rename = "currentTime")]
    pub current_time: TimeType,

    // Time at which daylight savings ends (dstOffset no longer applied). Result
    // of dstEndRule calculation.
    #[yaserde(rename = "dstEndTime")]
    pub dst_end_time: TimeType,

    // Daylight savings time offset from local standard time. A typical practice
    // is advancing clocks one hour when daylight savings time is in effect,
    // which would result in a positive dstOffset.
    #[yaserde(rename = "dstOffset")]
    pub dst_offset: TimeOffsetType,

    // Time at which daylight savings begins (apply dstOffset). Result of
    // dstStartRule calculation.
    #[yaserde(rename = "dstStartTime")]
    pub dst_start_time: TimeType,

    // Local time: localTime = currentTime + tzOffset (+ dstOffset when in
    // effect).
    #[yaserde(rename = "localTime")]
    pub local_time: Option<TimeType>,

    // Metric indicating the quality of the time source from which the service
    // acquired time. Lower (smaller) quality enumeration values are assumed to
    // be more accurate.
    // 3 - time obtained from external authoritative source such as NTP
    // 4 - time obtained from level 3 source
    // 5 - time manually set or obtained from level 4 source
    // 6 - time obtained from level 5 source
    // 7 - time intentionally uncoordinated
    // All other values are reserved for future use.
    #[yaserde(rename = "quality")]
    pub quality: TimeQuality,

    // Local time zone offset from currentTime. Does not include any daylight
    // savings time offsets. For American time zones, a negative tzOffset SHALL
    // be used (eg, EST = GMT-5 which is -18000).
    #[yaserde(rename = "tzOffset")]
    pub tz_offset: TimeOffsetType,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Time {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum TimeQuality {
    #[default]
    ExternalAuth = 3,
    LevelThree = 4,
    ManualOrLevelFour = 5,
    LevelFive = 6,
    IntentUncoordinated = 7,
}
