use crate::traits::{
    SEIdentifiedObject, SEList, SEMeterReadingBase, SEReadingBase, SEReadingSetBase, SEResource,
    SESubscribableList, SESubscribableResource, SEUsagePointBase, Validate,
};
use sep2_common_derive::{
    SEIdentifiedObject, SEList, SEMeterReadingBase, SEReadingBase, SEReadingSetBase, SEResource,
    SESubscribableList, SESubscribableResource, SEUsagePointBase,
};
use yaserde::{YaDeserialize, YaSerialize};

use super::{
    links::{
        MeterReadingListLink, RateComponentListLink, ReadingLink, ReadingListLink,
        ReadingSetListLink, ReadingTypeLink,
    },
    primitives::{HexBinary16, HexBinary160, Int48, String32, Uint32, Uint48, Uint8},
    types::{
        AccumulationBehaviourType, CommodityType, ConsumptionBlockType, DataQualifierType,
        DateTimeInterval, FlowDirectionType, KindType, MRIDType, PhaseCode,
        PowerOfTenMultiplierType, RoleFlagsType, ServiceKind, SubscribableType, Toutype,
        UnitValueType, UomType, UsagePointStatus, VersionType,
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
    SEMeterReadingBase,
    SEIdentifiedObject,
    SEResource,
)]
#[yaserde(rename = "MeterReading")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MeterReading {
    #[yaserde(rename = "RateComponentListLink")]
    pub rate_component_list_link: Option<RateComponentListLink>,

    #[yaserde(rename = "ReadingLink")]
    pub reading_link: Option<ReadingLink>,

    #[yaserde(rename = "ReadingSetListLink")]
    pub reading_set_list_link: Option<ReadingSetListLink>,

    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: ReadingTypeLink,

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

impl PartialOrd for MeterReading {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MeterReading {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for MeterReading {}

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
#[yaserde(rename = "MeterReadingList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MeterReadingList {
    #[yaserde(rename = "MeterReading")]
    pub meter_reading: Vec<MeterReading>,

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

impl Validate for MeterReadingList {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEReadingBase, SEResource,
)]
#[yaserde(rename = "Reading")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Reading {
    /// The local identifier for this reading within the reading set. localIDs
    /// are assigned in order of creation time. For interval data, this value
    /// SHALL increase with each interval time, and for block/tier readings,
    /// localID SHALL not be specified.
    #[yaserde(rename = "localID")]
    pub local_id: Option<HexBinary16>,

    /// Indicates whether or not subscriptions are supported for this resource,
    /// and whether or not conditional (thresholds) are supported. If not
    /// specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

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
    pub quality_flags: Option<HexBinary16>,

    /// The time interval associated with the reading. If not specified, then
    /// defaults to the intervalLength specified in the associated ReadingType.
    #[yaserde(rename = "timePeriod")]
    pub time_period: Option<DateTimeInterval>,

    /// Indicates the time of use tier related to the reading. REQUIRED if
    /// ReadingType numberOfTouTiers is non-zero. If not specified, is assumed to
    /// be "0 - N/A".
    #[yaserde(rename = "touTier")]
    pub tou_tier: Option<Toutype>,

    /// Value in units specified by ReadingType
    #[yaserde(rename = "value")]
    pub value: Option<Int48>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for Reading {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Reading {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - localID (ascending)
        match self.local_id.cmp(&other.local_id) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - consumptionBlock (ascending)
        match self.consumption_block.cmp(&other.consumption_block) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Tertiary Key - touTier (ascending)
        self.tou_tier.cmp(&other.tou_tier)
    }
}

impl Validate for Reading {}

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
#[yaserde(rename = "ReadingList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingList {
    #[yaserde(rename = "Reading")]
    pub reading: Vec<Reading>,

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

impl Validate for ReadingList {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEReadingSetBase,
    SEIdentifiedObject,
    SEResource,
)]
#[yaserde(rename = "ReadingSet")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingSet {
    #[yaserde(rename = "ReadingListLink")]
    pub reading_list_link: Option<ReadingListLink>,

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

