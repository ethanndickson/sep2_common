use crate::traits::{
    SEEvent, SEIdentifiedObject, SEList, SERandomizableEvent, SEResource, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SESubscribableList, SESubscribableResource,
    Validate,
};
use sep2_common_derive::{
    SEEvent, SEIdentifiedObject, SEList, SERandomizableEvent, SEResource, SERespondableResource,
    SERespondableSubscribableIdentifiedObject, SESubscribableList, SESubscribableResource,
};

use super::{
    der::ActivePower,
    identification::{Link, ListLink, ResponseRequired},
    objects::EventStatus,
    primitives::{Int16, String32, Uint16, Uint32, Uint8},
    types::{
        ApplianceLoadReductionType, DateTimeInterval, DeviceCategoryType, MRIDType,
        OneHourRangeType, Percent, PrimacyType, SubscribableType, TimeType, UnitType, VersionType,
    },
};
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "LoadShedAvailabilityList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LoadShedAvailabilityList {
    #[yaserde(rename = "LoadShedAvailability")]
    pub load_shed_availability: Vec<LoadShedAvailability>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

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

impl Validate for LoadShedAvailabilityList {}

// The ApplianceLoadReduction object is used by a Demand Response service
// provider to provide signals for ENERGY STAR compliant appliances. See the
// definition of ApplianceLoadReductionType for more information.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "ApplianceLoadReduction")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ApplianceLoadReduction {
    // Indicates the type of appliance load reduction requested.
    #[yaserde(rename = "type")]
    pub _type: ApplianceLoadReductionType,
}

impl Validate for ApplianceLoadReduction {}

#[derive(
    Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEIdentifiedObject, SEResource,
)]
#[yaserde(rename = "DemandResponseProgram")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DemandResponseProgram {
    #[yaserde(rename = "ActiveEndDeviceControlListLink")]
    pub active_end_device_control_list_link: Option<ListLink>,

    // This attribute allows program providers to specify the requested
    // granularity of updates to LoadShedAvailability sheddablePercent. If not
    // present, or set to 0, then updates to LoadShedAvailability SHALL NOT be
    // provided. If present and greater than zero, then clients SHALL provide
    // their LoadShedAvailability if it has not previously been provided, and
    // thereafter if the difference between the previously provided value and
    // the current value of LoadShedAvailability sheddablePercent is greater
    // than availabilityUpdatePercentChangeThreshold.
    #[yaserde(rename = "availabilityUpdatePercentChangeThreshold")]
    pub availability_update_percent_change_threshold: Option<Percent>,

    // This attribute allows program providers to specify the requested
    // granularity of updates to LoadShedAvailability sheddablePower. If not
    // present, or set to 0, then updates to LoadShedAvailability SHALL NOT be
    // provided. If present and greater than zero, then clients SHALL provide
    // their LoadShedAvailability if it has not previously been provided, and
    // thereafter if the difference between the previously provided value and
    // the current value of LoadShedAvailability sheddablePower is greater than
    // availabilityUpdatePowerChangeThreshold.
    #[yaserde(rename = "availabilityUpdatePowerChangeThreshold")]
    pub availability_update_power_change_threshold: Option<ActivePower>,

    #[yaserde(rename = "EndDeviceControlListLink")]
    pub end_device_control_list_link: Option<ListLink>,

    // Indicates the relative primacy of the provider of this program.
    #[yaserde(rename = "primacy")]
    pub primacy: PrimacyType,

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

impl PartialOrd for DemandResponseProgram {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DemandResponseProgram {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - Primacy (ascending)
        match self.primacy.cmp(&other.primacy) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mRID (descending)
        self.mrid.cmp(&other.mrid).reverse()
    }
}

impl Validate for DemandResponseProgram {}

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
#[yaserde(rename = "DemandResponseProgramList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DemandResponseProgramList {
    #[yaserde(rename = "DemandResponseProgram")]
    pub demand_response_program: Vec<DemandResponseProgram>,

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

