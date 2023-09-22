use crate::traits::{
    SEIdentifiedObject, SEList, SEResource, SESubscribableList, SESubscribableResource, Validate,
};
use sep2_common_derive::{
    SEIdentifiedObject, SEList, SEResource, SESubscribableList, SESubscribableResource,
};
use yaserde::{YaDeserialize, YaSerialize};

use super::{
    links::{
        ActiveTimeTariffIntervalListLink, RateComponentListLink, ReadingTypeLink,
        TimeTariffIntervalListLink,
    },
    objects::TimeTariffInterval,
    primitives::{Int32, String20, String32, Uint32, Uint48, Uint8},
    types::{
        ConsumptionBlockType, CurrencyCode, MRIDType, PowerOfTenMultiplierType, PrimacyType,
        RoleFlagsType, ServiceKind, SubscribableType, UnitValueType, VersionType,
    },
};

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "ConsumptionTariffInterval")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ConsumptionTariffInterval {
    // Indicates the consumption block related to the reading. If not specified,
    // is assumed to be "0 - N/A".
    #[yaserde(rename = "consumptionBlock")]
    pub consumption_block: ConsumptionBlockType,

    #[yaserde(rename = "EnvironmentalCost")]
    pub environmental_cost: Vec<EnvironmentalCost>,

    // The charge for this rate component, per unit of measure defined by the
    // associated ReadingType, in currency specified in TariffProfile.
    // The Pricing service provider determines the appropriate price attribute
    // value based on its applicable regulatory rules. For example, price could
    // be net or inclusive of applicable taxes, fees, or levies.
    // The Billing function set provides the ability to represent billing
    // information in a more detailed manner.
    #[yaserde(rename = "price")]
    pub price: Option<Int32>,

    // The lowest level of consumption that defines the starting point of this
    // consumption step or block. Thresholds start at zero for each billing
    // period.
    // If specified, the first ConsumptionTariffInterval.startValue for a
    // TimeTariffInteral instance SHALL begin at "0." Subsequent
    // ConsumptionTariffInterval.startValue elements SHALL be greater than the
    // previous one.
    #[yaserde(rename = "startValue")]
    pub start_value: Uint48,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for ConsumptionTariffInterval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ConsumptionTariffInterval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - startValue (ascending)
        self.start_value.cmp(&other.start_value)
    }
}

impl Validate for ConsumptionTariffInterval {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "ConsumptionTariffIntervalList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ConsumptionTariffIntervalList {
    #[yaserde(rename = "ConsumptionTariffInterval")]
    pub consumption_tariff_interval: Vec<ConsumptionTariffInterval>,

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

impl Validate for ConsumptionTariffIntervalList {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "CostKindType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum CostKindType {
    #[default]
    // Carbon Dioxide emissions, in grams per unit
    CarbonDioxide = 0,
    // Sulfur Dioxide emissions, in grams per unit
    SulfurDioxide = 1,
    // Nitrogen Oxides emissions, in grams per unit
    NitrogenOxide = 2,
    // Renewable generation, as a percentage of overall generation
    RenewablePercentage = 3,
    // 4-255 RESERVED
}

impl Validate for CostKindType {}

// Provides alternative or secondary price information for the relevant
// RateComponent. Supports jurisdictions that seek to convey the environmental
// price per unit of the specified commodity not expressed in currency.
// Implementers and consumers can use this attribute to prioritize operations of
// their HAN devices (e.g., PEV charging during times of high availability of
// renewable electricity resources).
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "EnvironmentalCost")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EnvironmentalCost {
    // The estimated or actual environmental or other cost, per commodity unit
    // defined by the ReadingType, for this RateComponent (e.g., grams of carbon
    // dioxide emissions each per kWh).
    #[yaserde(rename = "amount")]
    pub amount: Uint32,

    // The kind of cost referred to in the amount.
    #[yaserde(rename = "costKind")]
    pub cost_kind: CostKindType,

    // The relative level of the amount attribute. In conjunction with
    // numCostLevels, this attribute informs a device of the relative scarcity
    // of the amount attribute (e.g., a high or low availability of renewable
    // generation).
    // numCostLevels and costLevel values SHALL ascend in order of scarcity,
    // where "0" signals the lowest relative cost and higher values signal
    // increasing cost. For example, if numCostLevels is equal to “3,” then
    // if the lowest relative costLevel were equal to “0,” devices would
    // assume this is the lowest relative period to operate. Likewise, if the
    // costLevel in the next TimeTariffInterval instance is equal to “1,”
    // then the device would assume it is relatively more expensive, in
    // environmental terms, to operate during this TimeTariffInterval instance
    // than the previous one.
    // There is no limit to the number of relative price levels other than that
    // indicated in the attribute type, but for practicality, service providers
    // should strive for simplicity and recognize the diminishing returns
    // derived from increasing the numCostLevel value greater than four.
    #[yaserde(rename = "costLevel")]
    pub cost_level: Uint8,

    // The number of all relative cost levels.
    // In conjunction with costLevel, numCostLevels signals the relative
    // scarcity of the commodity for the duration of the TimeTariffInterval
    // instance (e.g., a relative indication of cost). This is useful in
    // providing context for nominal cost signals to consumers or devices that
    // might see a range of amount values from different service providres or
    // from the same service provider.
    #[yaserde(rename = "numCostLevels")]
    pub num_cost_levels: Uint8,
}

