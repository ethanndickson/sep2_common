use crate::traits::{
    SEBillingMeterReadingBase, SEIdentifiedObject, SEList, SEMeterReadingBase, SEReadingBase,
    SEReadingSetBase, SEResource, SESubscribableList, SESubscribableResource, Validate,
};
use sep2_common_derive::{
    SEBillingMeterReadingBase, SEIdentifiedObject, SEList, SEMeterReadingBase, SEReadingBase,
    SEReadingSetBase, SEResource, SESubscribableList, SESubscribableResource,
};

use super::{
    links::{
        ActiveBillingPeriodListLink, ActiveProjectionReadingListLink, ActiveTargetReadingListLink,
        BillingPeriodListLink, BillingReadingListLink, BillingReadingSetListLink,
        CustomerAgreementListLink, HistoricalReadingListLink, PrepaymentLink,
        ProjectionReadingListLink, ReadingTypeLink, ServiceSupplierLink, TargetReadingListLink,
        TariffProfileLink, UsagePointLink,
    },
    primitives::{HexBinary16, Int32, Int48, String20, String32, String42, Uint16, Uint32},
    types::{
        ConsumptionBlockType, DateTimeInterval, MRIDType, PowerOfTenMultiplierType,
        SubscribableType, TOUType, TimeType, VersionType,
    },
};
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "BillingPeriod")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingPeriod {
    /// The amount of the bill for the previous billing period.
    #[yaserde(rename = "billLastPeriod")]
    pub bill_last_period: Option<Int48>,

    /// The bill amount related to the billing period as of the statusTimeStamp.
    #[yaserde(rename = "billToDate")]
    pub bill_to_date: Option<Int48>,

    /// The time interval for this billing period.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    /// The date / time of the last update of this resource.
    #[yaserde(rename = "statusTimeStamp")]
    pub status_time_stamp: Option<TimeType>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for BillingPeriod {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BillingPeriod {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - customerName (ascending)
        match self.interval.start.cmp(&other.interval.start).reverse() {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mRID (descending)
        self.href.cmp(&other.href)
    }
}

impl Validate for BillingPeriod {}

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
#[yaserde(rename = "BillingPeriodList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingPeriodList {
    #[yaserde(rename = "BillingPeriod")]
    pub billing_period: Vec<BillingPeriod>,

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

impl Validate for BillingPeriodList {}

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
    SEBillingMeterReadingBase,
)]
#[yaserde(rename = "BillingMeterReadingBase")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingMeterReadingBase {
    #[yaserde(rename = "BillingReadingSetListLink")]
    pub billing_reading_set_list_link: Option<BillingReadingSetListLink>,

    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: Option<ReadingTypeLink>,

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

