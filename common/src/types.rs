/*
* Described in detail in section B.2.3.4.
* Specifically, it contains type aliases for
* data types used for various function sets
* (e.g. TimeType is a 64-bit integer or "Int64")
*/
use super::primitives::*;
use bitflags::bitflags;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

// Nice to do's:
/*
 * DstRuleType p166
 * GPSLocationType.lat and GPSLocationType.lon constructer that does error checking p167
 */
pub type DstRuleType = HexBinary32;

pub type LocaleType = String42;
pub type mRIDType = HexBinary128;
pub type PENType = UInt32;
pub type SFDIType = UInt40;
pub type TimeOffsetType = Int32;
pub type TimeType = Int64;
pub type VersionType = UInt16;

pub struct DateTimeInterval {
    duration: UInt16,
    start: TimeType,
}

pub struct GPSLocationType {
    lat: String32,
    lon: String32,
}

pub struct RealEnergy {
    multiplier: PowerOfTenMultiplierType,
    value: UInt48,
}

pub struct SignedRealEnergy {
    multiplier: PowerOfTenMultiplierType,
    value: Int48,
}

// for this and all the other tuple structs, the new() method should be used
// If only there was a way to enforce it as the only form of construction
// Optional Optimisation find way to enforce new() method for construction
#[derive(Debug)]
pub struct OneHourRangeType(Int16);

impl OneHourRangeType {
    fn new(value: Int16) -> Result<OneHourRangeType, Error> {
        if value < -3600 || value > 3600 {
            Err(Error::from(ErrorKind::InvalidInput))
        } else {
            Ok(OneHourRangeType(value))
        }
    }
}

impl fmt::Display for OneHourRangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let OneHourRangeType(a) = self;
        write!(f, "{}", a)
    }
}

// measured in 1/100ths of a percent (eg. PerCent(102) is 1.02%)
#[derive(Debug)]
pub struct PerCent(UInt16);

impl PerCent {
    fn new(value: UInt16) -> Result<PerCent, Error> {
        if value > 10000 {
            Err(Error::from(ErrorKind::InvalidInput))
        } else {
            Ok(PerCent(value))
        }
    }
}

impl fmt::Display for PerCent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PerCent(a) = self;
        write!(f, "{:.2}%", *a as f32 / 100.00)
    }
}

#[derive(Debug)]
pub struct SignedPerCent(Int16);

impl SignedPerCent {
    fn new(value: Int16) -> Result<SignedPerCent, Error> {
        if value < -10000 || value > 10000 {
            Err(Error::from(ErrorKind::InvalidInput))
        } else {
            Ok(SignedPerCent(value))
        }
    }
}

impl fmt::Display for SignedPerCent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let SignedPerCent(a) = self;
        write!(f, "{:.2}%", *a as f32 / 100.00)
    }
}

#[derive(Debug)]
pub struct PINType(UInt32);

impl PINType {
    fn new(value: UInt32) -> Result<PINType, Error> {
        if value > 999999 {
            Err(Error::from(ErrorKind::InvalidInput))
        } else {
            Ok(PINType(value))
        }
    }
}

impl fmt::Display for PINType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PINType(a) = self;
        write!(f, "{}", a)
    }
}

#[derive(Debug, Default)]
pub struct PowerOfTenMultiplierType(Int8);

impl PowerOfTenMultiplierType {
    fn new(value: Int8) -> Result<PowerOfTenMultiplierType, Error> {
        if value < -9 || value > 9 {
            Err(Error::from(ErrorKind::InvalidInput))
        } else {
            Ok(PowerOfTenMultiplierType(value))
        }
    }
}

impl fmt::Display for PowerOfTenMultiplierType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PowerOfTenMultiplierType(a) = self;
        write!(f, "x {}", a)
    }
}

/*
 * Values possible for indication of “Primary” provider:
 * 0 = In-home energy management system
 * 1 = Contracted premises service provider
 * 2 = Non-contractual service provider
 * 3 to 64 -= Reserved
 * 65 to 191 = User-defined
 */
// this implementation isn't particularly flexible. Luckily, this is extra and not
// particularly requried.
// Optional Optimisation make this better if you can.
#[derive(Debug)]
#[repr(u8)]
pub enum PrimacyType {
    HEMS,
    CPSP,
    NCSP,
    User(u8),
}

impl PrimacyType {
    pub fn new(value: u8) -> Result<PrimacyType, Error> {
        if (value > 2 && value < 65) || value > 191 {
            Err(Error::from(ErrorKind::InvalidInput))
        } else {
            match value {
                0 => Ok(PrimacyType::HEMS),
                1 => Ok(PrimacyType::CPSP),
                2 => Ok(PrimacyType::NCSP),
                _ => Ok(PrimacyType::User(value)),
            }
        }
    }

    pub fn num_value(&self) -> u8 {
        match self {
            PrimacyType::HEMS => 0,
            PrimacyType::CPSP => 1,
            PrimacyType::NCSP => 2,
            PrimacyType::User(value) => *value,
        }
    }

    pub fn format_meaning(&self) -> String {
        match self {
            PrimacyType::HEMS => format!("In-home energy management system: 0"),
            PrimacyType::CPSP => format!("Contracted premises service provider: 1"),
            PrimacyType::NCSP => format!("Non-contractual service provider: 2"),
            PrimacyType::User(value) => format!("User-defined: {}", value as &u8),
        }
    }

