use std::fmt::Display;
// File auto-generated using xsd-parser-rs & IEEE 2030.5 sep-ordered-dep.xsd
// Types should eventually be put in a module corresponding to their package
use anyhow::anyhow;
use bitflags::bitflags;
use std::str::FromStr;
use xsd_macro_utils::{UtilsDefaultSerde, UtilsTupleIo};
use xsd_parser::generator::validator::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

use super::primitives::{
    HexBinary128, Int32, Int48, Int64, String32, String42, Uint16, Uint32, Uint48,
};

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "AccumulationBehaviourType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum AccumulationBehaviourType {
    #[default]
    // (default, if not specified)
    NotApplicable = 0,
    // The sum of the previous billing period values. Note: “Cumulative” is commonly used in conjunction with “demand.” Each demand reset causes the maximum demand value for the present billing period (since the last demand reset) to accumulate as an accumulative total of all maximum demands. So instead of “zeroing” the demand register, a demand reset has the affect of adding the present maximum demand to this accumulating total.
    Cumulative = 3,
    // The difference between the value at the end of the prescribed interval and the beginning of the interval. This is used for incremental interval data.
    // Note: One common application would be for load profile data, another use might be to report the number of events within an interval (such as the number of equipment energizations within the specified period of time.)
    DeltaData = 4,
    // As if a needle is swung out on the meter face to a value to indicate the current value. (Note: An “indicating” value is typically measured over hundreds of milliseconds or greater, or may imply a “pusher” mechanism to capture a value. Compare this to “instantaneous” which is measured over a shorter period of time.)
    Indicating = 6,
    // A form of accumulation which is selective with respect to time.
    // Note : “Summation” could be considered a specialization of “Bulk Quantity” according to the rules of inheritance where “Summation” selectively accumulates pulses over a timing pattern, and “BulkQuantity” accumulates pulses all of the time.
    Summation = 9,
    // Typically measured over the fastest period of time allowed by the definition of the metric (usually milliseconds or tens of milliseconds.) (Note: “Instantaneous” was moved to attribute #3 in 61968-9Ed2 from attribute #1 in 61968-9Ed1.)
    Instantaneous = 12,
    // ALL OTHERS RESERVED
}

impl Validate for AccumulationBehaviourType {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ApplianceLoadReductionType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum ApplianceLoadReductionType {
    #[default]
    // Parameter requesting the appliance to respond by providing a moderate load reduction for the duration of a delay period.  Typically referring to a “non-emergency” event in which appliances can continue operating if already in a load consuming period.
    DelayApplianceLoad = 0,
    // Parameter requesting the appliance to respond by providing an aggressive load reduction for a short time period.  Typically referring to an “emergency/spinning reserve” event in which an appliance should start shedding load if currently in a load consuming period.
    TemporaryApplianceLoadReduction = 1,
    // 2-255 reserved
    // * Full definition of how appliances react when receiving each parameter is document in the EPA document - ENERGY STAR® Program Requirements, Product Specification for Residential Refrigerators and Freezers, Eligibility Criteria 5, Draft 2 Version 5.0.
}

impl Validate for ApplianceLoadReductionType {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "CommodityType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub enum CommodityType {
    #[default]
    NotApplicable = 0, // (default, if not specified)
    ElectricitySecondaryMetered = 1, // metered value (a premises meter is typically on the low voltage, or secondary, side of a service transformer)
    ElectricityPrimaryMetered = 2, // metered value (measured on the high voltage, or primary, side of the service transformer)
    Air = 4,
    NaturalGas = 7,
    Propane = 8,
    PotableWater = 9,
    Steam = 10,
    WasteWater = 11,
    HeatingFluid = 12,
    CoolingFluid = 13,
    // All other values reserved
}

impl Validate for CommodityType {}

#[derive(
    Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, YaSerialize, YaDeserialize,
)]
#[yaserde(rename = "ConsumptionBlockType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum ConsumptionBlockType {
    #[default]
    NotApplicable = 0,
    Block1 = 1,
    Block2 = 2,
    Block3 = 3,
    Block4 = 4,
    Block5 = 5,
    Block6 = 6,
    Block7 = 7,
    Block8 = 8,
    Block9 = 9,
    Block10 = 10,
    Block11 = 11,
    Block12 = 12,
    Block13 = 13,
    Block14 = 14,
    Block15 = 15,
    Block16 = 16,
    // 17-255 RESERVED
}

