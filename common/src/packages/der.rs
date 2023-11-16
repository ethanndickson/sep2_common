use crate::traits::{
    SEEvent, SEIdentifiedObject, SEList, SERandomizableEvent, SEResource, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SESubscribableIdentifiedObject, SESubscribableList,
    SESubscribableResource, Validate,
};
use sep2_common_derive::{
    SEEvent, SEIdentifiedObject, SEList, SERandomizableEvent, SEResource, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SESubscribableIdentifiedObject, SESubscribableList,
    SESubscribableResource,
};

use super::{
    identification::ResponseRequired,
    links::{
        ActiveDERControlListLink, AssociatedDERProgramListLink, AssociatedUsagePointLink,
        DERAvailabilityLink, DERCapabilityLink, DERControlListLink, DERCurveLink, DERCurveListLink,
        DERProgramLink, DERSettingsLink, DERStatusLink, DefaultDERControlLink,
    },
    objects::EventStatus,
    primitives::{Int16, Int32, String32, String6, Uint16, Uint32, Uint8},
    types::{
        DateTimeInterval, DeviceCategoryType, MRIDType, OneHourRangeType, Percent,
        PowerOfTenMultiplierType, PrimacyType, SignedPercent, SubscribableType, TimeType,
        VersionType,
    },
};
use bitflags::bitflags;
use yaserde::{HexBinaryYaSerde, YaDeserialize, YaSerialize};

#[cfg(test)]
use crate::serialize;

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SESubscribableIdentifiedObject,
    SEIdentifiedObject,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "DefaultDERControl")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DefaultDERControl {
    #[yaserde(rename = "DERControlBase")]
    pub der_control_base: DERControlBase,

    /// Enter service delay, in hundredths of a second. When present, this value
    /// SHALL update the value of the corresponding setting
    /// (DERSettings::setESDelay).
    #[yaserde(rename = "setESDelay")]
    pub set_es_delay: Option<Uint32>,

    /// Enter service frequency high. Specified in hundredths of Hz. When
    /// present, this value SHALL update the value of the corresponding setting
    /// (DERSettings::setESHighFreq).
    #[yaserde(rename = "setESHighFreq")]
    pub set_es_high_freq: Option<Uint16>,

    /// Enter service voltage high. Specified as an effective percent voltage,
    /// defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in
    /// hundredths of a percent. When present, this value SHALL update the value
    /// of the corresponding setting (DERSettings::setESHighVolt).
    #[yaserde(rename = "setESHighVolt")]
    pub set_es_high_volt: Option<Int16>,

    /// Enter service frequency low. Specified in hundredths of Hz. When present,
    /// this value SHALL update the value of the corresponding setting
    /// (DERSettings::setESLowFreq).
    #[yaserde(rename = "setESLowFreq")]
    pub set_es_low_freq: Option<Uint16>,

    /// Enter service voltage low. Specified as an effective percent voltage,
    /// defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in
    /// hundredths of a percent. When present, this value SHALL update the value
    /// of the corresponding setting (DERSettings::setESLowVolt).
    #[yaserde(rename = "setESLowVolt")]
    pub set_es_low_volt: Option<Int16>,

    /// Enter service ramp time, in hundredths of a second. When present, this
    /// value SHALL update the value of the corresponding setting
    /// (DERSettings::setESRampTms).
    #[yaserde(rename = "setESRampTms")]
    pub set_es_ramp_tms: Option<Uint32>,

    /// Enter service randomized delay, in hundredths of a second. When present,
    /// this value SHALL update the value of the corresponding setting
    /// (DERSettings::setESRandomDelay).
    #[yaserde(rename = "setESRandomDelay")]
    pub set_es_random_delay: Option<Uint32>,

    /// Set default rate of change (ramp rate) of active power output due to
    /// command or internal action, defined in %setWMax / second. Resolution is
    /// in hundredths of a percent/second. A value of 0 means there is no limit.
    /// Interpreted as a percentage change in output capability limit per second
    /// when used as a default ramp rate. When present, this value SHALL update
    /// the value of the corresponding setting (DERSettings::setGradW).
    #[yaserde(rename = "setGradW")]
    pub set_grad_w: Option<Uint16>,

    /// Set soft-start rate of change (soft-start ramp rate) of active power
    /// output due to command or internal action, defined in %setWMax / second.
    /// Resolution is in hundredths of a percent/second. A value of 0 means there
    /// is no limit. Interpreted as a percentage change in output capability
    /// limit per second when used as a ramp rate. When present, this value SHALL
    /// update the value of the corresponding setting
    /// (DERSettings::setSoftGradW).
    #[yaserde(rename = "setSoftGradW")]
    pub set_soft_grad_w: Option<Uint16>,

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

impl Validate for DefaultDERControl {}

/// Type for Frequency-Droop (Frequency-Watt) operation.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "FreqDroopType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FreqDroopType {
    /// Frequency droop dead band for over-frequency conditions. In thousandths
    /// of Hz.
    #[yaserde(rename = "dBOF")]
    pub d_bof: Uint32,

    /// Frequency droop dead band for under-frequency conditions. In thousandths
    /// of Hz.
    #[yaserde(rename = "dBUF")]
    pub d_buf: Uint32,

    /// Frequency droop per-unit frequency change for over-frequency conditions
    /// corresponding to 1 per-unit power output change. In thousandths,
    /// unitless.
    #[yaserde(rename = "kOF")]
    pub k_of: Uint16,

    /// Frequency droop per-unit frequency change for under-frequency conditions
    /// corresponding to 1 per-unit power output change. In thousandths,
    /// unitless.
    #[yaserde(rename = "kUF")]
    pub k_uf: Uint16,

    /// Open loop response time, the duration from a step change in control
    /// signal input until the output changes by 90% of its final change before
    /// any overshoot, in hundredths of a second. Resolution is 1/100 sec. A
    /// value of 0 is used to mean no limit.
    #[yaserde(rename = "openLoopTms")]
    pub open_loop_tms: Uint16,
}

impl Validate for FreqDroopType {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "DER")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DER {
    #[yaserde(rename = "AssociatedDERProgramListLink")]
    pub associated_der_program_list_link: Option<AssociatedDERProgramListLink>,

    #[yaserde(rename = "AssociatedUsagePointLink")]
    pub associated_usage_point_link: Option<AssociatedUsagePointLink>,

    #[yaserde(rename = "CurrentDERProgramLink")]
    pub current_der_program_link: Option<DERProgramLink>,

    #[yaserde(rename = "DERAvailabilityLink")]
    pub der_availability_link: Option<DERAvailabilityLink>,

    #[yaserde(rename = "DERCapabilityLink")]
    pub der_capability_link: Option<DERCapabilityLink>,

    #[yaserde(rename = "DERSettingsLink")]
    pub der_settings_link: Option<DERSettingsLink>,

    #[yaserde(rename = "DERStatusLink")]
    pub der_status_link: Option<DERStatusLink>,

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

impl PartialOrd for DER {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DER {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - href (ascending)
        self.href.cmp(&other.href)
    }
}

impl Validate for DER {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "DERList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERList {
    #[yaserde(rename = "DER")]
    pub der: Vec<DER>,

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

impl Validate for DERList {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "DERSettings")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERSettings {
    /// Bitmap indicating the DER Controls enabled on the device. See
    /// DERControlType for values. If a control is supported (see
    /// DERCapability::modesSupported), but not enabled, the control will not be
    /// executed if encountered.
    #[yaserde(rename = "modesEnabled")]
    pub modes_enabled: Option<DERControlType>,

    /// Enter service delay, in hundredths of a second.
    #[yaserde(rename = "setESDelay")]
    pub set_es_delay: Option<Uint32>,

    /// Enter service frequency high. Specified in hundredths of Hz.
    #[yaserde(rename = "setESHighFreq")]
    pub set_es_high_freq: Option<Uint16>,

    /// Enter service voltage high. Specified as an effective percent voltage,
    /// defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in
    /// hundredths of a percent.
    #[yaserde(rename = "setESHighVolt")]
    pub set_es_high_volt: Option<Int16>,

    /// Enter service frequency low. Specified in hundredths of Hz.
    #[yaserde(rename = "setESLowFreq")]
    pub set_es_low_freq: Option<Uint16>,

    /// Enter service voltage low. Specified as an effective percent voltage,
    /// defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in
    /// hundredths of a percent.
    #[yaserde(rename = "setESLowVolt")]
    pub set_es_low_volt: Option<Int16>,

    /// Enter service ramp time, in hundredths of a second.
    #[yaserde(rename = "setESRampTms")]
    pub set_es_ramp_tms: Option<Uint32>,

    /// Enter service randomized delay, in hundredths of a second.
    #[yaserde(rename = "setESRandomDelay")]
    pub set_es_random_delay: Option<Uint32>,