impl PartialOrd for ReadingSet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ReadingSet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - timePeriod.start (descending)
        match self
            .time_period
            .start
            .cmp(&other.time_period.start)
            .reverse()
        {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for ReadingSet {}

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
#[yaserde(rename = "ReadingSetList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingSetList {
    #[yaserde(rename = "ReadingSet")]
    pub reading_set: Vec<ReadingSet>,

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

impl Validate for ReadingSetList {}

#[derive(
    Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, YaSerialize, YaDeserialize, SEResource,
)]
#[yaserde(rename = "ReadingType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingType {
    /// The “accumulation behaviour” indicates how the value is represented
    /// to accumulate over time.
    #[yaserde(rename = "accumulationBehaviour")]
    pub accumulation_behaviour: Option<AccumulationBehaviourType>,

    /// The amount of heat generated when a given mass of fuel is completely
    /// burned. The CalorificValue is used to convert the measured volume or mass
    /// of gas into kWh. The CalorificValue attribute represents the current
    /// active value.
    #[yaserde(rename = "calorificValue")]
    pub calorific_value: Option<UnitValueType>,

    /// Indicates the commodity applicable to this ReadingType.
    #[yaserde(rename = "commodity")]
    pub commodity: Option<CommodityType>,

    /// Accounts for changes in the volume of gas based on temperature and
    /// pressure. The ConversionFactor attribute represents the current active
    /// value. The ConversionFactor is dimensionless. The default value for the
    /// ConversionFactor is 1, which means no conversion is applied. A price
    /// server can advertise a new/different value at any time.
    #[yaserde(rename = "conversionFactor")]
    pub conversion_factor: Option<UnitValueType>,

    /// The data type can be used to describe a salient attribute of the data.
    /// Possible values are average, absolute, and etc.
    #[yaserde(rename = "dataQualifier")]
    pub data_qualifier: Option<DataQualifierType>,

    /// Anything involving current might have a flow direction. Possible values
    /// include forward and reverse.
    #[yaserde(rename = "flowDirection")]
    pub flow_direction: Option<FlowDirectionType>,

    /// Default interval length specified in seconds.
    #[yaserde(rename = "intervalLength")]
    pub interval_length: Option<Uint32>,

    /// Compound class that contains kindCategory and kindIndex
    #[yaserde(rename = "kind")]
    pub kind: Option<KindType>,

    /// To be populated for mirrors of interval data to set the expected number
    /// of intervals per ReadingSet. Servers may discard intervals received that
    /// exceed this number.
    #[yaserde(rename = "maxNumberOfIntervals")]
    pub max_number_of_intervals: Option<Uint8>,

    /// Number of consumption blocks. 0 means not applicable, and is the default
    /// if not specified. The value needs to be at least 1 if any actual prices
    /// are provided.
    #[yaserde(rename = "numberOfConsumptionBlocks")]
    pub number_of_consumption_blocks: Option<Uint8>,

    /// The number of TOU tiers that can be used by any resource configured by
    /// this ReadingType. Servers SHALL populate this value with the largest
    /// touTier value that will <i>ever</i> be used while this ReadingType is in
    /// effect. Servers SHALL set numberOfTouTiers equal to the number of
    /// standard TOU tiers plus the number of CPP tiers that may be used while
    /// this ReadingType is in effect. Servers SHALL specify a value between 0
    /// and 255 (inclusive) for numberOfTouTiers (servers providing flat rate
    /// pricing SHOULD set numberOfTouTiers to 0, as in practice there is no
    /// difference between having no tiers and having one tier).
    #[yaserde(rename = "numberOfTouTiers")]
    pub number_of_tou_tiers: Option<Uint8>,

    /// Contains phase information associated with the type.
    #[yaserde(rename = "phase")]
    pub phase: Option<PhaseCode>,

    /// Indicates the power of ten multiplier applicable to the unit of measure
    /// of this ReadingType.
    #[yaserde(rename = "powerOfTenMultiplier")]
    pub power_of_ten_multiplier: Option<PowerOfTenMultiplierType>,

    /// Default sub-interval length specified in seconds for Readings of
    /// ReadingType. Some demand calculations are done over a number of smaller
    /// intervals. For example, in a rolling demand calculation, the demand value
    /// is defined as the rolling sum of smaller intervals over the
    /// intervalLength. The subintervalLength is the length of the smaller
    /// interval in this calculation. It SHALL be an integral division of the
    /// intervalLength. The number of sub-intervals can be calculated by dividing
    /// the intervalLength by the subintervalLength.
    #[yaserde(rename = "subIntervalLength")]
    pub sub_interval_length: Option<Uint32>,

    /// Reflects the supply limit set in the meter. This value can be compared to
    /// the Reading value to understand if limits are being approached or
    /// exceeded. Units follow the same definition as in this ReadingType.
    #[yaserde(rename = "supplyLimit")]
    pub supply_limit: Option<Uint48>,

    /// Specifies whether or not the consumption blocks are differentiated by
    /// TOUTier or not. Default is false, if not specified.
    /// true = consumption accumulated over individual tiers
    /// false = consumption accumulated over all tiers
    #[yaserde(rename = "tieredConsumptionBlocks")]
    pub tiered_consumption_blocks: Option<bool>,

    /// Indicates the measurement type for the units of measure for the readings
    /// of this type.
    #[yaserde(rename = "uom")]
    pub uom: Option<UomType>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ReadingType {}

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
#[yaserde(rename = "UsagePoint")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UsagePoint {
    /// The LFDI of the source device. This attribute SHALL be present when
    /// mirroring.
    #[yaserde(rename = "deviceLFDI")]
    pub device_lfdi: Option<HexBinary160>,

    #[yaserde(rename = "MeterReadingListLink")]
    pub meter_reading_list_link: Option<MeterReadingListLink>,

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

impl PartialOrd for UsagePoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UsagePoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for UsagePoint {}

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
#[yaserde(rename = "UsagePointList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UsagePointList {
    #[yaserde(rename = "UsagePoint")]
    pub usage_point: Vec<UsagePoint>,

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

impl Validate for UsagePointList {}