impl Validate for ConsumptionBlockType {}

/// Follows codes defined in ISO 4217.
/// An exhaustive list is currently out of scope for this project
pub type CurrencyCode = Uint16;

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "DataQualifierType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum DataQualifierType {
    #[default]
    NotApplicable = 0,
    Average = 2,
    Maximum = 8,
    Minimum = 9,
    Normal = 12,
    StandardDeviationOfPopulation = 29,
    StandardDeviationOfSample = 30,
}

impl Validate for DataQualifierType {}

// Interval of date and time.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "DateTimeInterval")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DateTimeInterval {
    // Duration of the interval, in seconds.
    #[yaserde(rename = "duration")]
    pub duration: Uint32,

    // Date and time of the start of the interval.
    #[yaserde(rename = "start")]
    pub start: TimeType,
}

impl Validate for DateTimeInterval {}

bitflags! {
    #[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, UtilsTupleIo, UtilsDefaultSerde)]
    pub struct DeviceCategoryType: u32 { // HexBinary32
        const ProgrammableCommunicatingThermostat = 1;
        const StripHeaters = 2;
        const WaterHeater = 8;
        const PoolPump = 16;
        const Sauna = 32;
        const HotTub = 64;
        const SmartAppliance = 128;
        const IrrigationPump = 256;
        const ManagedCommercialAndIndustrialLoads = 512;
        const SimpleMiscLoads = 1024;
        const ExteriorLighting = 2048;
        const InteriorLighting = 4096;
        const LoadControlSwitch = 8192;
        const EnergyManagementSystem = 16384;
        const SmartEnergyModule = 65536;
        const ElectricVehicle = 262144;
        const VirutalOrMixedDer = 524288;
        const ReciprocatingEngine = 2097152;
        const PhotovoltaicSystem = 8388608;
        const CombinedPvAndStorage = 16777216;
        const OtherGenerationSystem = 33554432;
        const OtherStorageSystem = 67108864;
    }
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, UtilsTupleIo, UtilsDefaultSerde)]
pub struct DstRuleType(u32);

impl DstRuleType {
    const SECONDS_SHIFT: u32 = 0;
    const HOURS_SHIFT: u32 = 12;
    const DAY_OF_WEEK_SHIFT: u32 = 17;
    const DAY_OF_MONTH_SHIFT: u32 = 20;
    const OPERATOR_SHIFT: u32 = 25;
    const MONTH_SHIFT: u32 = 28;

    const SECONDS_MASK: u32 = 0xFFF << Self::SECONDS_SHIFT; // Bits 0-11
    const HOURS_MASK: u32 = 0x1F << Self::HOURS_SHIFT; // Bits 12-16
    const DAY_OF_WEEK_MASK: u32 = 0x7 << Self::DAY_OF_WEEK_SHIFT; // Bits 17-19
    const DAY_OF_MONTH_MASK: u32 = 0x1F << Self::DAY_OF_MONTH_SHIFT; // Bits 20-24
    const OPERATOR_MASK: u32 = 0x7 << Self::OPERATOR_SHIFT; // Bits 25-27
    const MONTH_MASK: u32 = 0xF << Self::MONTH_SHIFT; // Bits 28-31

    /// Create a complete DST Rule
    pub fn new(
        seconds: u32,
        hours: u32,
        day_of_week: u32,
        day_of_month: u32,
        operator: u32,
        month: u32,
    ) -> Self {
        let mut out = DstRuleType(0);
        out.set_seconds(seconds);
        out.set_hours(hours);
        out.set_day_of_week(day_of_week);
        out.set_day_of_month(day_of_month);
        out.set_operator(operator);
        out.set_month(month);
        out
    }

    // Get the seconds (bits 0 - 11)
    pub fn seconds(&self) -> u32 {
        self.0 & Self::SECONDS_MASK
    }

    // Set the seconds (bits 0 - 11)
    pub fn set_seconds(&mut self, seconds: u32) {
        self.0 = (self.0 & !Self::SECONDS_MASK) | (seconds & Self::SECONDS_MASK);
    }