    /// Set default rate of change (ramp rate) of active power output due to
    /// command or internal action, defined in %setWMax / second. Resolution is
    /// in hundredths of a percent/second. A value of 0 means there is no limit.
    /// Interpreted as a percentage change in output capability limit per second
    /// when used as a default ramp rate.
    #[yaserde(rename = "setGradW")]
    pub set_grad_w: Uint16,

    /// AC current maximum. Maximum AC current in RMS Amperes.
    #[yaserde(rename = "setMaxA")]
    pub set_max_a: Option<CurrentRMS>,

    /// Maximum usable energy storage capacity of the DER, in AmpHours. Note:
    /// this may be different from physical capability.
    #[yaserde(rename = "setMaxAh")]
    pub set_max_ah: Option<AmpereHour>,

    /// Apparent power charge maximum. Maximum apparent power the DER can absorb
    /// from the grid in Volt-Amperes. May differ from the apparent power maximum
    /// (setMaxVA).
    #[yaserde(rename = "setMaxChargeRateVA")]
    pub set_max_charge_rate_va: Option<ApparentPower>,

    /// Maximum rate of energy transfer received by the storage device, in Watts.
    /// Defaults to rtgMaxChargeRateW.
    #[yaserde(rename = "setMaxChargeRateW")]
    pub set_max_charge_rate_w: Option<ActivePower>,

    /// Apparent power discharge maximum. Maximum apparent power the DER can
    /// deliver to the grid in Volt-Amperes. May differ from the apparent power
    /// maximum (setMaxVA).
    #[yaserde(rename = "setMaxDischargeRateVA")]
    pub set_max_discharge_rate_va: Option<ApparentPower>,

    /// Maximum rate of energy transfer delivered by the storage device, in
    /// Watts. Defaults to rtgMaxDischargeRateW.
    #[yaserde(rename = "setMaxDischargeRateW")]
    pub set_max_discharge_rate_w: Option<ActivePower>,

    /// AC voltage maximum setting.
    #[yaserde(rename = "setMaxV")]
    pub set_max_v: Option<VoltageRMS>,

    /// Set limit for maximum apparent power capability of the DER (in VA).
    /// Defaults to rtgMaxVA.
    #[yaserde(rename = "setMaxVA")]
    pub set_max_va: Option<ApparentPower>,

    /// Set limit for maximum reactive power delivered by the DER (in var). SHALL
    /// be a positive value &lt;= rtgMaxVar (default).
    #[yaserde(rename = "setMaxVar")]
    pub set_max_var: Option<ReactivePower>,

    /// Set limit for maximum reactive power received by the DER (in var). If
    /// present, SHALL be a negative value &gt;= rtgMaxVarNeg (default). If
    /// absent, defaults to negative setMaxVar.
    #[yaserde(rename = "setMaxVarNeg")]
    pub set_max_var_neg: Option<ReactivePower>,

    /// Set limit for maximum active power capability of the DER (in W). Defaults
    /// to rtgMaxW.
    #[yaserde(rename = "setMaxW")]
    pub set_max_w: ActivePower,

    /// Maximum energy storage capacity of the DER, in WattHours. Note: this may
    /// be different from physical capability.
    #[yaserde(rename = "setMaxWh")]
    pub set_max_wh: Option<WattHour>,

    /// Set minimum Power Factor displacement limit of the DER when injecting
    /// reactive power (over-excited); SHALL be a positive value between 0.0
    /// (typically &gt; 0.7) and 1.0. SHALL be &gt;= rtgMinPFOverExcited
    /// (default).
    #[yaserde(rename = "setMinPFOverExcited")]
    pub set_min_pf_over_excited: Option<PowerFactor>,

    /// Set minimum Power Factor displacement limit of the DER when absorbing
    /// reactive power (under-excited); SHALL be a positive value between 0.0
    /// (typically &gt; 0.7) and 0.9999. If present, SHALL be &gt;=
    /// rtgMinPFUnderExcited (default). If absent, defaults to
    /// setMinPFOverExcited.
    #[yaserde(rename = "setMinPFUnderExcited")]
    pub set_min_pf_under_excited: Option<PowerFactor>,

    /// AC voltage minimum setting.
    #[yaserde(rename = "setMinV")]
    pub set_min_v: Option<VoltageRMS>,

    /// Set soft-start rate of change (soft-start ramp rate) of active power
    /// output due to command or internal action, defined in %setWMax / second.
    /// Resolution is in hundredths of a percent/second. A value of 0 means there
    /// is no limit. Interpreted as a percentage change in output capability
    /// limit per second when used as a ramp rate.
    #[yaserde(rename = "setSoftGradW")]
    pub set_soft_grad_w: Option<Uint16>,

    /// AC voltage nominal setting.
    #[yaserde(rename = "setVNom")]
    pub set_v_nom: Option<VoltageRMS>,

    /// The nominal AC voltage (RMS) at the utility's point of common coupling.
    #[yaserde(rename = "setVRef")]
    pub set_v_ref: Option<VoltageRMS>,

    /// The nominal AC voltage (RMS) offset between the DER's electrical
    /// connection point and the utility's point of common coupling.
    #[yaserde(rename = "setVRefOfs")]
    pub set_v_ref_ofs: Option<VoltageRMS>,

    /// Specifies the time at which the DER information was last updated.
    #[yaserde(rename = "updatedTime")]
    pub updated_time: TimeType,

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

impl Validate for DERSettings {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "DERType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum DERType {
    /// Not applicable / Unknown
    #[default]
    Unknown = 0,
    VirtualOrMixedDER = 1,
    ReciprocatingEngine = 2,
    FuelCell = 3,
    PhotovoltaicSystem = 4,
    HeatAndPower = 5,
    /// Other generation system
    OtherGeneration = 6,
    /// Other storage system
    OtherStorage = 80,
    ElectricVehicle = 81,
    EVSE = 82,
    CombinedPVAndStorage = 83,
    // ALL OTHERS RESERVED
}

impl Validate for DERType {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "DERAvailability")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERAvailability {
    /// Indicates number of seconds the DER will be able to deliver active power
    /// at the reservePercent level.
    #[yaserde(rename = "availabilityDuration")]
    pub availability_duration: Option<Uint32>,

    /// Indicates number of seconds the DER will be able to receive active power
    /// at the reserveChargePercent level.
    #[yaserde(rename = "maxChargeDuration")]
    pub max_charge_duration: Option<Uint32>,

    /// The timestamp when the DER availability was last updated.
    #[yaserde(rename = "readingTime")]
    pub reading_time: TimeType,

    /// Percent of continuous received active power (%setMaxChargeRateW) that is
    /// estimated to be available in reserve.
    #[yaserde(rename = "reserveChargePercent")]
    pub reserve_charge_percent: Option<Percent>,

    /// Percent of continuous delivered active power (%setMaxW) that is estimated
    /// to be available in reserve.
    #[yaserde(rename = "reservePercent")]
    pub reserve_percent: Option<Percent>,

    /// Estimated reserve reactive power, in var. Represents the lesser of
    /// received or delivered reactive power.
    #[yaserde(rename = "statVarAvail")]
    pub stat_var_avail: Option<ReactivePower>,

    /// Estimated reserve active power, in watts.
    #[yaserde(rename = "statWAvail")]
    pub stat_w_avail: Option<ActivePower>,

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

impl Validate for DERAvailability {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "DERCapability")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[cfg_attr(
    feature = "csip_aus",
    yaserde(namespace = "csipaus: https://csipaus.org/ns")
)]
pub struct DERCapability {
    /// Bitmap indicating the DER Controls implemented by the device. See
    /// DERControlType for values.
    #[yaserde(rename = "modesSupported")]
    pub modes_supported: DERControlType,

    /// Bitmap indicating the CSIP-AUS controls implemented
    #[cfg(feature = "csip_aus")]
    #[yaserde(rename = "doeModesSupported")]
    #[yaserde(prefix = "csipaus", namespace = "csipaus: https://csipaus.org/ns")]
    pub doe_modes_supported: DOEControlType,

    /// Abnormal operating performance category as defined by IEEE 1547-2018. One
    /// of:
    /// 0 - not specified
    /// 1 - Category I
    /// 2 - Category II
    /// 3 - Category III
    /// All other values reserved.
    #[yaserde(rename = "rtgAbnormalCategory")]
    pub rtg_abnormal_category: Option<AbnormalCategory>,

    /// Maximum continuous AC current capability of the DER, in Amperes (RMS).
    #[yaserde(rename = "rtgMaxA")]
    pub rtg_max_a: Option<CurrentRMS>,

    /// Usable energy storage capacity of the DER, in AmpHours.
    #[yaserde(rename = "rtgMaxAh")]
    pub rtg_max_ah: Option<AmpereHour>,