impl Validate for EnvironmentalCost {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "RateComponent")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RateComponent {
    #[yaserde(rename = "ActiveTimeTariffIntervalListLink")]
    pub active_time_tariff_interval_list_link: Option<ActiveTimeTariffIntervalListLink>,

    // Specifies the maximum flow rate (e.g. kW for electricity) for which this
    // RateComponent applies, for the usage point and given rate / tariff.
    // In combination with flowRateStartLimit, allows a service provider to
    // define the demand or output characteristics for the particular tariff
    // design. If a server includes the flowRateEndLimit attribute, then it
    // SHALL also include flowRateStartLimit attribute.
    // For example, a service provider’s tariff limits customers to 20 kWs of
    // demand for the given rate structure. Above this threshold (from 20-50
    // kWs), there are different demand charges per unit of consumption. The
    // service provider can use flowRateStartLimit and flowRateEndLimit to
    // describe the demand characteristics of the different rates. Similarly,
    // these attributes can be used to describe limits on premises DERs that
    // might be producing a commodity and sending it back into the distribution
    // network.
    // Note: At the time of writing, service provider tariffs with demand-based
    // components were not originally identified as being in scope, and service
    // provider tariffs vary widely in their use of demand components and the
    // method for computing charges. It is expected that industry groups (e.g.,
    // OpenSG) will document requirements in the future that the IEEE 2030.5
    // community can then use as source material for the next version of IEEE
    // 2030.5.
    #[yaserde(rename = "flowRateEndLimit")]
    pub flow_rate_end_limit: Option<UnitValueType>,

    // Specifies the minimum flow rate (e.g., kW for electricity) for which this
    // RateComponent applies, for the usage point and given rate / tariff.
    // In combination with flowRateEndLimit, allows a service provider to define
    // the demand or output characteristics for the particular tariff design. If
    // a server includes the flowRateStartLimit attribute, then it SHALL also
    // include flowRateEndLimit attribute.
    #[yaserde(rename = "flowRateStartLimit")]
    pub flow_rate_start_limit: Option<UnitValueType>,

    // Provides indication of the ReadingType with which this price is
    // associated.
    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: ReadingTypeLink,

    // Specifies the roles that this usage point has been assigned.
    #[yaserde(rename = "roleFlags")]
    pub role_flags: RoleFlagsType,

    #[yaserde(rename = "TimeTariffIntervalListLink")]
    pub time_tariff_interval_list_link: TimeTariffIntervalListLink,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for RateComponent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RateComponent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for RateComponent {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "RateComponentList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RateComponentList {
    #[yaserde(rename = "RateComponent")]
    pub rate_component: Vec<RateComponent>,

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

impl Validate for RateComponentList {}

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
#[yaserde(rename = "TariffProfileList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TariffProfileList {
    #[yaserde(rename = "TariffProfile")]
    pub tariff_profile: Vec<TariffProfile>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

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

impl Validate for TariffProfileList {}

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
#[yaserde(rename = "TimeTariffIntervalList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TimeTariffIntervalList {
    #[yaserde(rename = "TimeTariffInterval")]
    pub time_tariff_interval: Vec<TimeTariffInterval>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

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

impl Validate for TimeTariffIntervalList {}

// A schedule of charges; structure that allows the definition
// of tariff structures such as step (block) and time of use (tier)
// when used in conjunction with TimeTariffInterval and ConsumptionTariffInterval.
#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "TariffProfile")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TariffProfile {
    // The currency code indicating the currency for this TariffProfile.
    #[yaserde(rename = "currency")]
    pub currency: Option<CurrencyCode>,

    // Indicates the power of ten multiplier for the price attribute.
    #[yaserde(rename = "pricePowerOfTenMultiplier")]
    pub price_power_of_ten_multiplier: Option<PowerOfTenMultiplierType>,

    // Indicates the relative primacy of the provider of this program.
    #[yaserde(rename = "primacy")]
    pub primacy: PrimacyType,

    // The rate code for this tariff profile. Provided by the Pricing service
    // provider per its internal business needs and practices and provides a
    // method to identify the specific rate code for the TariffProfile instance.
    // This would typically not be communicated to the user except to facilitate
    // troubleshooting due to its service provider-specific technical nature.
    #[yaserde(rename = "rateCode")]
    pub rate_code: Option<String20>,

    #[yaserde(rename = "RateComponentListLink")]
    pub rate_component_list_link: Option<RateComponentListLink>,

    // The kind of service provided by this usage point.
    #[yaserde(rename = "serviceCategoryKind")]
    pub service_category_kind: ServiceKind,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub mrid: MRIDType,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for TariffProfile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TariffProfile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for TariffProfile {}