    // Get the hours (bits 12 - 16)
    pub fn hours(&self) -> u32 {
        (self.0 & Self::HOURS_MASK) >> Self::HOURS_SHIFT
    }

    // Set the hours (bits 12 - 16)
    pub fn set_hours(&mut self, hours: u32) {
        self.0 = (self.0 & !Self::HOURS_MASK) | ((hours << Self::HOURS_SHIFT) & Self::HOURS_MASK);
    }

    // Get the day of the week (bits 17 - 19)
    pub fn day_of_week(&self) -> u32 {
        (self.0 & Self::DAY_OF_WEEK_MASK) >> Self::DAY_OF_WEEK_SHIFT
    }

    // Set the day of the week (bits 17 - 19)
    pub fn set_day_of_week(&mut self, day_of_week: u32) {
        self.0 = (self.0 & !Self::DAY_OF_WEEK_MASK)
            | ((day_of_week << Self::DAY_OF_WEEK_SHIFT) & Self::DAY_OF_WEEK_MASK);
    }

    // Get the day of the month (bits 20 - 24)
    pub fn day_of_month(&self) -> u32 {
        (self.0 & Self::DAY_OF_MONTH_MASK) >> Self::DAY_OF_MONTH_SHIFT
    }

    // Set the day of the month (bits 20 - 24)
    pub fn set_day_of_month(&mut self, day_of_month: u32) {
        self.0 = (self.0 & !Self::DAY_OF_MONTH_MASK)
            | ((day_of_month << Self::DAY_OF_MONTH_SHIFT) & Self::DAY_OF_MONTH_MASK);
    }

    // Get the operator (bits 25 - 27)
    pub fn operator(&self) -> u32 {
        (self.0 & Self::OPERATOR_MASK) >> Self::OPERATOR_SHIFT
    }

    // Set the operator (bits 25 - 27)
    pub fn set_operator(&mut self, operator: u32) {
        self.0 = (self.0 & !Self::OPERATOR_MASK)
            | ((operator << Self::OPERATOR_SHIFT) & Self::OPERATOR_MASK);
    }

    // Get the month (bits 28 - 31)
    pub fn month(&self) -> u32 {
        (self.0 & Self::MONTH_MASK) >> Self::MONTH_SHIFT
    }

    // Set the month (bits 28 - 31)
    pub fn set_month(&mut self, month: u32) {
        self.0 = (self.0 & !Self::MONTH_MASK) | ((month << Self::MONTH_SHIFT) & Self::MONTH_MASK);
    }
}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "FlowDirectionType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum FlowDirectionType {
    #[default]
    NotApplicable = 0,
    Forward = 1,  // delivered to customer
    Reverse = 19, // received from customer
}

impl Validate for FlowDirectionType {}

// Specifies a GPS location, expressed in WGS 84 coordinates.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "GPSLocationType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct GpslocationType {
    // Specifies the latitude from equator. -90 (south) to +90 (north) in
    // decimal degrees.
    #[yaserde(rename = "lat")]
    pub lat: String32,

    // Specifies the longitude from Greenwich Meridian. -180 (west) to +180
    // (east) in decimal degrees.
    #[yaserde(rename = "lon")]
    pub lon: String32,
}

impl Validate for GpslocationType {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "KindType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum KindType {
    #[default]
    NotApplicable = 0,
    Currency = 3,
    Demand = 8,
    Energy = 12,
    Power = 37,
}

impl Validate for KindType {}

/// IETF RFC 4646 identifier of a language-region.
/// An exhaustive list is currently out of scope for this project
pub type LocaleType = String42;

pub type Mridtype = HexBinary128;

/// A signed time offset, typically applied to a Time value, expressed in seconds, with range −3600 to 3600
#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, UtilsDefaultSerde)]
pub struct OneHourRangeType(i16);

impl OneHourRangeType {
    pub fn new(val: i16) -> Option<OneHourRangeType> {
        if !(-3600..=3600).contains(&val) {
            None
        } else {
            Some(OneHourRangeType(val))
        }
    }
    pub fn get(&self) -> i16 {
        self.0
    }
}
impl FromStr for OneHourRangeType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .ok()
            .and_then(OneHourRangeType::new)
            .ok_or(anyhow!(
                "OneHourRangeType value must be between -3600 and 3600"
            ))
    }
}