    /// Maximum apparent power charge rating in Volt-Amperes. May differ from the
    /// maximum apparent power rating.
    #[yaserde(rename = "rtgMaxChargeRateVA")]
    pub rtg_max_charge_rate_va: Option<ApparentPower>,

    /// Maximum rate of energy transfer received by the storage DER, in Watts.
    #[yaserde(rename = "rtgMaxChargeRateW")]
    pub rtg_max_charge_rate_w: Option<ActivePower>,

    /// Maximum apparent power discharge rating in Volt-Amperes. May differ from
    /// the maximum apparent power rating.
    #[yaserde(rename = "rtgMaxDischargeRateVA")]
    pub rtg_max_discharge_rate_va: Option<ApparentPower>,

    /// Maximum rate of energy transfer delivered by the storage DER, in Watts.
    /// Required for combined generation/storage DERs (e.g. DERType == 83).
    #[yaserde(rename = "rtgMaxDischargeRateW")]
    pub rtg_max_discharge_rate_w: Option<ActivePower>,

    /// AC voltage maximum rating.
    #[yaserde(rename = "rtgMaxV")]
    pub rtg_max_v: Option<VoltageRMS>,

    /// Maximum continuous apparent power output capability of the DER, in VA.
    #[yaserde(rename = "rtgMaxVA")]
    pub rtg_max_va: Option<ApparentPower>,

    /// Maximum continuous reactive power delivered by the DER, in var.
    #[yaserde(rename = "rtgMaxVar")]
    pub rtg_max_var: Option<ReactivePower>,

    /// Maximum continuous reactive power received by the DER, in var. If absent,
    /// defaults to negative rtgMaxVar.
    #[yaserde(rename = "rtgMaxVarNeg")]
    pub rtg_max_var_neg: Option<ReactivePower>,

    /// Maximum continuous active power output capability of the DER, in watts.
    /// Represents combined generation plus storage output if DERType == 83.
    #[yaserde(rename = "rtgMaxW")]
    pub rtg_max_w: ActivePower,

    /// Maximum energy storage capacity of the DER, in WattHours.
    #[yaserde(rename = "rtgMaxWh")]
    pub rtg_max_wh: Option<WattHour>,

    /// Minimum Power Factor displacement capability of the DER when injecting
    /// reactive power (over-excited); SHALL be a positive value between 0.0
    /// (typically &gt; 0.7) and 1.0. If absent, defaults to unity.
    #[yaserde(rename = "rtgMinPFOverExcited")]
    pub rtg_min_pf_over_excited: Option<PowerFactor>,

    /// Minimum Power Factor displacement capability of the DER when absorbing
    /// reactive power (under-excited); SHALL be a positive value between 0.0
    /// (typically &gt; 0.7) and 0.9999. If absent, defaults to
    /// rtgMinPFOverExcited.
    #[yaserde(rename = "rtgMinPFUnderExcited")]
    pub rtg_min_pf_under_excited: Option<PowerFactor>,

    /// AC voltage minimum rating.
    #[yaserde(rename = "rtgMinV")]
    pub rtg_min_v: Option<VoltageRMS>,

    /// Normal operating performance category as defined by IEEE 1547-2018. One
    /// of:
    /// 0 - not specified
    /// 1 - Category A
    /// 2 - Category B
    /// All other values reserved.
    #[yaserde(rename = "rtgNormalCategory")]
    pub rtg_normal_category: Option<NormalCategory>,

    /// Specified over-excited power factor.
    #[yaserde(rename = "rtgOverExcitedPF")]
    pub rtg_over_excited_pf: Option<PowerFactor>,

    /// Active power rating in Watts at specified over-excited power factor
    /// (rtgOverExcitedPF). If present, rtgOverExcitedPF SHALL be present.
    #[yaserde(rename = "rtgOverExcitedW")]
    pub rtg_over_excited_w: Option<ActivePower>,

    /// Reactive susceptance that remains connected to the Area EPS in the cease
    /// to energize and trip state.
    #[yaserde(rename = "rtgReactiveSusceptance")]
    pub rtg_reactive_susceptance: Option<ReactiveSusceptance>,

    /// Specified under-excited power factor.
    #[yaserde(rename = "rtgUnderExcitedPF")]
    pub rtg_under_excited_pf: Option<PowerFactor>,

    /// Active power rating in Watts at specified under-excited power factor
    /// (rtgUnderExcitedPF). If present, rtgUnderExcitedPF SHALL be present.
    #[yaserde(rename = "rtgUnderExcitedW")]
    pub rtg_under_excited_w: Option<ActivePower>,

    /// AC voltage nominal rating.
    #[yaserde(rename = "rtgVNom")]
    pub rtg_v_nom: Option<VoltageRMS>,

    /// Type of DER; see DERType object
    #[yaserde(rename = "type")]
    pub _type: DERType,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum AbnormalCategory {
    #[default]
    NotSpecified = 0,
    CategoryOne = 1,
    CategoryTwo = 2,
    CategoryThree = 3,
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum NormalCategory {
    #[default]
    NotSpecified = 0,
    CategoryA = 1,
    CategoryB = 2,
}

impl Validate for DERCapability {}

/// Distributed Energy Resource (DER) control values.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "DERControlBase")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[cfg_attr(
    feature = "csip_aus",
    yaserde(namespace = "csipaus: https://csipaus.org/ns")
)]
pub struct DERControlBase {
    /// Set DER as connected (true) or disconnected (false). Used in conjunction
    /// with ramp rate when re-connecting. Implies galvanic isolation.
    #[yaserde(rename = "opModConnect")]
    pub op_mod_connect: Option<bool>,

    /// Set DER as energized (true) or de-energized (false). Used in conjunction
    /// with ramp rate when re-energizing.
    #[yaserde(rename = "opModEnergize")]
    pub op_mod_energize: Option<bool>,

    /// The opModFixedPFAbsorbW function specifies a requested fixed Power Factor
    /// (PF) setting for when active power is being absorbed. The actual
    /// displacement SHALL be within the limits established by
    /// setMinPFOverExcited and setMinPFUnderExcited. If issued simultaneously
    /// with other reactive power controls (e.g. opModFixedVar) the control
    /// resulting in least var magnitude SHOULD take precedence.
    #[yaserde(rename = "opModFixedPFAbsorbW")]
    pub op_mod_fixed_pf_absorb_w: Option<PowerFactorWithExcitation>,

    /// The opModFixedPFInjectW function specifies a requested fixed Power Factor
    /// (PF) setting for when active power is being injected. The actual
    /// displacement SHALL be within the limits established by
    /// setMinPFOverExcited and setMinPFUnderExcited. If issued simultaneously
    /// with other reactive power controls (e.g. opModFixedVar) the control
    /// resulting in least var magnitude SHOULD take precedence.
    #[yaserde(rename = "opModFixedPFInjectW")]
    pub op_mod_fixed_pf_inject_w: Option<PowerFactorWithExcitation>,

    /// The opModFixedVar function specifies the delivered or received reactive
    /// power setpoint. The context for the setpoint value is determined by
    /// refType and SHALL be one of %setMaxW, %setMaxVar, or %statVarAvail. If
    /// issued simultaneously with other reactive power controls (e.g.
    /// opModFixedPFInjectW) the control resulting in least var magnitude SHOULD
    /// take precedence.
    #[yaserde(rename = "opModFixedVar")]
    pub op_mod_fixed_var: Option<FixedVar>,

    /// The opModFixedW function specifies a requested charge or discharge mode
    /// setpoint, in %setMaxChargeRateW if negative value or %setMaxW or
    /// %setMaxDischargeRateW if positive value (in hundredths).
    #[yaserde(rename = "opModFixedW")]
    pub op_mod_fixed_w: Option<SignedPercent>,

    /// Specifies a frequency-watt operation. This operation limits active power
    /// generation or consumption when the line frequency deviates from nominal
    /// by a specified amount.
    #[yaserde(rename = "opModFreqDroop")]
    pub op_mod_freq_droop: Option<FreqDroopType>,