    pub fn print_meaning(&self) {
        match self {
            PrimacyType::HEMS => println!("In-home energy management system: 0"),
            PrimacyType::CPSP => println!("Contracted premises service provider: 1"),
            PrimacyType::NCSP => println!("Non-contractual service provider: 2"),
            PrimacyType::User(value) => println!("User-defined: {}", value as &u8),
        }
    }
}

pub struct UnitValueType {
    multiplier: PowerOfTenMultiplierType,
    unit: UomType,
    value: Int48,
}

#[repr(u8)]
pub enum AccumulationBehaviourType {
    NotApplicable = 0,
    Cumulative = 3,
    DeltaData = 4,
    Indicating = 6,
    Summation = 9,
    Instantaneous = 12,
}

impl Default for AccumulationBehaviourType {
    fn default() -> Self {
        AccumulationBehaviourType::NotApplicable
    }
}

// p162
#[repr(u8)]
pub enum ApplianceLoadReductionType {
    DelayApplianceLoad = 0,
    TemporaryApplianceLoadReduction = 1,
}

#[repr(u8)]
pub enum CommodityType {
    NotApplicable = 0,
    ElectricitySecondaryMetered = 1,
    ElectricityPrimaryMetered = 2,
    Air = 4,
    NaturalGas = 7,
    Propane = 8,
    PotableWater = 9,
    Steam = 10,
    WasteWater = 11,
    HeatingFluid = 12,
    CoolingFluid = 13,
}

impl Default for CommodityType {
    fn default() -> Self {
        CommodityType::NotApplicable
    }
}

#[repr(u8)]
pub enum ConsumptionBlockType {
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
}

impl Default for ConsumptionBlockType {
    fn default() -> Self {
        ConsumptionBlockType::NotApplicable
    }
}

// p164
#[repr(u8)]
pub enum CurrencyCode {
    NotApplicable = 0,
    TemporaryApplianceLoadReduction = 1,
}

impl Default for CurrencyCode {
    fn default() -> Self {
        CurrencyCode::NotApplicable
    }
}

#[repr(u8)]
pub enum DataQualifierType {
    NotApplicable = 0,
    Average = 2,
    Maximum = 8,
    Minimum = 9,
    Normal = 12,
    StandardDeviationOfPopulation = 29,
    StandardDeviationOfSample = 30,
}

impl Default for DataQualifierType {
    fn default() -> Self {
        DataQualifierType::NotApplicable
    }
}

#[repr(u8)]
pub enum FlowDirectionType {
    NotApplicable = 0,
    Forward = 1,
    Reverse = 19,
}

impl Default for FlowDirectionType {
    fn default() -> Self {
        FlowDirectionType::NotApplicable
    }
}

#[repr(u8)]
pub enum KindType {
    NotApplicable = 0,
    Currency = 3,
    Demand = 8,
    Energy = 12,
    Power = 37,
}

impl Default for KindType {
    fn default() -> Self {
        KindType::NotApplicable
    }
}

#[repr(u8)]
pub enum PhaseCode {
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
}

impl Default for PhaseCode {
    fn default() -> Self {
        PhaseCode::NotApplicable
    }
}

#[repr(u8)]
pub enum ServiceKind {
    Electricity = 0,
    Gas = 1,
    Water = 2,
    Time = 3,
    Pressure = 4,
    Heat = 5,
    Cooling = 6,
}

// «XSDsimpleType»
#[derive(Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum SubscribableType {
    NoSubscriptionsSupported = 0,
    NonConditionalSubscriptions = 1,
    ConditionalSubscriptions = 2,
    AllSubscriptions = 3,
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum TOUType {
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

impl Default for TOUType {
    fn default() -> Self {
        TOUType::NotApplicable
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum UnitType {
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

#[derive(Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum UomType {
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

impl Default for UomType {
    fn default() -> Self {
        UomType::NotApplicable
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RoleFlagsType: UInt16 {
        const IsMirror = 1;
        const IsPremiseAggregationPoint = 2;
        const IsPEV = 4;
        const IsDER = 8;
        const IsRevenueQuality = 16;
        const IsDC = 32;
        const IsSubmeter = 64;
    }
}

bitflags! {
    pub struct DeviceCategoryType: UInt32 {
        const ProgrammableCommunicatingThermostat = 1;
        const StripHeaters = 2;
        const BaseboardHeaters = 4;
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
        const Energy_managementSystem = 16384;
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

bitflags! {
    pub struct DERControlType: UInt32 {
        const ChargeMode = 1;
        const DischargeMode = 2;
        const OpModConnect = 4;
        const OpModEnergize = 8;
        const OpModFixedPFAbsorbW = 16;
        const OpModFixedPFInjectW = 32;
        const OpModFixedVar = 64;
        const OpModFixedW = 128;
        const OpModFreqDroop = 256;
        const OpModFreqWatt = 512;
        const OpModHFRTMayTrip = 1024;
        const OpModHFRTMustTrip = 2048;
        const OpModHVRTMayTrip = 4096;
        const OpModHVRTMomentaryCessation = 8192;
        const OpModHVRTMustTrip = 16384;
        const OpModLFRTMayTrip = 32768;
    }
}
