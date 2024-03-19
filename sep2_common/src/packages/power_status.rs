use crate::traits::{SEResource, Validate};
use sep2_common_derive::SEResource;

use super::{
    der::ActivePower,
    primitives::Uint32,
    types::{Percent, RealEnergy, TimeType},
};
use sepserde::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "PowerStatus")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PowerStatus {
    /// Battery system status
    /// 0 = unknown
    /// 1 = normal (more than LowChargeThreshold remaining)
    /// 2 = low (less than LowChargeThreshold remaining)
    /// 3 = depleted (0% charge remaining)
    /// 4 = not applicable (mains powered only)
    #[yaserde(rename = "batteryStatus")]
    pub battery_status: BatteryStatus,

    /// The time at which the reported values were recorded.
    #[yaserde(rename = "changedTime")]
    pub changed_time: TimeType,

    /// This value will be fixed for devices powered by a single source. This
    /// value may change for devices able to transition between multiple power
    /// sources (mains to battery backup, etc.).
    #[yaserde(rename = "currentPowerSource")]
    pub current_power_source: PowerSourceType,

    /// Estimate of remaining battery charge as a percent of full charge.
    #[yaserde(rename = "estimatedChargeRemaining")]
    pub estimated_charge_remaining: Option<Percent>,

    /// Estimated time (in seconds) to total battery charge depletion (under
    /// current load)
    #[yaserde(rename = "estimatedTimeRemaining")]
    pub estimated_time_remaining: Option<Uint32>,

    #[yaserde(rename = "PEVInfo")]
    pub pev_info: Option<Pevinfo>,

    /// If the device has a battery, this is the time since the device last
    /// switched to battery power, or the time since the device was restarted,
    /// whichever is less, in seconds.
    #[yaserde(rename = "sessionTimeOnBattery")]
    pub session_time_on_battery: Option<Uint32>,

    /// If the device has a battery, this is the total time the device has been
    /// on battery power, in seconds. It may be reset when the battery is
    /// replaced.
    #[yaserde(rename = "totalTimeOnBattery")]
    pub total_time_on_battery: Option<Uint32>,

    /// The default polling rate for this function set (this resource and all
    /// resources below), in seconds. If not specified, a default of 900 seconds
    /// (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    /// this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    /// A reference to the resource address (URI). Required in a response to a
    /// GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

#[derive(
    Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, YaSerialize, YaDeserialize,
)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum BatteryStatus {
    #[default]
    Unknown = 0,
    /// More than LowChargeThreshold remaining
    Normal = 1,
    /// Less than LowChargeThreshold remaining
    Low = 2,
    /// 0% charge remaining
    Depleted = 3,
    /// Mains Powered Only
    NotApplicable = 4,
}

impl Validate for PowerStatus {}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, YaSerialize, YaDeserialize)]
#[yaserde(rename = "PowerSourceType")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum PowerSourceType {
    #[default]
    None = 0,
    Mains = 1,
    LocalGeneration = 3,
    Emergency = 4,
    Unknown = 5,
    // 6-255 RESERVED
}

impl Validate for PowerSourceType {}

/// Contains attributes that can be exposed by PEVs and other devices that have
/// charging requirements.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "PEVInfo")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Pevinfo {
    /// This is the actual power flow in or out of the charger or inverter. This
    /// is calculated by the vehicle based on actual measurements. This number is
    /// positive for charging.
    #[yaserde(rename = "chargingPowerNow")]
    pub charging_power_now: ActivePower,

    /// This is the amount of energy that must be transferred from the grid to
    /// EVSE and PEV to achieve the target state of charge allowing for charger
    /// efficiency and any vehicle and EVSE parasitic loads. This is calculated
    /// by the vehicle and changes throughout the connection as forward or
    /// reverse power flow change the battery state of charge. This number is
    /// positive for charging.
    #[yaserde(rename = "energyRequestNow")]
    pub energy_request_now: RealEnergy,

    /// This is maximum power transfer capability that could be used for charging
    /// the PEV to perform the requested energy transfer. It is the lower of the
    /// vehicle or EVSE physical power limitations. It is not based on economic
    /// considerations. The vehicle may draw less power than this value based on
    /// its charging cycle. The vehicle defines this parameter. This number is
    /// positive for charging power flow.
    #[yaserde(rename = "maxForwardPower")]
    pub max_forward_power: ActivePower,

    /// This is computed by the PEV based on the charging profile to complete the
    /// energy transfer if the maximum power is authorized. The value will never
    /// be smaller than the ratio of the energy request to the power request
    /// because the charging profile may not allow the maximum power to be used
    /// throughout the transfer. This is a critical parameter for determining
    /// whether any slack time exists in the charging cycle between the current
    /// time and the TCIN.
    #[yaserde(rename = "minimumChargingDuration")]
    pub minimum_charging_duration: Uint32,

    /// This is the target state of charge that is to be achieved during charging
    /// before the time of departure (TCIN). The default value is 100%. The value
    /// cannot be set to a value less than the actual state of charge.
    #[yaserde(rename = "targetStateOfCharge")]
    pub target_state_of_charge: Percent,

    /// Time Charge is Needed (TCIN) is the time that the PEV is expected to
    /// depart. The value is manually entered using controls and displays in the
    /// vehicle or on the EVSE or using a mobile device. It is authenticated and
    /// saved by the PEV. This value may be updated during a charging session.
    #[yaserde(rename = "timeChargeIsNeeded")]
    pub time_charge_is_needed: TimeType,

    /// This is the time that the parameters are updated, except for changes to
    /// TCIN.
    #[yaserde(rename = "timeChargingStatusPEV")]
    pub time_charging_status_pev: TimeType,
}

impl Validate for Pevinfo {}