impl Display for OneHourRangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// IANA Private Enterprise Number (PEN)
pub type PENType = Uint32;

/// Used for percentages, specified in hundredths of a percent, 0 to 10 000. (10 000 = 100%)
#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, UtilsDefaultSerde)]
pub struct Percent(u16);

impl Percent {
    pub fn new(val: u16) -> Option<Percent> {
        if val > 10_000 {
            None
        } else {
            Some(Percent(val))
        }
    }
    pub fn get(&self) -> u16 {
        self.0
    }
}
impl FromStr for Percent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .ok()
            .and_then(Percent::new)
            .ok_or(anyhow!("Percent value must be between 0 and 10000"))
    }
}

impl Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "PhaseCode")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum PhaseCode {
    #[default]
    NotApplicable = 0,
    PhaseC = 32,  // and S2
    PhaseCN = 33, // and S2N
    PhaseCA = 40,
    PhaseB = 64,
    PhaseBN = 65,
    PhaseBC = 66,
    PhaseA = 128,  // and S1
    PhaseAN = 129, // and S1N
    PhaseAB = 132,
    PhaseABC = 224,
    // ALL OTHERS RESERVED
}

impl Validate for PhaseCode {}

/// Six digit unsigned decimal integer (0 to 999999).
#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, UtilsDefaultSerde)]
pub struct PINType(u32);

impl PINType {
    pub fn new(val: u32) -> Option<PINType> {
        if val > 999_999 {
            None
        } else {
            Some(PINType(val))
        }
    }
    pub fn get(&self) -> u32 {
        self.0
    }
}
impl FromStr for PINType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .ok()
            .and_then(PINType::new)
            .ok_or(anyhow!("PINType value must be between 0 and 999,999"))
    }
}

impl Display for PINType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// For many variants there is no Internal System of Units designated prefix, and as such the number is used as a name instead.
#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "PowerOfTenMultiplierType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(i8)]
pub enum PowerOfTenMultiplierType {
    Nano = -9,
    NegativeEight = -8,
    NegativeSeven = -7,
    Micro = -6,
    NegativeFive = -5,
    NegativeFour = -4,
    Milli = -3,
    Centi = -2,
    Deci = -1,
    #[default]
    None = 0,
    Deca = 1,
    Hecto = 2,
    Kilo = 3,
    Four = 4,
    Five = 5,
    Mega = 6,
    Seven = 7,
    Eight = 8,
    Giga = 9,
}

#[derive(
    Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, YaSerialize, YaDeserialize,
)]
#[yaserde(rename = "PrimacyType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum PrimacyType {
    #[default]
    InHomeEnergyManagementSystem = 0,
    ContractedPremisesServiceProvider = 1,
    NonContractualServiceProvider = 2,
}

impl Validate for PrimacyType {}

// Real electrical energy
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "RealEnergy")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RealEnergy {
    // Multiplier for 'unit'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value of the energy in Watt-hours. (uom 72)
    #[yaserde(rename = "value")]
    pub value: Uint48,
}

impl Validate for RealEnergy {}

bitflags! {
    #[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, UtilsTupleIo, UtilsDefaultSerde)]
    pub struct RoleFlagsType: u16 { // HexBinary16
        const IsMirror = 1;
        const IsPremiseAggregationPoint = 2;
        const IsPEV = 4;
        const IsDER = 8;
        const IsRevenueQuality = 16;
        const IsDC = 32;
        const IsSubmeter = 64;
    }
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ServiceKind")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum ServiceKind {
    #[default]
    Electricity = 0,
    Gas = 1,
    Water = 2,
    Time = 3,
    Pressure = 4,
    Heat = 5,
    Cooling = 6,
}

impl Validate for ServiceKind {}

/// Unsigned integer, maximum inclusive 687194767359, which is 2^36 - 1 (68,719,476,735), with added check digit.
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, UtilsDefaultSerde)]
pub struct SFDIType(u64);

