use crate::traits::{
    SEIdentifiedObject, SEList, SEResource, SESubscribableList, SESubscribableResource, Validate,
};
use sep2_common_derive::{
    SEIdentifiedObject, SEList, SEResource, SESubscribableList, SESubscribableResource,
};

use yaserde::{YaDeserialize, YaSerialize};

use super::{
    identification::{Link, ListLink},
    metering::UsagePoint,
    primitives::{Int32, String32, Uint32},
    types::{
        CurrencyCode, DateTimeInterval, MRIDType, PowerOfTenMultiplierType, RealEnergy,
        SubscribableType, TimeType, VersionType,
    },
};

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "AccountBalance")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AccountBalance {
    // AvailableCredit shows the balance of the sum of credits minus the sum of
    // charges. In a Central Wallet mode this value may be passed down to the
    // Prepayment server via an out-of-band mechanism. In Local or ESI modes,
    // this value may be calculated based upon summation of CreditRegister
    // transactions minus consumption charges calculated using Metering (and
    // possibly Pricing) function set data. This value may be negative; for
    // instance, if disconnection is prevented due to a Supply Interruption
    // Override.
    #[yaserde(rename = "availableCredit")]
    pub available_credit: AccountingUnit,

    // CreditStatus identifies whether the present value of availableCredit is
    // considered OK, low, exhausted, or negative.
    #[yaserde(rename = "creditStatus")]
    pub credit_status: Option<CreditStatusType>,

    // EmergencyCredit is the amount of credit still available for the given
    // service or commodity prepayment instance. If both availableCredit and
    // emergyCredit are exhausted, then service will typically be disconnected.
    #[yaserde(rename = "emergencyCredit")]
    pub emergency_credit: Option<AccountingUnit>,

    // EmergencyCreditStatus identifies whether the present value of
    // emergencyCredit is considered OK, low, exhausted, or negative.
    #[yaserde(rename = "emergencyCreditStatus")]
    pub emergency_credit_status: Option<CreditStatusType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for AccountBalance {}

// Unit for accounting; use either 'energyUnit' or 'currencyUnit' to specify the
// unit for 'value'.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "AccountingUnit")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AccountingUnit {
    // Unit of service.
    #[yaserde(rename = "energyUnit")]
    pub energy_unit: Option<RealEnergy>,

    // Unit of currency.
    #[yaserde(rename = "monetaryUnit")]
    pub monetary_unit: CurrencyCode,

    // Multiplier for the 'energyUnit' or 'monetaryUnit'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value of the monetary aspect
    #[yaserde(rename = "value")]
    pub value: Int32,
}

impl Validate for AccountingUnit {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "CreditRegister")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CreditRegister {
    // CreditAmount is the amount of credit being added by a particular
    // CreditRegister transaction. Negative values indicate that credit is being
    // subtracted.
    #[yaserde(rename = "creditAmount")]
    pub credit_amount: AccountingUnit,

    // CreditType indicates whether the credit transaction applies to regular or
    // emergency credit.
    #[yaserde(rename = "creditType")]
    pub credit_type: Option<CreditTypeType>,

    // EffectiveTime identifies the time at which the credit transaction goes
    // into effect. For credit addition transactions, this is typically the
    // moment at which the transaction takes place. For credit subtraction
    // transactions, (e.g., non-fuel debt recovery transactions initiated from a
    // back-haul or ESI) this may be a future time at which credit is deducted.
    #[yaserde(rename = "effectiveTime")]
    pub effective_time: TimeType,