impl Validate for DemandResponseProgramList {}

// Duty cycle control is a device specific issue and is managed by the device.
// The duty cycle of the device under control should span the shortest practical
// time period in accordance with the nature of the device under control and the
// intent of the request for demand reduction. The default factory setting
// SHOULD be three minutes for each 10% of duty cycle. This indicates that the
// default time period over which a duty cycle is applied is 30 minutes, meaning
// a 10% duty cycle would cause a device to be ON for 3 minutes. The “off
// state” SHALL precede the “on state”.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "DutyCycle")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DutyCycle {
    // Contains the maximum On state duty cycle applied by the end device, as a
    // percentage of time. The field not present indicates that this field has
    // not been used by the end device.
    #[yaserde(rename = "normalValue")]
    pub normal_value: Uint8,
}

impl Validate for DutyCycle {}

// Instructs an EndDevice to perform a specified action.
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
#[yaserde(rename = "EndDeviceControl")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceControl {
    #[yaserde(rename = "ApplianceLoadReduction")]
    pub appliance_load_reduction: Option<ApplianceLoadReduction>,

    // Specifies the bitmap indicating the categories of devices that SHOULD
    // respond. Devices SHOULD ignore events that do not indicate their device
    // category.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: DeviceCategoryType,

    // A flag to indicate if the EndDeviceControl is considered a mandatory
    // event as defined by the service provider issuing the EndDeviceControl.
    // The drProgramMandatory flag alerts the client/user that they will be
    // subject to penalty or ineligibility based on the service provider’s
    // program rules for that deviceCategory.
    #[yaserde(rename = "drProgramMandatory")]
    pub dr_program_mandatory: bool,

    #[yaserde(rename = "DutyCycle")]
    pub duty_cycle: Option<DutyCycle>,

    // Indicates that the event intends to increase consumption. A value of true
    // indicates the intention to increase usage value, and a value of false
    // indicates the intention to decrease usage.
    #[yaserde(rename = "loadShiftForward")]
    pub load_shift_forward: bool,

    #[yaserde(rename = "Offset")]
    pub offset: Option<Offset>,

    // The overrideDuration attribute provides a duration, in seconds, for which
    // a client device is allowed to override this EndDeviceControl and still
    // meet the contractual agreement with a service provider without opting
    // out. If overrideDuration is not specified, then it SHALL default to 0.
    #[yaserde(rename = "overrideDuration")]
    pub override_duration: Option<Uint16>,

    #[yaserde(rename = "SetPoint")]
    pub set_point: Option<SetPoint>,

    #[yaserde(rename = "TargetReduction")]
    pub target_reduction: Option<TargetReduction>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval duration, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeDuration")]
    pub randomize_duration: Option<OneHourRangeType>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval start time, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeStart")]
    pub randomize_start: Option<OneHourRangeType>,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

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

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<ResponseRequired>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for EndDeviceControl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EndDeviceControl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primar Key - interval.start (ascending)
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

impl Validate for EndDeviceControl {}

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
#[yaserde(rename = "EndDeviceControlList")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceControlList {
    #[yaserde(rename = "EndDeviceControl")]
    pub end_device_control: Vec<EndDeviceControl>,

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

impl Validate for EndDeviceControlList {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "LoadShedAvailability")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LoadShedAvailability {
    // Indicates for how many seconds the consuming device will be able to
    // reduce consumption at the maximum response level.
    #[yaserde(rename = "availabilityDuration")]
    pub availability_duration: Option<Uint32>,

    #[yaserde(rename = "DemandResponseProgramLink")]
    pub demand_response_program_link: Option<Link>,

    // Maximum percent of current operating load that is estimated to be
    // sheddable.
    #[yaserde(rename = "sheddablePercent")]
    pub sheddable_percent: Option<Percent>,