    /// Specify DERCurveLink for curveType == 0. The Frequency-Watt function
    /// limits active power generation or consumption when the line frequency
    /// deviates from nominal by a specified amount. The Frequency-Watt curve is
    /// specified as an array of Frequency-Watt pairs that are interpolated into
    /// a piecewise linear function with hysteresis. The x value of each pair
    /// specifies a frequency in Hz. The y value specifies a corresponding active
    /// power output in %setMaxW.
    #[yaserde(rename = "opModFreqWatt")]
    pub op_mod_freq_watt: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 1. The High Frequency Ride-Through
    /// (HFRT) function is specified by one or two duration-frequency curves that
    /// define the operating region under high frequency conditions. Each HFRT
    /// curve is specified by an array of duration-frequency pairs that will be
    /// interpolated into a piecewise linear function that defines an operating
    /// region. The x value of each pair specifies a duration (time at a given
    /// frequency in seconds). The y value of each pair specifies a frequency, in
    /// Hz. This control specifies the "may trip" region.
    #[yaserde(rename = "opModHFRTMayTrip")]
    pub op_mod_hfrt_may_trip: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 2. The High Frequency Ride-Through
    /// (HFRT) function is specified by a duration-frequency curve that defines
    /// the operating region under high frequency conditions. Each HFRT curve is
    /// specified by an array of duration-frequency pairs that will be
    /// interpolated into a piecewise linear function that defines an operating
    /// region. The x value of each pair specifies a duration (time at a given
    /// frequency in seconds). The y value of each pair specifies a frequency, in
    /// Hz. This control specifies the "must trip" region.
    #[yaserde(rename = "opModHFRTMustTrip")]
    pub op_mod_hfrt_must_trip: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 3. The High Voltage Ride-Through
    /// (HVRT) function is specified by one, two, or three duration-volt curves
    /// that define the operating region under high voltage conditions. Each HVRT
    /// curve is specified by an array of duration-volt pairs that will be
    /// interpolated into a piecewise linear function that defines an operating
    /// region. The x value of each pair specifies a duration (time at a given
    /// voltage in seconds). The y value of each pair specifies an effective
    /// percentage voltage, defined as ((locally measured voltage - setVRefOfs /
    /// setVRef). This control specifies the "may trip" region.
    #[yaserde(rename = "opModHVRTMayTrip")]
    pub op_mod_hvrt_may_trip: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 4. The High Voltage Ride-Through
    /// (HVRT) function is specified by duration-volt curves that define the
    /// operating region under high voltage conditions. Each HVRT curve is
    /// specified by an array of duration-volt pairs that will be interpolated
    /// into a piecewise linear function that defines an operating region. The x
    /// value of each pair specifies a duration (time at a given voltage in
    /// seconds). The y value of each pair specifies an effective percent
    /// voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef).
    /// This control specifies the "momentary cessation" region.
    #[yaserde(rename = "opModHVRTMomentaryCessation")]
    pub op_mod_hvrt_momentary_cessation: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 5. The High Voltage Ride-Through
    /// (HVRT) function is specified by duration-volt curves that define the
    /// operating region under high voltage conditions. Each HVRT curve is
    /// specified by an array of duration-volt pairs that will be interpolated
    /// into a piecewise linear function that defines an operating region. The x
    /// value of each pair specifies a duration (time at a given voltage in
    /// seconds). The y value of each pair specifies an effective percent
    /// voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef).
    /// This control specifies the "must trip" region.
    #[yaserde(rename = "opModHVRTMustTrip")]
    pub op_mod_hvrt_must_trip: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 6. The Low Frequency Ride-Through
    /// (LFRT) function is specified by one or two duration-frequency curves that
    /// define the operating region under low frequency conditions. Each LFRT
    /// curve is specified by an array of duration-frequency pairs that will be
    /// interpolated into a piecewise linear function that defines an operating
    /// region. The x value of each pair specifies a duration (time at a given
    /// frequency in seconds). The y value of each pair specifies a frequency, in
    /// Hz. This control specifies the "may trip" region.
    #[yaserde(rename = "opModLFRTMayTrip")]
    pub op_mod_lfrt_may_trip: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 7. The Low Frequency Ride-Through
    /// (LFRT) function is specified by a duration-frequency curve that defines
    /// the operating region under low frequency conditions. Each LFRT curve is
    /// specified by an array of duration-frequency pairs that will be
    /// interpolated into a piecewise linear function that defines an operating
    /// region. The x value of each pair specifies a duration (time at a given
    /// frequency in seconds). The y value of each pair specifies a frequency, in
    /// Hz. This control specifies the "must trip" region.
    #[yaserde(rename = "opModLFRTMustTrip")]
    pub op_mod_lfrt_must_trip: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 8. The Low Voltage Ride-Through
    /// (LVRT) function is specified by one, two, or three duration-volt curves
    /// that define the operating region under low voltage conditions. Each LVRT
    /// curve is specified by an array of duration-volt pairs that will be
    /// interpolated into a piecewise linear function that defines an operating
    /// region. The x value of each pair specifies a duration (time at a given
    /// voltage in seconds). The y value of each pair specifies an effective
    /// percent voltage, defined as ((locally measured voltage - setVRefOfs) /
    /// setVRef). This control specifies the "may trip" region.
    #[yaserde(rename = "opModLVRTMayTrip")]
    pub op_mod_lvrt_may_trip: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 9. The Low Voltage Ride-Through
    /// (LVRT) function is specified by duration-volt curves that define the
    /// operating region under low voltage conditions. Each LVRT curve is
    /// specified by an array of duration-volt pairs that will be interpolated
    /// into a piecewise linear function that defines an operating region. The x
    /// value of each pair specifies a duration (time at a given voltage in
    /// seconds). The y value of each pair specifies an effective percent
    /// voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef).
    /// This control specifies the "momentary cessation" region.
    #[yaserde(rename = "opModLVRTMomentaryCessation")]
    pub op_mod_lvrt_momentary_cessation: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 10. The Low Voltage Ride-Through
    /// (LVRT) function is specified by duration-volt curves that define the
    /// operating region under low voltage conditions. Each LVRT curve is
    /// specified by an array of duration-volt pairs that will be interpolated
    /// into a piecewise linear function that defines an operating region. The x
    /// value of each pair specifies a duration (time at a given voltage in
    /// seconds). The y value of each pair specifies an effective percent
    /// voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef).
    /// This control specifies the "must trip" region.
    #[yaserde(rename = "opModLVRTMustTrip")]
    pub op_mod_lvrt_must_trip: Option<DERCurveLink>,

    /// The opModMaxLimW function sets the maximum active power generation level
    /// at the electrical coupling point as a percentage of set capacity
    /// (%setMaxW, in hundredths). This limitation may be met e.g. by reducing PV
    /// output or by using excess PV output to charge associated storage.
    #[yaserde(rename = "opModMaxLimW")]
    pub op_mod_max_lim_w: Option<Percent>,

    /// Target reactive power, in var. This control is likely to be more useful
    /// for aggregators, as individual DERs may not be able to maintain a target
    /// setting.
    #[yaserde(rename = "opModTargetVar")]
    pub op_mod_target_var: Option<ReactivePower>,

    /// Target output power, in Watts. This control is likely to be more useful
    /// for aggregators, as individual DERs may not be able to maintain a target
    /// setting.
    #[yaserde(rename = "opModTargetW")]
    pub op_mod_target_w: Option<ActivePower>,

    /// Specify DERCurveLink for curveType == 11. The static volt-var function
    /// provides over- or under-excited var compensation as a function of
    /// measured voltage. The volt-var curve is specified as an array of volt-var
    /// pairs that are interpolated into a piecewise linear function with
    /// hysteresis. The x value of each pair specifies an effective percent
    /// voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef)
    /// and SHOULD support a domain of at least 0 - 135. If VRef is present in
    /// DERCurve, then the x value of each pair is additionally multiplied by
    /// (VRef / 10000). The y value specifies a target var output interpreted as
    /// a signed percentage (-100 to 100). The meaning of the y value is
    /// determined by yRefType and must be one of %setMaxW, %setMaxVar, or
    /// %statVarAvail.
    #[yaserde(rename = "opModVoltVar")]
    pub op_mod_volt_var: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 12. The Volt-Watt reduces active
    /// power output as a function of measured voltage. The Volt-Watt curve is
    /// specified as an array of Volt-Watt pairs that are interpolated into a
    /// piecewise linear function with hysteresis. The x value of each pair
    /// specifies an effective percent voltage, defined as ((locally measured
    /// voltage - setVRefOfs) / setVRef) and SHOULD support a domain of at least
    /// 0 - 135. The y value specifies an active power output interpreted as a
    /// percentage (0 - 100). The meaning of the y value is determined by
    /// yRefType and must be one of %setMaxW or %statWAvail.
    #[yaserde(rename = "opModVoltWatt")]
    pub op_mod_volt_watt: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 13. The Watt-PF function varies
    /// Power Factor (PF) as a function of delivered active power. The Watt-PF
    /// curve is specified as an array of Watt-PF coordinates that are
    /// interpolated into a piecewise linear function with hysteresis. The x
    /// value of each pair specifies a watt setting in %setMaxW, (0 - 100). The
    /// PF output setting is a signed displacement in y value (PF sign SHALL be
    /// interpreted according to the EEI convention, where unity PF is considered
    /// unsigned). These settings are not expected to be updated very often
    /// during the life of the installation, therefore only a single curve is
    /// required. If issued simultaneously with other reactive power controls
    /// (e.g. opModFixedPFInjectW) the control resulting in least var magnitude
    /// SHOULD take precedence.
    #[yaserde(rename = "opModWattPF")]
    pub op_mod_watt_pf: Option<DERCurveLink>,