    // Token is security data that authenticates the legitimacy of the
    // transaction. The details of this token are not defined by IEEE 2030.5.
    // How a Prepayment server handles this field is left as vendor specific
    // implementation or will be defined by one or more other standards.
    #[yaserde(rename = "token")]
    pub token: String32,

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

impl PartialOrd for CreditRegister {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CreditRegister {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - effectiveTime (descending)
        match self.effective_time.cmp(&other.effective_time).reverse() {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for CreditRegister {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "CreditRegisterList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CreditRegisterList {
    #[yaserde(rename = "CreditRegister")]
    pub credit_register: Vec<CreditRegister>,

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

impl Validate for CreditRegisterList {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "Prepayment")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Prepayment {
    #[yaserde(rename = "AccountBalanceLink")]
    pub account_balance_link: Link,

    #[yaserde(rename = "ActiveCreditRegisterListLink")]
    pub active_credit_register_list_link: Option<ListLink>,

    #[yaserde(rename = "ActiveSupplyInterruptionOverrideListLink")]
    pub active_supply_interruption_override_list_link: Option<ListLink>,

    // CreditExpiryLevel is the set point for availableCredit at which the
    // service level may be changed. The typical value for this attribute is 0,
    // regardless of whether the account balance is measured in a monetary or
    // commodity basis. The units for this attribute SHALL match the units used
    // for availableCredit.
    #[yaserde(rename = "creditExpiryLevel")]
    pub credit_expiry_level: Option<AccountingUnit>,

    #[yaserde(rename = "CreditRegisterListLink")]
    pub credit_register_list_link: ListLink,

    // LowCreditWarningLevel is the set point for availableCredit at which the
    // creditStatus attribute in the AccountBalance resource SHALL indicate that
    // available credit is low. The units for this attribute SHALL match the
    // units used for availableCredit. Typically, this value is set by the
    // service provider.
    #[yaserde(rename = "lowCreditWarningLevel")]
    pub low_credit_warning_level: Option<AccountingUnit>,

    // LowEmergencyCreditWarningLevel is the set point for emergencyCredit at
    // which the creditStatus attribute in the AccountBalance resource SHALL
    // indicate that emergencycredit is low. The units for this attribute SHALL
    // match the units used for availableCredit. Typically, this value is set by
    // the service provider.
    #[yaserde(rename = "lowEmergencyCreditWarningLevel")]
    pub low_emergency_credit_warning_level: Option<AccountingUnit>,

    // PrepayMode specifies whether the given Prepayment instance is operating
    // in Credit, Central Wallet, ESI, or Local prepayment mode. The Credit mode
    // indicates that prepayment is not presently in effect. The other modes are
    // described in the Overview Section above.
    #[yaserde(rename = "prepayMode")]
    pub prepay_mode: PrepayModeType,

    #[yaserde(rename = "PrepayOperationStatusLink")]
    pub prepay_operation_status_link: Link,

    #[yaserde(rename = "SupplyInterruptionOverrideListLink")]
    pub supply_interruption_override_list_link: ListLink,

    #[yaserde(rename = "UsagePoint")]
    pub usage_point: Vec<UsagePoint>,

    #[yaserde(rename = "UsagePointLink")]
    pub usage_point_link: Option<Link>,

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

impl PartialOrd for Prepayment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Prepayment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for Prepayment {}

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
#[yaserde(rename = "PrepaymentList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PrepaymentList {
    #[yaserde(rename = "Prepayment")]
    pub prepayment: Vec<Prepayment>,

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

impl Validate for PrepaymentList {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "PrepayModeType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum PrepayModeType {
    #[default]
    CentralWallet = 0,
    ESI = 1,
    Local = 2,
    Credit = 3,
    // 4-255 RESERVED
}

impl Validate for PrepayModeType {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "PrepayOperationStatus")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PrepayOperationStatus {
    // CreditTypeChange is used to define a pending change of creditTypeInUse,
    // which will activate at a specified time.
    #[yaserde(rename = "creditTypeChange")]
    pub credit_type_change: Option<CreditTypeChange>,

    // CreditTypeInUse identifies whether the present mode of operation is
    // consuming regular credit or emergency credit.
    #[yaserde(rename = "creditTypeInUse")]
    pub credit_type_in_use: Option<CreditTypeType>,

    // ServiceChange is used to define a pending change of serviceStatus, which
    // will activate at a specified time.
    #[yaserde(rename = "serviceChange")]
    pub service_change: Option<ServiceChange>,

    // ServiceStatus identifies whether the service is connected or
    // disconnected, or armed for connection or disconnection.
    #[yaserde(rename = "serviceStatus")]
    pub service_status: ServiceStatusType,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for PrepayOperationStatus {}

// Specifies a change to the service status.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ServiceChange")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ServiceChange {
    // The new service status, to take effect at the time specified by startTime
    #[yaserde(rename = "newStatus")]
    pub new_status: ServiceStatusType,

    // The date/time when the change is to take effect.
    #[yaserde(rename = "startTime")]
    pub start_time: TimeType,
}

impl Validate for ServiceChange {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "SupplyInterruptionOverride")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupplyInterruptionOverride {
    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Interval defines the period of time during which supply should not be
    // interrupted.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for SupplyInterruptionOverride {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SupplyInterruptionOverride {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - interval.start (ascending)
        match self.interval.start.cmp(&other.interval.start) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - interval.duration (ascending)
        self.interval.duration.cmp(&other.interval.duration)
    }
}

impl Validate for SupplyInterruptionOverride {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "SupplyInterruptionOverrideList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupplyInterruptionOverrideList {
    #[yaserde(rename = "SupplyInterruptionOverride")]
    pub supply_interruption_override: Vec<SupplyInterruptionOverride>,

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

impl Validate for SupplyInterruptionOverrideList {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "CreditStatusType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum CreditStatusType {
    #[default]
    Ok = 0,
    Low = 1,
    Exhausted = 2,
    Negative = 3,
    // 4-255 RESERVED
}

impl Validate for CreditStatusType {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "CreditTypeType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum CreditTypeType {
    #[default]
    Regular = 0,
    Emergency = 1,
    RegularThenEmergency = 2,
    EmergencyThenRegular = 3,
    // 4-255 RESERVED
}

impl Validate for CreditTypeType {}

// Specifies a change to the credit type.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "CreditTypeChange")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CreditTypeChange {
    // The new credit type, to take effect at the time specified by startTime
    #[yaserde(rename = "newType")]
    pub new_type: CreditTypeType,

    // The date/time when the change is to take effect.
    #[yaserde(rename = "startTime")]
    pub start_time: TimeType,
}

impl Validate for CreditTypeChange {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ServiceStatusType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum ServiceStatusType {
    #[default]
    Connected = 0,
    Disconnected = 1,
    ArmedForConnect = 2,
    ArmedForDisconnect = 3,
    NoContactor = 4,
    LoadLimited = 5,
    // 6-255 RESERVED
}

impl Validate for ServiceStatusType {}