    // Maximum amount of current operating load that is estimated to be
    // sheddable, in Watts.
    #[yaserde(rename = "sheddablePower")]
    pub sheddable_power: Option<ActivePower>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl PartialOrd for LoadShedAvailability {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LoadShedAvailability {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - href (ascending)
        self.href.cmp(&other.href).reverse()
    }
}

impl Validate for LoadShedAvailability {}

// If a temperature offset is sent that causes the heating or cooling
// temperature set point to exceed the limit boundaries that are programmed into
// the device, the device SHALL respond by setting the temperature at the limit.
// If an EDC is being targeted at multiple devices or to a device that controls
// multiple devices (e.g., EMS), it can provide multiple Offset types within one
// EDC. For events with multiple Offset types, a client SHALL select the Offset
// that best fits their operating function.
// Alternatively, an event with a single Offset type can be targeted at an EMS
// in order to request a percentage load reduction on the average energy usage
// of the entire premise. An EMS SHOULD use the Metering function set to
// determine the initial load in the premise, reduce energy consumption by
// controlling devices at its disposal, and at the conclusion of the event, once
// again use the Metering function set to determine if the desired load
// reduction was achieved.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "Offset")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Offset {
    // The value change requested for the cooling offset, in degree C / 10. The
    // value should be added to the normal set point for cooling, or if
    // loadShiftForward is true, then the value should be subtracted from the
    // normal set point.
    #[yaserde(rename = "coolingOffset")]
    pub cooling_offset: Option<Uint8>,

    // The value change requested for the heating offset, in degree C / 10. The
    // value should be subtracted for heating, or if loadShiftForward is true,
    // then the value should be added to the normal set point.
    #[yaserde(rename = "heatingOffset")]
    pub heating_offset: Option<Uint8>,

    // The value change requested for the load adjustment percentage. The value
    // should be subtracted from the normal setting, or if loadShiftForward is
    // true, then the value should be added to the normal setting.
    #[yaserde(rename = "loadAdjustmentPercentageOffset")]
    pub load_adjustment_percentage_offset: Option<Percent>,
}

impl Validate for Offset {}

// The SetPoint object is used to apply specific temperature set points to a
// temperature control device. The values of the heatingSetpoint and
// coolingSetpoint attributes SHALL be calculated as follows:
// Cooling/Heating Temperature Set Point / 100 = temperature in degrees Celsius
// where -273.15°C &lt;= temperature &lt;= 327.67°C, corresponding to a
// Cooling and/or Heating Temperature Set Point. The maximum resolution this
// format allows is 0.01°C.
// The field not present in a Response indicates that this field has not been
// used by the end device.
// If a temperature is sent that exceeds the temperature limit boundaries that
// are programmed into the device, the device SHALL respond by setting the
// temperature at the limit.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "SetPoint")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SetPoint {
    // This attribute represents the cooling temperature set point in degrees
    // Celsius / 100. (Hundredths of a degree C)
    #[yaserde(rename = "coolingSetpoint")]
    pub cooling_setpoint: Option<Int16>,

    // This attribute represents the heating temperature set point in degrees
    // Celsius / 100. (Hundredths of a degree C)
    #[yaserde(rename = "heatingSetpoint")]
    pub heating_setpoint: Option<Int16>,
}

impl Validate for SetPoint {}

// The TargetReduction object is used by a Demand Response service provider to
// provide a RECOMMENDED threshold that a device/premises should maintain its
// consumption below. For example, a service provider can provide a RECOMMENDED
// threshold of some kWh for a 3-hour event. This means that the device/premises
// would maintain its consumption below the specified limit for the specified
// period.
#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(rename = "TargetReduction")]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TargetReduction {
    // Indicates the type of reduction requested.
    #[yaserde(rename = "type")]
    pub _type: UnitType,

    // Indicates the requested amount of the relevant commodity to be reduced.
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for TargetReduction {}