    /// Specify DERCurveLink for curveType == 14. The Watt-Var function varies
    /// vars as a function of delivered active power. The Watt-Var curve is
    /// specified as an array of Watt-Var pairs that are interpolated into a
    /// piecewise linear function with hysteresis. The x value of each pair
    /// specifies a watt setting in %setMaxW, (0-100). The y value specifies a
    /// target var output interpreted as a signed percentage (-100 to 100). The
    /// meaning of the y value is determined by yRefType and must be one of
    /// %setMaxW, %setMaxVar, or %statVarAvail.
    #[yaserde(rename = "opModWattVar")]
    pub op_mod_watt_var: Option<DERCurveLink>,

    /// This is the constraint on the imported active power at the connection point.
    #[cfg(feature = "csip_aus")]
    #[yaserde(rename = "opModImpLimW")]
    #[yaserde(prefix = "csipaus", namespace = "csipaus: https://csipaus.org/ns")]
    pub op_mod_imp_lim_w: Option<ActivePower>,

    /// This is the constraint on the exported active power at the connection point.
    #[cfg(feature = "csip_aus")]
    #[yaserde(rename = "opModExpLimW")]
    #[yaserde(prefix = "csipaus", namespace = "csipaus: https://csipaus.org/ns")]
    pub op_mod_exp_lim_w: Option<ActivePower>,

    /// This is a constraint on the maxium allowable discharge rate, in Watts,
    /// specifically for a single physical device (or aggregation of devices,
    /// excluding uncontrolled devices) such as an EV charge station.
    #[cfg(feature = "csip_aus")]
    #[yaserde(rename = "opModGenLimW")]
    #[yaserde(prefix = "csipaus", namespace = "csipaus: https://csipaus.org/ns")]
    pub op_mod_gen_lim_w: Option<ActivePower>,

    /// This is a constraint on the maximum allowable charge rate, in Watts,
    /// specifically for a single physical device (or aggregation of devices,
    /// excluding uncontrolled devices) such as an EV charge station.
    #[cfg(feature = "csip_aus")]
    #[yaserde(rename = "opModLoadLimW")]
    #[yaserde(prefix = "csipaus", namespace = "csipaus: https://csipaus.org/ns")]
    pub op_mod_load_lim_w: Option<ActivePower>,

    /// Requested ramp time, in hundredths of a second, for the device to
    /// transition from the current DERControl mode setting(s) to the new mode
    /// setting(s). If absent, use default ramp rate (setGradW). Resolution is
    /// 1/100 sec.
    #[yaserde(rename = "rampTms")]
    pub ramp_tms: Option<Uint16>,
}

impl DERControlBase {
    /// Determine if two DERControlBase instances target the same set of controls by whether they contain the same optional fields.
    pub fn same_target(&self, other: &Self) -> bool {
        #[cfg(feature = "csip_aus")]
        let extensions = {
            self.op_mod_imp_lim_w.is_some() == other.op_mod_imp_lim_w.is_some()
                && self.op_mod_exp_lim_w.is_some() == other.op_mod_exp_lim_w.is_some()
                && self.op_mod_gen_lim_w.is_some() == other.op_mod_gen_lim_w.is_some()
                && self.op_mod_load_lim_w.is_some() == other.op_mod_load_lim_w.is_some()
        };
        #[cfg(not(feature = "csip_aus"))]
        let extensions = true;
        self.op_mod_connect.is_some() == other.op_mod_connect.is_some()
            && self.op_mod_energize.is_some() == other.op_mod_energize.is_some()
            && self.op_mod_fixed_pf_absorb_w.is_some() == other.op_mod_fixed_pf_absorb_w.is_some()
            && self.op_mod_fixed_pf_inject_w.is_some() == other.op_mod_fixed_pf_inject_w.is_some()
            && self.op_mod_fixed_var.is_some() == other.op_mod_fixed_var.is_some()
            && self.op_mod_fixed_w.is_some() == other.op_mod_fixed_w.is_some()
            && self.op_mod_freq_droop.is_some() == other.op_mod_freq_droop.is_some()
            && self.op_mod_freq_watt.is_some() == other.op_mod_freq_watt.is_some()
            && self.op_mod_hfrt_may_trip.is_some() == other.op_mod_hfrt_may_trip.is_some()
            && self.op_mod_hfrt_must_trip.is_some() == other.op_mod_hfrt_must_trip.is_some()
            && self.op_mod_hvrt_may_trip.is_some() == other.op_mod_hvrt_may_trip.is_some()
            && self.op_mod_hvrt_momentary_cessation.is_some()
                == other.op_mod_hvrt_momentary_cessation.is_some()
            && self.op_mod_hvrt_must_trip.is_some() == other.op_mod_hvrt_must_trip.is_some()
            && self.op_mod_lfrt_may_trip.is_some() == other.op_mod_lfrt_may_trip.is_some()
            && self.op_mod_lfrt_must_trip.is_some() == other.op_mod_lfrt_must_trip.is_some()
            && self.op_mod_lvrt_may_trip.is_some() == other.op_mod_lvrt_may_trip.is_some()
            && self.op_mod_lvrt_momentary_cessation.is_some()
                == other.op_mod_lvrt_momentary_cessation.is_some()
            && self.op_mod_lvrt_must_trip.is_some() == other.op_mod_lvrt_must_trip.is_some()
            && self.op_mod_max_lim_w.is_some() == other.op_mod_max_lim_w.is_some()
            && self.op_mod_target_var.is_some() == other.op_mod_target_var.is_some()
            && self.op_mod_target_w.is_some() == other.op_mod_target_w.is_some()
            && self.op_mod_volt_var.is_some() == other.op_mod_volt_var.is_some()
            && self.op_mod_volt_watt.is_some() == other.op_mod_volt_watt.is_some()
            && self.op_mod_watt_pf.is_some() == other.op_mod_watt_pf.is_some()
            && self.op_mod_watt_var.is_some() == other.op_mod_watt_var.is_some()
            && self.ramp_tms.is_some() == other.ramp_tms.is_some()
            && extensions
    }
}

impl Validate for DERControlBase {}

/// Distributed Energy Resource (DER) time/event-based control.
#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SERandomizableEvent,
    SEEvent,
    SERespondableSubscribableIdentifiedObject,
    SEIdentifiedObject,
    SESubscribableResource,
    SERespondableResource,
    SEResource,
)]
#[yaserde(rename = "DERControl")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERControl {
    #[yaserde(rename = "DERControlBase")]
    pub der_control_base: DERControlBase,

    /// Specifies the bitmap indicating the categories of devices that SHOULD
    /// respond. Devices SHOULD ignore events that do not indicate their device
    /// category. If not present, all devices SHOULD respond.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: Option<DeviceCategoryType>,

    /// Number of seconds boundary inside which a random value must be selected
    /// to be applied to the associated interval duration, to avoid sudden
    /// synchronized demand changes. If related to price level changes, sign may
    /// be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    /// default.
    #[yaserde(rename = "randomizeDuration")]
    pub randomize_duration: Option<OneHourRangeType>,

    /// Number of seconds boundary inside which a random value must be selected
    /// to be applied to the associated interval start time, to avoid sudden
    /// synchronized demand changes. If related to price level changes, sign may
    /// be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    /// default.
    #[yaserde(rename = "randomizeStart")]
    pub randomize_start: Option<OneHourRangeType>,

    /// The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    /// The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

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

