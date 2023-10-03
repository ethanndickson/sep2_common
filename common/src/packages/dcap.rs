use crate::traits::{SEFunctionSetAssignmentsBase, SEResource, Validate};
use sep2_common_derive::{SEFunctionSetAssignmentsBase, SEResource};

use yaserde::{YaDeserialize, YaSerialize};

use super::{
    identification::{Link, ListLink},
    primitives::Uint32,
};

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SEFunctionSetAssignmentsBase,
    SEResource,
)]
#[yaserde(rename = "DeviceCapability")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceCapability {
    #[yaserde(rename = "EndDeviceListLink")]
    pub end_device_list_link: Option<ListLink>,

    #[yaserde(rename = "MirrorUsagePointListLink")]
    pub mirror_usage_point_list_link: Option<ListLink>,

    #[yaserde(rename = "SelfDeviceLink")]
    pub self_device_link: Option<Link>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    #[yaserde(rename = "CustomerAccountListLink")]
    pub customer_account_list_link: Option<ListLink>,

    #[yaserde(rename = "DemandResponseProgramListLink")]
    pub demand_response_program_list_link: Option<ListLink>,

    #[yaserde(rename = "DERProgramListLink")]
    pub der_program_list_link: Option<ListLink>,

    #[yaserde(rename = "FileListLink")]
    pub file_list_link: Option<ListLink>,

    #[yaserde(rename = "MessagingProgramListLink")]
    pub messaging_program_list_link: Option<ListLink>,

    #[yaserde(rename = "PrepaymentListLink")]
    pub prepayment_list_link: Option<ListLink>,

    #[yaserde(rename = "ResponseSetListLink")]
    pub response_set_list_link: Option<ListLink>,

    #[yaserde(rename = "TariffProfileListLink")]
    pub tariff_profile_list_link: Option<ListLink>,

    #[yaserde(rename = "TimeLink")]
    pub time_link: Option<Link>,

    #[yaserde(rename = "UsagePointListLink")]
    pub usage_point_list_link: Option<ListLink>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DeviceCapability {}
