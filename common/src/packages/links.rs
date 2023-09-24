use yaserde::{YaDeserialize, YaSerialize};

use crate::traits::{SELink, SEListLink, Validate};
use sep2_common_derive::{SELink, SEListLink};

use super::primitives::Uint32;

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "AccountBalanceLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AccountBalanceLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for AccountBalanceLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ActiveBillingPeriodListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveBillingPeriodListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveBillingPeriodListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ActiveCreditRegisterListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveCreditRegisterListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveCreditRegisterListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ActiveDERControlListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveDERControlListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveDERControlListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ActiveEndDeviceControlListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveEndDeviceControlListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveEndDeviceControlListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ActiveFlowReservationListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveFlowReservationListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveFlowReservationListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ActiveProjectionReadingListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveProjectionReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveProjectionReadingListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ActiveSupplyInterruptionOverrideListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveSupplyInterruptionOverrideListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveSupplyInterruptionOverrideListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ActiveTargetReadingListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveTargetReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveTargetReadingListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ActiveTextMessageListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveTextMessageListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveTextMessageListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ActiveTimeTariffIntervalListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveTimeTariffIntervalListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveTimeTariffIntervalListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "AssociatedDERProgramListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AssociatedDERProgramListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for AssociatedDERProgramListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "AssociatedUsagePointLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AssociatedUsagePointLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for AssociatedUsagePointLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "BillingPeriodListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingPeriodListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for BillingPeriodListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "BillingReadingListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for BillingReadingListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "BillingReadingSetListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReadingSetListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for BillingReadingSetListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "ConfigurationLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ConfigurationLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ConfigurationLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ConsumptionTariffIntervalListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ConsumptionTariffIntervalListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ConsumptionTariffIntervalListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "CreditRegisterListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CreditRegisterListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for CreditRegisterListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "CustomerAccountLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAccountLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for CustomerAccountLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "CustomerAccountListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAccountListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for CustomerAccountListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "CustomerAgreementListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAgreementListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for CustomerAgreementListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DemandResponseProgramLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DemandResponseProgramLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DemandResponseProgramLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "DemandResponseProgramListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DemandResponseProgramListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DemandResponseProgramListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DERAvailabilityLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERAvailabilityLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DERAvailabilityLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DefaultDERControlLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DefaultDERControlLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DefaultDERControlLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DERCapabilityLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERCapabilityLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DERCapabilityLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "DERControlListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERControlListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DERControlListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DERCurveLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERCurveLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DERCurveLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "DERCurveListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERCurveListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DERCurveListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DERLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DERLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "DERListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DERListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DERProgramLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERProgramLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DERProgramLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "DERProgramListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERProgramListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DERProgramListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DERSettingsLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERSettingsLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DERSettingsLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DERStatusLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERStatusLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DERStatusLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DeviceCapabilityLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceCapabilityLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DeviceCapabilityLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DeviceInformationLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceInformationLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DeviceInformationLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "DeviceStatusLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceStatusLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DeviceStatusLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "EndDeviceControlListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceControlListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for EndDeviceControlListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "EndDeviceLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for EndDeviceLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "EndDeviceListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for EndDeviceListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "FileLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FileLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FileLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "FileListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FileListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FileListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "FileStatusLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FileStatusLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FileStatusLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "FlowReservationRequestListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationRequestListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FlowReservationRequestListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "FlowReservationResponseListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationResponseListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FlowReservationResponseListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "FunctionSetAssignmentsListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FunctionSetAssignmentsListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FunctionSetAssignmentsListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "HistoricalReadingListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct HistoricalReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for HistoricalReadingListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "IPAddrListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IPAddrListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for IPAddrListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "IPInterfaceListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IPInterfaceListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for IPInterfaceListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "LLInterfaceListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LlinterfaceListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for LlinterfaceListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "LoadShedAvailabilityListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LoadShedAvailabilityListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for LoadShedAvailabilityListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "LogEventListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LogEventListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for LogEventListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "MessagingProgramListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MessagingProgramListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for MessagingProgramListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "MeterReadingLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MeterReadingLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for MeterReadingLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "MeterReadingListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MeterReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for MeterReadingListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "MirrorUsagePointListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorUsagePointListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for MirrorUsagePointListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "NeighborListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct NeighborListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for NeighborListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "NotificationListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct NotificationListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for NotificationListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "PowerStatusLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PowerStatusLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for PowerStatusLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "PrepaymentLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PrepaymentLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for PrepaymentLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "PrepaymentListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PrepaymentListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for PrepaymentListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "PrepayOperationStatusLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PrepayOperationStatusLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for PrepayOperationStatusLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "PriceResponseCfgListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PriceResponseCfgListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for PriceResponseCfgListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ProjectionReadingListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ProjectionReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ProjectionReadingListLink {}

#[derive(
    Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, YaSerialize, YaDeserialize, SELink,
)]
#[yaserde(rename = "RateComponentLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RateComponentLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for RateComponentLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "RateComponentListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RateComponentListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for RateComponentListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "ReadingLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ReadingLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ReadingListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ReadingListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ReadingSetListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingSetListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ReadingSetListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "ReadingTypeLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingTypeLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ReadingTypeLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "RegistrationLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RegistrationLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for RegistrationLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ResponseListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ResponseListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ResponseListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "ResponseSetListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ResponseSetListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ResponseSetListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "RPLInstanceListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RplinstanceListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for RplinstanceListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "RPLSourceRoutesListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RplsourceRoutesListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for RplsourceRoutesListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "SelfDeviceLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SelfDeviceLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for SelfDeviceLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "ServiceSupplierLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ServiceSupplierLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ServiceSupplierLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "SubscriptionListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscriptionListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for SubscriptionListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "SupplyInterruptionOverrideListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupplyInterruptionOverrideListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for SupplyInterruptionOverrideListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "SupportedLocaleListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupportedLocaleListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for SupportedLocaleListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "TargetReadingListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TargetReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TargetReadingListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "TariffProfileLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TariffProfileLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TariffProfileLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "TariffProfileListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TariffProfileListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TariffProfileListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "TextMessageListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TextMessageListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TextMessageListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "TimeLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TimeLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TimeLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "TimeTariffIntervalListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TimeTariffIntervalListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TimeTariffIntervalListLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SELink)]
#[yaserde(rename = "UsagePointLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UsagePointLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for UsagePointLink {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEListLink, SELink)]
#[yaserde(rename = "UsagePointListLink")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UsagePointListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for UsagePointListLink {}