    /// Indicates whether or not subscriptions are supported for this resource,
    /// and whether or not conditional (thresholds) are supported. If not
    /// specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    /// A reference to the response resource address (URI). Required on a
    /// response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    /// Indicates whether or not a response is required upon receipt, creation or
    /// update of this resource. Responses shall be posted to the collection
    /// specified in "replyTo".
    /// If the resource has a deviceCategory field, devices that match one or
    /// more of the device types indicated in deviceCategory SHALL respond
    /// according to the rules listed below. If the category does not match, the
    /// device SHALL NOT respond. If the resource does not have a deviceCategory
    /// field, a device receiving the resource SHALL respond according to the
    /// rules listed below.
    /// Value encoded as hex according to the following bit assignments, any
    /// combination is possible.
    /// See Table 27 for the list of appropriate Response status codes to be sent
    /// for these purposes.
    /// 0 - End device shall indicate that message was received
    /// 1 - End device shall indicate specific response.
    /// 2 - End user / customer response is required.
    /// All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<ResponseRequired>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for DERControl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DERControl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - primacy (ascending)
        match self.interval.start.cmp(&other.interval.start) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - creationTime (descending)
        match self.creation_time.cmp(&other.creation_time).reverse() {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Tertiary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for DERControl {}

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
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[yaserde(rename = "DERControlList")]
pub struct DERControlList {
    #[yaserde(rename = "DERControl")]
    pub der_control: Vec<DERControl>,

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

impl Validate for DERControlList {}

bitflags! {
    /// Control modes supported by the DER.  Bit positions SHALL be defined as follows:
    ///
    /// 0 - Charge mode
    ///
    /// 1 - Discharge mode
    ///
    /// 2 - opModConnect (Connect / Disconnect - implies galvanic isolation)
    ///
    /// 3 - opModEnergize (Energize / De-Energize)
    ///
    /// 4 - opModFixedPFAbsorbW (Fixed Power Factor Setpoint when absorbing active power)
    ///
    /// 5 - opModFixedPFInjectW (Fixed Power Factor Setpoint when injecting active power)
    ///
    /// 6 - opModFixedVar (Reactive Power Setpoint)
    ///
    /// 7 - opModFixedW (Charge / Discharge Setpoint)
    ///
    /// 8 - opModFreqDroop (Frequency-Watt Parameterized Mode)
    ///
    /// 9 - opModFreqWatt (Frequency-Watt Curve Mode)
    ///
    /// 10 - opModHFRTMayTrip (High Frequency Ride Through, May Trip Mode)
    ///
    /// 11 - opModHFRTMustTrip (High Frequency Ride Through, Must Trip Mode)
    ///
    /// 12 - opModHVRTMayTrip (High Voltage Ride Through, May Trip Mode)
    ///
    /// 13 - opModHVRTMomentaryCessation (High Voltage Ride Through, Momentary Cessation Mode)
    ///
    /// 14 - opModHVRTMustTrip (High Voltage Ride Through, Must Trip Mode)
    ///
    /// 15 - opModLFRTMayTrip (Low Frequency Ride Through, May Trip Mode)
    ///
    /// 16 - opModLFRTMustTrip (Low Frequency Ride Through, Must Trip Mode)
    ///
    /// 17 - opModLVRTMayTrip (Low Voltage Ride Through, May Trip Mode)
    ///
    /// 18 - opModLVRTMomentaryCessation (Low Voltage Ride Through, Momentary Cessation Mode)
    ///
    /// 19 - opModLVRTMustTrip (Low Voltage Ride Through, Must Trip Mode)
    ///
    /// 20 - opModMaxLimW (Maximum Active Power)
    ///
    /// 21 - opModTargetVar (Target Reactive Power)
    ///
    /// 22 - opModTargetW (Target Active Power)
    ///
    /// 23 - opModVoltVar (Volt-Var Mode)
    ///
    /// 24 - opModVoltWatt (Volt-Watt Mode)
    ///
    /// 25 - opModWattPF (Watt-PowerFactor Mode)
    ///
    /// 26 - opModWattVar (Watt-Var Mode)
    ///
    /// All other values reserved.
    ///
    #[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, HexBinaryYaSerde)]
    pub struct DERControlType: u32 { // HexBinary32
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
        const OpModLFRTMustTrip = 65536;
        const OpModLVRTMayTrip = 131072;
        const OpModLVRTMomentaryCessation = 262144;
        const OpModLVRTMustTrip = 524288;
        const OpModMaxLimW = 1048576;
        const OpModTargetVar = 2097152;
        const OpModTargetW = 4194304;
        const OpModVoltVar = 8388608;
        const OpModVoltWatt = 16777216;
        const OpModWattPF = 33554432;
        const OpModWattVar = 67108864;
    }
}

impl Validate for DERControlType {}

#[cfg(feature = "csip_aus")]
bitflags! {
    #[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, HexBinaryYaSerde)]
    pub struct DOEControlType: u16 { // HexBinary 16
        const OpModExpLimW = 1;
        const OpModImpLimW = 2;
        const OpModGenLimW = 4;
        const OpModLoadLimW = 8;
    }
}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "DERCurve")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERCurve {
    /// If the curveType is opModVoltVar, then this field MAY be present. If the
    /// curveType is not opModVoltVar, then this field SHALL NOT be present.
    /// Enable/disable autonomous vRef adjustment. When enabled, the Volt-Var
    /// curve characteristic SHALL be adjusted autonomously as vRef changes and
    /// autonomousVRefTimeConstant SHALL be present. If a DER is able to support
    /// Volt-Var mode but is unable to support autonomous vRef adjustment, then
    /// the DER SHALL execute the curve without autonomous vRef adjustment. If
    /// not specified, then the value is false.
    #[yaserde(rename = "autonomousVRefEnable")]
    pub autonomous_v_ref_enable: Option<bool>,

    /// If the curveType is opModVoltVar, then this field MAY be present. If the
    /// curveType is not opModVoltVar, then this field SHALL NOT be present.
    /// Adjustment range for vRef time constant, in hundredths of a second.
    #[yaserde(rename = "autonomousVRefTimeConstant")]
    pub autonomous_v_ref_time_constant: Option<Uint32>,

    /// The time at which the object was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    /// Data point values for defining a curve or schedule
    /// Max size: 10
    /// TODO: Validate this max
    #[yaserde(rename = "CurveData")]
    pub curve_data: Vec<CurveData>,

    /// Specifies the associated curve-based control mode.
    #[yaserde(rename = "curveType")]
    pub curve_type: DERCurveType,

    /// Open loop response time, the time to ramp up to 90% of the new target in
    /// response to the change in voltage, in hundredths of a second. Resolution
    /// is 1/100 sec. A value of 0 is used to mean no limit. When not present,
    /// the device SHOULD follow its default behavior.
    #[yaserde(rename = "openLoopTms")]
    pub open_loop_tms: Option<Uint16>,

    /// Decreasing ramp rate, interpreted as a percentage change in output
    /// capability limit per second (e.g. %setMaxW / sec). Resolution is in
    /// hundredths of a percent/second. A value of 0 means there is no limit. If
    /// absent, ramp rate defaults to setGradW.
    #[yaserde(rename = "rampDecTms")]
    pub ramp_dec_tms: Option<Uint16>,

    /// Increasing ramp rate, interpreted as a percentage change in output
    /// capability limit per second (e.g. %setMaxW / sec). Resolution is in
    /// hundredths of a percent/second. A value of 0 means there is no limit. If
    /// absent, ramp rate defaults to rampDecTms.
    #[yaserde(rename = "rampIncTms")]
    pub ramp_inc_tms: Option<Uint16>,

    /// The configuration parameter for a low-pass filter, PT1 is a time, in
    /// hundredths of a second, in which the filter will settle to 95% of a step
    /// change in the input value. Resolution is 1/100 sec.
    #[yaserde(rename = "rampPT1Tms")]
    pub ramp_pt1_tms: Option<Uint16>,

    /// If the curveType is opModVoltVar, then this field MAY be present. If the
    /// curveType is not opModVoltVar, then this field SHALL NOT be present. The
    /// nominal AC voltage (RMS) adjustment to the voltage curve points for
    /// Volt-Var curves.
    #[yaserde(rename = "vRef")]
    pub v_ref: Option<Percent>,

    /// Exponent for X-axis value.
    #[yaserde(rename = "xMultiplier")]
    pub x_multiplier: PowerOfTenMultiplierType,

    /// Exponent for Y-axis value.
    #[yaserde(rename = "yMultiplier")]
    pub y_multiplier: PowerOfTenMultiplierType,

    /// The Y-axis units context.
    #[yaserde(rename = "yRefType")]
    pub y_ref_type: DERUnitRefType,

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

impl PartialOrd for DERCurve {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DERCurve {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - creationTime (descending)
        match self.creation_time.cmp(&other.creation_time).reverse() {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for DERCurve {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "DERCurveList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERCurveList {
    #[yaserde(rename = "DERCurve")]
    pub der_curve: Vec<DERCurve>,

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

impl Validate for DERCurveList {}

/// Data point values for defining a curve or schedule
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "CurveData")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CurveData {
    /// If yvalue is Power Factor, then this field SHALL be present. If yvalue is
    /// not Power Factor, then this field SHALL NOT be present.
    /// True when DER is absorbing reactive power (under-excited), false
    /// when DER is injecting reactive power (over-excited).
    #[yaserde(rename = "excitation")]
    pub excitation: Option<bool>,

    /// The data value of the X-axis (independent) variable, depending on the
    /// curve type. See definitions in DERControlBase for further information.
    #[yaserde(rename = "xvalue")]
    pub xvalue: Int32,

    /// The data value of the Y-axis (dependent) variable, depending on the curve
    /// type. See definitions in DERControlBase for further information. If
    /// yvalue is Power Factor, the excitation field SHALL be present and yvalue
    /// SHALL be a positive value. If yvalue is not Power Factor, the excitation
    /// field SHALL NOT be present.
    #[yaserde(rename = "yvalue")]
    pub yvalue: Int32,
}

impl Validate for CurveData {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "DERCurveType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum DERCurveType {
    #[default]
    OpModFreqWatt = 0, // (Frequency-Watt Curve Mode)
    OpModHfrtmayTrip = 1,  // (High Frequency Ride Through, May Trip Mode)
    OpModHfrtmustTrip = 2, // High Frequency Ride Through, Must Trip Mode)
    OpModHvrtmayTrip = 3,  // (High Voltage Ride Through, May Trip Mode)
    OpModHvrtmomentaryCessation = 4, // (High Voltage Ride Through, Momentary Cessation Mode)
    OpModHvrtmustTrip = 5, // (High Voltage Ride Through, Must Trip Mode)
    OpModLfrtmayTrip = 6,  // (Low Frequency Ride Through, May Trip Mode)
    OpModLfrtmustTrip = 7, // (Low Frequency Ride Through, Must Trip Mode)
    OpModLvrtmayTrip = 8,  // (Low Voltage Ride Through, May Trip Mode)
    OpModLvrtmomentaryCessation = 9, // (Low Voltage Ride Through, Momentary Cessation Mode)
    OpModLvrtmustTrip = 10, // (Low Voltage Ride Through, Must Trip Mode)
    OpModVoltVar = 11,     // (Volt-Var Mode)
    OpModVoltWatt = 12,    // (Volt-Watt Mode)
    OpModWattPf = 13,      // (Watt-PowerFactor Mode)
    OpModWattVar = 14,     // (Watt-Var Mode)
}

impl Validate for DERCurveType {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SESubscribableIdentifiedObject,
    SEIdentifiedObject,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "DERProgram")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERProgram {
    #[yaserde(rename = "ActiveDERControlListLink")]
    pub active_der_control_list_link: Option<ActiveDERControlListLink>,

    #[yaserde(rename = "DefaultDERControlLink")]
    pub default_der_control_link: Option<DefaultDERControlLink>,

    #[yaserde(rename = "DERControlListLink")]
    pub der_control_list_link: Option<DERControlListLink>,

    #[yaserde(rename = "DERCurveListLink")]
    pub der_curve_list_link: Option<DERCurveListLink>,

    /// Indicates the relative primacy of the provider of this Program.
    #[yaserde(rename = "primacy")]
    pub primacy: PrimacyType,

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

impl PartialOrd for DERProgram {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DERProgram {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - primacy (ascending)
        match self.primacy.cmp(&other.primacy) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for DERProgram {}

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
#[yaserde(rename = "DERProgramList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERProgramList {
    #[yaserde(rename = "DERProgram")]
    pub der_program: Vec<DERProgram>,

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

impl Validate for DERProgramList {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Debug,
    Clone,
    YaSerialize,
    YaDeserialize,
    SESubscribableResource,
    SEResource,
)]
#[yaserde(rename = "DERStatus")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DERStatus {
    /// Bitmap indicating the status of DER alarms (see DER LogEvents for more
    /// details).
    /// 0 - DER_FAULT_OVER_CURRENT
    /// 1 - DER_FAULT_OVER_VOLTAGE
    /// 2 - DER_FAULT_UNDER_VOLTAGE
    /// 3 - DER_FAULT_OVER_FREQUENCY
    /// 4 - DER_FAULT_UNDER_FREQUENCY
    /// 5 - DER_FAULT_VOLTAGE_IMBALANCE
    /// 6 - DER_FAULT_CURRENT_IMBALANCE
    /// 7 - DER_FAULT_EMERGENCY_LOCAL
    /// 8 - DER_FAULT_EMERGENCY_REMOTE
    /// 9 - DER_FAULT_LOW_POWER_INPUT
    /// 10 - DER_FAULT_PHASE_ROTATION
    /// 11-31 - Reserved
    #[yaserde(rename = "alarmStatus")]
    pub alarm_status: Option<DERAlarmStatus>,

    /// Connect/status value for generator DER.
    /// See ConnectStatusType for values.
    #[yaserde(rename = "genConnectStatus")]
    pub gen_connect_status: Option<ConnectStatusType>,

    /// DER InverterStatus/value.
    /// See InverterStatusType for values.
    #[yaserde(rename = "inverterStatus")]
    pub inverter_status: Option<InverterStatusType>,

    /// The local control mode status.
    /// See LocalControlModeStatusType for values.
    #[yaserde(rename = "localControlModeStatus")]
    pub local_control_mode_status: Option<LocalControlModeStatusType>,

    /// Manufacturer status code.
    #[yaserde(rename = "manufacturerStatus")]
    pub manufacturer_status: Option<ManufacturerStatusType>,

    /// Operational mode currently in use.
    /// See OperationalModeStatusType for values.
    #[yaserde(rename = "operationalModeStatus")]
    pub operational_mode_status: Option<OperationalModeStatusType>,

    /// The timestamp when the current status was last updated.
    #[yaserde(rename = "readingTime")]
    pub reading_time: TimeType,

    /// State of charge status.
    /// See StateOfChargeStatusType for values.
    #[yaserde(rename = "stateOfChargeStatus")]
    pub state_of_charge_status: Option<StateOfChargeStatusType>,

    /// Storage mode status.
    /// See StorageModeStatusType for values.
    #[yaserde(rename = "storageModeStatus")]
    pub storage_mode_status: Option<StorageModeStatusType>,

    /// Connect/status value for storage DER.
    /// See ConnectStatusType for values.
    #[yaserde(rename = "storConnectStatus")]
    pub stor_connect_status: Option<ConnectStatusType>,

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

bitflags! {
    #[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, HexBinaryYaSerde)]
    pub struct DERAlarmStatus: u32 { // HexBinary32
        const DER_FAULT_OVER_CURRENT = 1;
        const DER_FAULT_OVER_VOLTAGE = 2;
        const DER_FAULT_UNDER_VOLTAGE = 4;
        const DER_FAULT_OVER_FREQUENCY = 8;
        const DER_FAULT_UNDER_FREQUENCY = 16;
        const DER_FAULT_VOLTAGE_IMBALANCE = 32;
        const DER_FAULT_CURRENT_IMBALANCE = 64;
        const DER_FAULT_EMERGENCY_LOCAL = 128;
        const DER_FAULT_EMERGENCY_REMOTE = 256;
        const DER_FAULT_LOW_POWER_INPUT = 512;
        const DER_FAULT_PHASE_ROTATION = 1024;
    }
}

impl Validate for DERStatus {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "DERUnitRefType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum DERUnitRefType {
    /// Specifies context for interpreting percent values:
    #[default]
    NotApplicable = 0,
    SetMaxW = 1,
    SetMaxVar = 2,
    StatVarAvail = 3,
    SetEffectiveV = 4,
    SetMaxChargeRateW = 5,
    SetMaxDischargeRateW = 6,
    StatWAvail = 7,
    // 8-255 Reserved
}

impl Validate for DERUnitRefType {}

/// Average flow of charge through a conductor.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "CurrentRMS")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CurrentRMS {
    /// Specifies exponent of value.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    /// Value in amperes RMS (uom 5)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for CurrentRMS {}

/// Abstract type for specifying a fixed-point value without a given unit of
/// measure.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "FixedPointType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FixedPointType {
    /// Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    /// Dimensionless value
    #[yaserde(rename = "value")]
    pub value: Int16,
}

impl Validate for FixedPointType {}

/// Abstract type for specifying an unsigned fixed-point value without a given
/// unit of measure.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "UnsignedFixedPointType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UnsignedFixedPointType {
    /// Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    /// Dimensionless value
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for UnsignedFixedPointType {}

/// The active (real) power P (in W) is the product of root-mean-square (RMS)
/// voltage, RMS current, and cos(theta) where theta is the phase angle of
/// current relative to voltage. It is the primary measure of the rate of flow of
/// energy.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ActivePower")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActivePower {
    /// Specifies exponent for uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    /// Value in watts (uom 38)
    #[yaserde(rename = "value")]
    pub value: Int16,
}

impl Validate for ActivePower {}

/// Available electric charge
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "AmpereHour")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AmpereHour {
    /// Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    /// Value in ampere-hours (uom 106)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for AmpereHour {}

/// The apparent power S (in VA) is the product of root mean square (RMS) voltage
/// and RMS current.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ApparentPower")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ApparentPower {
    /// Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    /// Value in volt-amperes (uom 61)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for ApparentPower {}

/// The reactive power Q (in var) is the product of root mean square (RMS)
/// voltage, RMS current, and sin(theta) where theta is the phase angle of
/// current relative to voltage.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ReactivePower")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReactivePower {
    /// Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    /// Value in volt-amperes reactive (var) (uom 63)
    #[yaserde(rename = "value")]
    pub value: Int16,
}

impl Validate for ReactivePower {}

/// Reactive susceptance
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ReactiveSusceptance")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReactiveSusceptance {
    /// Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    /// Value in siemens (uom 53)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for ReactiveSusceptance {}

/// Specifies a setpoint for Displacement Power Factor, the ratio between
/// apparent and active powers at the fundamental frequency (e.g. 60 Hz).
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "PowerFactor")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PowerFactor {
    /// Significand of an unsigned value of cos(theta) between 0 and 1.0. E.g. a
    /// value of 0.95 may be specified as a displacement of 950 and a multiplier
    /// of -3.
    #[yaserde(rename = "displacement")]
    pub displacement: Uint16,

    /// Specifies exponent of 'displacement'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,
}

impl Validate for PowerFactor {}

/// Specifies a setpoint for Displacement Power Factor, the ratio between
/// apparent and active powers at the fundamental frequency (e.g. 60 Hz) and
/// includes an excitation flag.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "PowerFactorWithExcitation")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PowerFactorWithExcitation {
    /// Significand of an unsigned value of cos(theta) between 0 and 1.0. E.g. a
    /// value of 0.95 may be specified as a displacement of 950 and a multiplier
    /// of -3.
    #[yaserde(rename = "displacement")]
    pub displacement: Uint16,

    /// True when DER is absorbing reactive power (under-excited), false
    /// when DER is injecting reactive power (over-excited).
    #[yaserde(rename = "excitation")]
    pub excitation: bool,

    /// Specifies exponent of 'displacement'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,
}

impl Validate for PowerFactorWithExcitation {}

/// Specifies a signed setpoint for reactive power.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "FixedVar")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FixedVar {
    /// Indicates whether to interpret 'value' as %setMaxVar or %statVarAvail.
    #[yaserde(rename = "refType")]
    pub ref_type: DERUnitRefType,

    /// Specify a signed setpoint for reactive power in % (see 'refType' for
    /// context).
    #[yaserde(rename = "value")]
    pub value: SignedPercent,
}

impl Validate for FixedVar {}

/// Active (real) energy
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "WattHour")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct WattHour {
    /// Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    /// Value in watt-hours (uom 72)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for WattHour {}

/// Average electric potential difference between two points.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "VoltageRMS")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct VoltageRMS {
    /// Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    /// Value in volts RMS (uom 29)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for VoltageRMS {}

/// DER ConnectStatus value (bitmap):
///
/// 0 - Connected
///
/// 1 - Available
///
/// 2 - Operating
///
/// 3 - Test
///
/// 4 - Fault / Error
///
/// All other values reserved.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ConnectStatusType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ConnectStatusType {
    /// The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    /// The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: ConnectStatusValue,
}

bitflags! {
    #[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, HexBinaryYaSerde)]
    pub struct ConnectStatusValue: u8 { // HexBinary8
        const Connected = 1;
        const Available = 2;
        const Operating = 4;
        const Test = 8;
        const Error = 16;
    }
}

impl Validate for ConnectStatusType {}

/// DER InverterStatus value:
/// 0 - N/A
/// 1 - off
/// 2 - sleeping (auto-shutdown) or DER is at low output power/voltage
/// 3 - starting up or ON but not producing power
/// 4 - tracking MPPT power point
/// 5 - forced power reduction/derating
/// 6 - shutting down
/// 7 - one or more faults exist
/// 8 - standby (service on unit) - DER may be at high output voltage/power
/// 9 - test mode
/// 10 - as defined in manufacturer status
/// All other values reserved.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "InverterStatusType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct InverterStatusType {
    /// The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    /// The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: InverterStatusValue,
}

impl Validate for InverterStatusType {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum InverterStatusValue {
    #[default]
    NotApplicable = 0,
    Off = 1,
    SleepingOrLowPower = 2,
    StartingUpOrNoProduction = 3,
    TrackingMPPTPP = 4,
    ForcedPowerReduction = 5,
    ShuttingDown = 6,
    FaultsExist = 7,
    Standby = 8,
    TestMode = 9,
    ManufacturerStatus = 10,
}

/// DER LocalControlModeStatus/value:
/// 0 – local control
/// 1 – remote control
/// All other values reserved.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "LocalControlModeStatusType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LocalControlModeStatusType {
    /// The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    /// The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: Uint8,
}

impl Validate for LocalControlModeStatusType {}

/// DER ManufacturerStatus/value: String data type
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ManufacturerStatusType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ManufacturerStatusType {
    /// The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    /// The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: String6,
}

impl Validate for ManufacturerStatusType {}

/// DER OperationalModeStatus value:
/// 0 - Not applicable / Unknown
/// 1 - Off
/// 2 - Operational mode
/// 3 - Test mode
/// All other values reserved.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "OperationalModeStatusType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct OperationalModeStatusType {
    /// The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    /// The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: OperationalModeStatusValue,
}

