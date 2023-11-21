use crate::traits::{
    SEIdentifiedObject, SEList, SEMeterReadingBase, SEReadingSetBase, SEResource, SEUsagePointBase,
    Validate,
};
use sep2_common_derive::{
    SEIdentifiedObject, SEList, SEMeterReadingBase, SEReadingSetBase, SEResource, SEUsagePointBase,
};
use yaserde::{YaDeserialize, YaSerialize};

use super::{
    metering::{Reading, ReadingType},
    primitives::{HexBinary160, Int48, String32, Uint32},
    types::{
        ConsumptionBlockType, DateTimeInterval, MRIDType, QualityFlags, RoleFlagsType, ServiceKind,
        TOUType, TimeType, UsagePointStatus, VersionType,
    },
};

#[derive(
    Default,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEMeterReadingBase,
    SEIdentifiedObject,
    SEResource,
)]
#[yaserde(rename = "MirrorMeterReading")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorMeterReading {
    /// The date and time of the last update.
    #[yaserde(rename = "lastUpdateTime")]
    pub last_update_time: Option<TimeType>,

    #[yaserde(rename = "MirrorReadingSet")]
    pub mirror_reading_set: Vec<MirrorReadingSet>,

    /// The date and time of the next planned update.
    #[yaserde(rename = "nextUpdateTime")]
    pub next_update_time: Option<TimeType>,

    #[yaserde(rename = "Reading")]
    pub reading: Option<Reading>,

    #[yaserde(rename = "ReadingType")]
    pub reading_type: Option<ReadingType>,

    /// The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    /// The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    /// Contains the version number of the object. See the type definition for
    /// details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MirrorMeterReading {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "MirrorMeterReadingList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorMeterReadingList {
    #[yaserde(rename = "MirrorMeterReading")]
    pub mirror_meter_reading: Vec<MirrorMeterReading>,

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

impl Validate for MirrorMeterReadingList {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEIdentifiedObject,
    SEResource,
    SEMeterReadingBase,
)]
#[yaserde(rename = "MeterReadingBase")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MeterReadingBase {
    /// The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    /// The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    /// Contains the version number of the object. See the type definition for
    /// details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MeterReadingBase {}

#[derive(
    Default,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEReadingSetBase,
    SEIdentifiedObject,
    SEResource,
)]
#[yaserde(rename = "MirrorReadingSet")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorReadingSet {
    #[yaserde(rename = "Reading")]
    pub reading: Vec<Reading>,

    /// Specifies the time range during which the contained readings were taken.
    #[yaserde(rename = "timePeriod")]
    pub time_period: DateTimeInterval,

    /// The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    /// The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    /// Contains the version number of the object. See the type definition for
    /// details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MirrorReadingSet {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEUsagePointBase,
    SEIdentifiedObject,
    SEResource,
)]
#[yaserde(rename = "MirrorUsagePoint")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorUsagePoint {
    /// The LFDI of the device being mirrored.
    #[yaserde(rename = "deviceLFDI")]
    pub device_lfdi: HexBinary160,

    #[yaserde(rename = "MirrorMeterReading")]
    pub mirror_meter_reading: Vec<MirrorMeterReading>,

    /// POST rate, or how often mirrored data should be POSTed, in seconds. A
    /// client MAY indicate a preferred postRate when POSTing MirrorUsagePoint. A
    /// server MAY add or modify postRate to indicate its preferred posting rate.
    #[yaserde(rename = "postRate")]
    pub post_rate: Option<Uint32>,

    /// Specifies the roles that apply to the usage point.
    #[yaserde(rename = "roleFlags")]
    pub role_flags: RoleFlagsType,

    /// The kind of service provided by this usage point.
    #[yaserde(rename = "serviceCategoryKind")]
    pub service_category_kind: ServiceKind,

    /// Specifies the current status of the service at this usage point.
    /// 0 = off
    /// 1 = on
    #[yaserde(rename = "status")]
    pub status: UsagePointStatus,

    /// The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    /// The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    /// Contains the version number of the object. See the type definition for
    /// details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl MirrorUsagePoint {
    pub fn add_reading(&mut self, reading: MirrorMeterReading) {
        self.mirror_meter_reading.push(reading);
        self.mirror_meter_reading.sort();
    }
}

impl PartialOrd for MirrorUsagePoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MirrorUsagePoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for MirrorUsagePoint {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "MirrorUsagePointList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorUsagePointList {
    #[yaserde(rename = "MirrorUsagePoint")]
    pub mirror_usage_point: Vec<MirrorUsagePoint>,

    /// The default polling rate for this function set (this resource and all
    /// resources below), in seconds. If not specified, a default of 900 seconds
    /// (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    /// this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

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

impl Validate for MirrorUsagePointList {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "ReadingBase")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingBase {
    /// Indicates the consumption block related to the reading. REQUIRED if
    /// ReadingType numberOfConsumptionBlocks is non-zero. If not specified, is
    /// assumed to be "0 - N/A".
    #[yaserde(rename = "consumptionBlock")]
    pub consumption_block: Option<ConsumptionBlockType>,

    /// List of codes indicating the quality of the reading, using specification:
    /// Bit 0 - valid: data that has gone through all required validation checks
    /// and either passed them all or has been verified
    /// Bit 1 - manually edited: Replaced or approved by a human
    /// Bit 2 - estimated using reference day: data value was replaced by a
    /// machine computed value based on analysis of historical data using the
    /// same type of measurement.
    /// Bit 3 - estimated using linear interpolation: data value was computed
    /// using linear interpolation based on the readings before and after it
    /// Bit 4 - questionable: data that has failed one or more checks
    /// Bit 5 - derived: data that has been calculated (using logic or
    /// mathematical operations), not necessarily measured directly
    /// Bit 6 - projected (forecast): data that has been calculated as a
    /// projection or forecast of future readings
    #[yaserde(rename = "qualityFlags")]
    pub quality_flags: Option<QualityFlags>,

    /// The time interval associated with the reading. If not specified, then
    /// defaults to the intervalLength specified in the associated ReadingType.
    #[yaserde(rename = "timePeriod")]
    pub time_period: Option<DateTimeInterval>,

    /// Indicates the time of use tier related to the reading. REQUIRED if
    /// ReadingType numberOfTouTiers is non-zero. If not specified, is assumed to
    /// be "0 - N/A".
    #[yaserde(rename = "touTier")]
    pub tou_tier: Option<TOUType>,

    /// Value in units specified by ReadingType
    #[yaserde(rename = "value")]
    pub value: Option<Int48>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ReadingBase {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "ReadingSetBase")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingSetBase {
    /// Specifies the time range during which the contained readings were taken.
    #[yaserde(rename = "timePeriod")]
    pub time_period: DateTimeInterval,

    /// The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    /// The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    /// Contains the version number of the object. See the type definition for
    /// details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ReadingSetBase {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEIdentifiedObject,
    SEResource,
    SEUsagePointBase,
)]
#[yaserde(rename = "UsagePointBase")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UsagePointBase {
    /// Specifies the roles that apply to the usage point.
    #[yaserde(rename = "roleFlags")]
    pub role_flags: RoleFlagsType,

    /// The kind of service provided by this usage point.
    #[yaserde(rename = "serviceCategoryKind")]
    pub service_category_kind: ServiceKind,

    /// Specifies the current status of the service at this usage point.
    /// 0 = off
    /// 1 = on
    #[yaserde(rename = "status")]
    pub status: UsagePointStatus,

    /// The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    /// The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    /// Contains the version number of the object. See the type definition for
    /// details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for UsagePointBase {}
