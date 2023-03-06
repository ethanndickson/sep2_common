#![allow(clippy::needless_arbitrary_self_type)] // Not needless, our macro uses a lifetime annotation
#![allow(non_snake_case)] // We match the specs camel case usage
use crate::{
    identification::*,
    primitives::{String192, String20, UInt16},
    types::{
        DateTimeInterval, DeviceCategoryType, OneHourRangeType, PriorityType, TOUType, TimeType,
    },
};
use ::inheritance::{inheritable, Inheritance};
/*
* Described mainly in section 10.2.3 detail in section B.2.3.3.
* Specifically, it contains the traits and objects for Events
* and related types that extend it (e.g. RandomizableEvent)
*/

/*
=============
=== Traits ==
=============
*/
#[inheritable]
pub trait Event {
    fn creationTime(&self) -> &TimeType;
    fn interval(&self) -> &DateTimeInterval;
    fn status(&self) -> &EventStatus;
}

#[inheritable]
pub trait Randomizable {
    fn random_duration(&self) -> Option<&OneHourRangeType>;
    fn random_start(&self) -> Option<&OneHourRangeType>;
}

/*
======================================
=== Concrete Trait Implementations ===
======================================
*/

#[derive(Inheritance)]
pub struct EventObj {
    creation_time: TimeType,
    interval: DateTimeInterval,
    status: EventStatus,
    #[inherits(Respondable)]
    #[inherits(Subscribable)]
    #[inherits(Identified)]
    #[inherits(Resource)]
    respsubsidentrsrc: RespondableSubscribableIdentifiedObject,
}

impl Event for EventObj {
    fn creationTime(&self) -> &TimeType {
        &self.creation_time
    }

    fn interval(&self) -> &DateTimeInterval {
        &self.interval
    }

    fn status(&self) -> &EventStatus {
        &self.status
    }
}

pub struct RandomizableObj {
    random_duration: Option<OneHourRangeType>,
    random_start: Option<OneHourRangeType>,
}

impl Randomizable for RandomizableObj {
    fn random_duration(&self) -> Option<&OneHourRangeType> {
        self.random_duration.as_ref()
    }

    fn random_start(&self) -> Option<&OneHourRangeType> {
        self.random_start.as_ref()
    }
}

/*
=======================
=== Concrete Objects ==
=======================
*/

#[repr(u8)]
pub enum currentStatus {
    Scheduled = 0,
    Active = 1,
    Cancelled = 2,
    CancelledRandom = 3,
    Superseded = 4,
    // TODO Ethan: Can we use remaining for implementation details?
    // What does 'reserved' mean in this context?
    // Note: EPRI Client schedule.c:18
}
pub struct EventStatus {
    status: currentStatus,
    date_time: TimeType,
    potentially_superseded: bool,
    potentially_superseded_time: Option<TimeType>,
    reason: String192,
}

/*
==================
=== Inheritors ===
==================
*/
#[derive(Inheritance)]
pub struct TextMessage {
    orginator: Option<String20>,
    priority: PriorityType,
    // TODO Ethan: Spec just says String, I assume we can use a heap allocated String?
    textMessage: String,
    #[inherits(Event)]
    event: EventObj,
}

#[derive(Inheritance)]
pub struct RandomizableEvent {
    #[inherits(Event)]
    event: EventObj,
    #[inherits(Randomizable)]
    randomizable: RandomizableObj,
}

#[derive(Inheritance)]
pub struct TimeTarrifInterval {
    touTier: TOUType,
    #[inherits(Event)]
    #[inherits(Randomizable)]
    randomevent: RandomizableEvent,
}

#[derive(Inheritance)]
pub struct DERControl {
    deviceCategory: Option<DeviceCategoryType>,
    #[inherits(Event)]
    #[inherits(Randomizable)]
    randomevent: RandomizableEvent,
}

#[derive(Inheritance)]
pub struct EndDeviceControl {
    deviceCategory: DeviceCategoryType,
    drProgramMandatory: bool,
    loadShiftForward: bool,
    overrideDuration: Option<UInt16>,
    #[inherits(Event)]
    #[inherits(Randomizable)]
    randomevent: RandomizableEvent,
}