impl Validate for OperationalModeStatusType {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum OperationalModeStatusValue {
    #[default]
    NotApplicable = 0,
    Off = 1,
    Operational = 2,
    Test = 3,
}

/// DER StateOfChargeStatus value: Percent data type
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "StateOfChargeStatusType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct StateOfChargeStatusType {
    /// The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    /// The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: Percent,
}

impl Validate for StateOfChargeStatusType {}

/// DER StorageModeStatus value:
/// 0 – storage charging
/// 1 – storage discharging
/// 2 – storage holding
/// All other values reserved.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "StorageModeStatusType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct StorageModeStatusType {
    /// The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    /// The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: StorageModeStatusValue,
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum StorageModeStatusValue {
    #[default]
    StorageCharging = 0,
    StorageDischarging = 1,
    StorageHolding = 2,
}

impl Validate for StorageModeStatusType {}

#[cfg(not(feature = "csip_aus"))]
#[test]
fn dercap_no_csip_aus() {
    let expected = r#"<DERCapability xmlns="urn:ieee:std:2030.5:ns">
  <modesSupported>0</modesSupported>
  <rtgMaxW>
    <multiplier>0</multiplier>
    <value>0</value>
  </rtgMaxW>
  <type>0</type>
</DERCapability>"#;
    let dcap = DERCapability::default();
    let out = serialize(&dcap).unwrap();
    assert_eq!(expected, out);
}