impl SFDIType {
    pub fn new(val: u64) -> Option<SFDIType> {
        if val > 1_099_511_627_775 {
            None
        } else {
            Some(SFDIType(val))
        }
    }
    pub fn get(&self) -> u64 {
        self.0
    }
}
impl FromStr for SFDIType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .ok()
            .and_then(SFDIType::new)
            .ok_or(anyhow!("SFDIType value must be between -10,000 and 10,000"))
    }
}

impl Display for SFDIType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Used for signed percentages, specified in hundredths of a percent, −10 000 to 10 000. (10 000 = 100%)
#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, UtilsDefaultSerde)]
pub struct SignedPercent(i16);

impl SignedPercent {
    pub fn new(val: i16) -> Option<SignedPercent> {
        if !(-10_000..=10_000).contains(&val) {
            None
        } else {
            Some(SignedPercent(val))
        }
    }
    pub fn get(&self) -> i16 {
        self.0
    }
}
impl FromStr for SignedPercent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().ok().and_then(SignedPercent::new).ok_or(anyhow!(
            "SignedPercent value must be between -10,000 and 10,000"
        ))
    }
}

impl Display for SignedPercent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Real electrical energy, signed.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "SignedRealEnergy")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SignedRealEnergy {
    // Multiplier for 'unit'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value of the energy in Watt-hours. (uom 72)
    #[yaserde(rename = "value")]
    pub value: Int48,
}

impl Validate for SignedRealEnergy {}

// The subscribable values.
// 0 - Resource does not support subscriptions
// 1 - Resource supports non-conditional subscriptions
// 2 - Resource supports conditional subscriptions
// 3 - Resource supports both conditional and non-conditional subscriptions
// All other values reserved.
#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[repr(u8)]
pub enum SubscribableType {
    #[default]
    NoSubscriptionsSupported = 0,
    NonConditionalSubscriptions = 1,
    ConditionalSubscriptions = 2,
    AllSubscriptions = 3,
}

impl Validate for SubscribableType {}

pub type TimeOffsetType = Int32;

pub type TimeType = Int64;

#[derive(
    Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, YaSerialize, YaDeserialize,
)]
#[yaserde(rename = "TOUType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum Toutype {
    #[default]
    NotApplicable = 0,
    TouA = 1,
    TouB = 2,
    TouC = 3,
    TouD = 4,
    TouE = 5,
    TouF = 6,
    TouG = 7,
    TouH = 8,
    TouI = 9,
    TouJ = 10,
    TouK = 11,
    TouL = 12,
    TouM = 13,
    TouN = 14,
    TouO = 15,
}

impl Validate for Toutype {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "UnitType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum UnitType {
    #[default]
    kWh = 0,
    kW = 1,
    Watts = 2,
    CubicMeters = 3,
    CubicFeet = 4,
    USGallons = 5,
    ImperialGallons = 6,
    BTU = 7,
    Liters = 8,
    kPAGauge = 9,
    kPAAbsolute = 10,
    Megajoule = 11,
    Unitless = 12,
}

impl Validate for UnitType {}

// Type for specification of a specific value, with units and power of ten
// multiplier.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "UnitValueType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UnitValueType {
    // Multiplier for 'unit'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Unit in symbol
    #[yaserde(rename = "unit")]
    pub unit: UomType,

    // Value in units specified
    #[yaserde(rename = "value")]
    pub value: Int32,
}

impl Validate for UnitValueType {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "UomType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum UomType {
    #[default]
    NotApplicable = 0,
    Amperes = 5,
    Kelvin = 6,
    DegreesCelsius = 23,
    Voltage = 29,
    Joule = 31,
    Hz = 33,
    W = 38,
    MtrCubed = 42,
    VA = 61,
    VAr = 63,
    CosTheta = 65,
    VSquared = 67,
    ASquared = 69,
    VAh = 71,
    Wh = 72,
    VArh = 73,
    Ah = 106,
    FtCubed = 119,
    FtCubedPerHour = 122,
    MCubedPerHour = 125,
    USGallons = 128,
    UGGallonsPerHour = 129,
    ImperialGallons = 130,
    ImperialGallonsPerHour = 131,
    BTU = 132,
    BTUPerHour = 133,
    Liter = 134,
    LiterPerHour = 137,
    PAGauge = 140,
    PAAbsolute = 155,
    Therm = 169,
}

impl Validate for UomType {}

pub type VersionType = Uint16;