impl Validate for BillingMeterReadingBase {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEReadingBase, SEResource,
)]
#[yaserde(rename = "BillingReading")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReading {
    #[yaserde(rename = "Charge")]
    pub charge: Vec<Charge>,

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
    // Projection or forecast of future readings
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
    pub tou_tier: Option<TOUType>,

    /// Value in units specified by ReadingType
    #[yaserde(rename = "value")]
    pub value: Option<Int48>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for BillingReading {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BillingReading {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - timePeriod.start (ascending)
        if let Some((self_start, other_start)) =
            self.time_period.as_ref().zip(other.time_period.as_ref())
        {
            match self_start.start.cmp(&other_start.start) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            }
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

impl Validate for BillingReading {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "BillingReadingList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReadingList {
    #[yaserde(rename = "BillingReading")]
    pub billing_reading: Vec<BillingReading>,

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

impl Validate for BillingReadingList {}

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
#[yaserde(rename = "BillingReadingSet")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReadingSet {
    #[yaserde(rename = "BillingReadingListLink")]
    pub billing_reading_list_link: Option<BillingReadingListLink>,

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

impl PartialOrd for BillingReadingSet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BillingReadingSet {
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

impl Validate for BillingReadingSet {}

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
#[yaserde(rename = "BillingReadingSetList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReadingSetList {
    #[yaserde(rename = "BillingReadingSet")]
    pub billing_reading_set: Vec<BillingReadingSet>,

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

impl Validate for BillingReadingSetList {}

/// Charges contain charges on a customer bill. These could be items like taxes,
/// levies, surcharges, rebates, or others. This is meant to allow the HAN device
/// to retrieve enough information to be able to reconstruct an estimate of what
/// the total bill would look like.
// Providers can provide line item billing, including multiple charge kinds
/// (e.g. taxes, surcharges) at whatever granularity desired, using as many
/// Charges as desired during a billing period. There can also be any number of
/// Charges associated with different ReadingTypes to distinguish between TOU
/// tiers, consumption blocks, or demand charges.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "Charge")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Charge {
    /// A description of the charge.
    #[yaserde(rename = "description")]
    pub description: Option<String20>,

    /// The type (kind) of charge.
    #[yaserde(rename = "kind")]
    pub kind: Option<ChargeKind>,

    /// A monetary charge.
    #[yaserde(rename = "value")]
    pub value: Int32,
}

impl Validate for Charge {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ChargeKind")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum ChargeKind {
    #[default]
    ConsumptionCharge = 0,
    Rebate = 1,
    AuxiliaryCharge = 2,
    DemandCharge = 3,
    TaxCharge = 4,
    // 5-255 NOT RESERVED
}

impl Validate for ChargeKind {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "CustomerAccount")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAccount {
    /// The ISO 4217 code indicating the currency applicable to the bill amounts
    /// in the summary. See list at
    /// http://www.unece.org/cefact/recommendations/rec09/rec09_ecetrd203.pdf
    #[yaserde(rename = "currency")]
    pub currency: Uint16,

    /// The account number for the customer (if applicable).
    #[yaserde(rename = "customerAccount")]
    pub customer_account: Option<String42>,

    #[yaserde(rename = "CustomerAgreementListLink")]
    pub customer_agreement_list_link: Option<CustomerAgreementListLink>,

    /// The name of the customer.
    #[yaserde(rename = "customerName")]
    pub customer_name: Option<String42>,

    /// Indicates the power of ten multiplier for the prices in this function
    /// set.
    #[yaserde(rename = "pricePowerOfTenMultiplier")]
    pub price_power_of_ten_multiplier: PowerOfTenMultiplierType,

    #[yaserde(rename = "ServiceSupplierLink")]
    pub service_supplier_link: Option<ServiceSupplierLink>,

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

impl PartialOrd for CustomerAccount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CustomerAccount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - customerName (ascending)
        match self.customer_name.cmp(&other.customer_name) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for CustomerAccount {}

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
#[yaserde(rename = "CustomerAccountList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAccountList {
    #[yaserde(rename = "CustomerAccount")]
    pub customer_account: Vec<CustomerAccount>,

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

impl Validate for CustomerAccountList {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "CustomerAgreement")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAgreement {
    #[yaserde(rename = "ActiveBillingPeriodListLink")]
    pub active_billing_period_list_link: Option<ActiveBillingPeriodListLink>,

    #[yaserde(rename = "ActiveProjectionReadingListLink")]
    pub active_projection_reading_list_link: Option<ActiveProjectionReadingListLink>,

    #[yaserde(rename = "ActiveTargetReadingListLink")]
    pub active_target_reading_list_link: Option<ActiveTargetReadingListLink>,

    #[yaserde(rename = "BillingPeriodListLink")]
    pub billing_period_list_link: Option<BillingPeriodListLink>,

    #[yaserde(rename = "HistoricalReadingListLink")]
    pub historical_reading_list_link: Option<HistoricalReadingListLink>,

    #[yaserde(rename = "PrepaymentLink")]
    pub prepayment_link: Option<PrepaymentLink>,

    #[yaserde(rename = "ProjectionReadingListLink")]
    pub projection_reading_list_link: Option<ProjectionReadingListLink>,

    /// The account number of the service account (if applicable).
    #[yaserde(rename = "serviceAccount")]
    pub service_account: Option<String42>,

    /// The address or textual description of the service location.
    #[yaserde(rename = "serviceLocation")]
    pub service_location: Option<String42>,

    #[yaserde(rename = "TargetReadingListLink")]
    pub target_reading_list_link: Option<TargetReadingListLink>,

    #[yaserde(rename = "TariffProfileLink")]
    pub tariff_profile_link: Option<TariffProfileLink>,

    #[yaserde(rename = "UsagePointLink")]
    pub usage_point_link: Option<UsagePointLink>,

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

impl PartialOrd for CustomerAgreement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CustomerAgreement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - serviceLocation (ascending)
        match self.service_location.cmp(&other.service_location) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for CustomerAgreement {}

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
#[yaserde(rename = "CustomerAgreementList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAgreementList {
    #[yaserde(rename = "CustomerAgreement")]
    pub customer_agreement: Vec<CustomerAgreement>,

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

impl Validate for CustomerAgreementList {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEBillingMeterReadingBase,
    SEMeterReadingBase,
    SEIdentifiedObject,
    SEResource,
)]
#[yaserde(rename = "HistoricalReading")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct HistoricalReading {
    #[yaserde(rename = "BillingReadingSetListLink")]
    pub billing_reading_set_list_link: Option<BillingReadingSetListLink>,

    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: Option<ReadingTypeLink>,

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

impl PartialOrd for HistoricalReading {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HistoricalReading {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - description (ascending)
        match self.description.cmp(&other.description) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for HistoricalReading {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "HistoricalReadingList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct HistoricalReadingList {
    #[yaserde(rename = "HistoricalReading")]
    pub historical_reading: Vec<HistoricalReading>,

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

impl Validate for HistoricalReadingList {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEBillingMeterReadingBase,
    SEMeterReadingBase,
    SEIdentifiedObject,
    SEResource,
)]
#[yaserde(rename = "ProjectionReading")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ProjectionReading {
    #[yaserde(rename = "BillingReadingSetListLink")]
    pub billing_reading_set_list_link: Option<BillingReadingSetListLink>,

    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: Option<ReadingTypeLink>,

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

impl PartialOrd for ProjectionReading {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProjectionReading {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - description (ascending)
        match self.description.cmp(&other.description) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for ProjectionReading {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "ProjectionReadingList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ProjectionReadingList {
    #[yaserde(rename = "ProjectionReading")]
    pub projection_reading: Vec<ProjectionReading>,

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

impl Validate for ProjectionReadingList {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEBillingMeterReadingBase,
    SEMeterReadingBase,
    SEIdentifiedObject,
    SEResource,
)]
#[yaserde(rename = "TargetReading")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TargetReading {
    #[yaserde(rename = "BillingReadingSetListLink")]
    pub billing_reading_set_list_link: Option<BillingReadingSetListLink>,

    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: Option<ReadingTypeLink>,

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

impl PartialOrd for TargetReading {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TargetReading {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - description (ascending)
        match self.description.cmp(&other.description) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for TargetReading {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "TargetReadingList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TargetReadingList {
    #[yaserde(rename = "TargetReading")]
    pub target_reading: Vec<TargetReading>,

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

impl Validate for TargetReadingList {}

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
    SEIdentifiedObject,
    SEResource,
)]
#[yaserde(rename = "ServiceSupplier")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ServiceSupplier {
    /// E-mail address for this service supplier.
    #[yaserde(rename = "email")]
    pub email: Option<String32>,

    /// Human-readable phone number for this service supplier.
    #[yaserde(rename = "phone")]
    pub phone: Option<String20>,

    /// Contains the IANA PEN for the commodity provider.
    #[yaserde(rename = "providerID")]
    pub provider_id: Option<Uint32>,

    /// Website URI address for this service supplier.
    #[yaserde(rename = "web")]
    pub web: Option<String42>,

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

impl Validate for ServiceSupplier {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "ServiceSupplierList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ServiceSupplierList {
    #[yaserde(rename = "ServiceSupplier")]
    pub service_supplier: Vec<ServiceSupplier>,

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

impl Validate for ServiceSupplierList {}