#[cfg(feature = "csip_aus")]
#[test]
fn csip_aus_dercap() {
    let expected = r#"<DERCapability xmlns="urn:ieee:std:2030.5:ns" xmlns:csipaus="https://csipaus.org/ns">
  <modesSupported>0</modesSupported>
  <csipaus:doeModesSupported>0</csipaus:doeModesSupported>
  <rtgMaxW>
    <multiplier>0</multiplier>
    <value>0</value>
  </rtgMaxW>
  <type>0</type>
</DERCapability>"#;
    let dcap = DERCapability::default();
    let out = serialize(&dcap).unwrap();
    assert_eq!(expected, out);
}

#[cfg(not(feature = "csip_aus"))]
#[test]
fn dercontrolbase_no_csip_aus() {
    let expected = r#"<DERControlBase xmlns="urn:ieee:std:2030.5:ns" />"#;
    let dercb = DERControlBase::default();
    let out = serialize(&dercb).unwrap();
    assert_eq!(expected, out);
}

#[cfg(feature = "csip_aus")]
#[test]
fn csip_aus_dercontrolbase() {
    let expected = r#"<DERControlBase xmlns="urn:ieee:std:2030.5:ns" xmlns:csipaus="https://csipaus.org/ns">
  <csipaus:opModImpLimW>
    <multiplier>0</multiplier>
    <value>0</value>
  </csipaus:opModImpLimW>
  <csipaus:opModExpLimW>
    <multiplier>0</multiplier>
    <value>0</value>
  </csipaus:opModExpLimW>
  <csipaus:opModGenLimW>
    <multiplier>0</multiplier>
    <value>0</value>
  </csipaus:opModGenLimW>
  <csipaus:opModLoadLimW>
    <multiplier>0</multiplier>
    <value>0</value>
  </csipaus:opModLoadLimW>
</DERControlBase>"#;
    let mut dercb = DERControlBase::default();
    dercb.op_mod_imp_lim_w = Some(Default::default());
    dercb.op_mod_exp_lim_w = Some(Default::default());
    dercb.op_mod_gen_lim_w = Some(Default::default());
    dercb.op_mod_load_lim_w = Some(Default::default());
    let out = serialize(&dercb).unwrap();
    assert_eq!(expected, out);
}
