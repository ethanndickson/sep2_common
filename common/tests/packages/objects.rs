#![allow(non_snake_case)]
use common::config::YASERDE_CFG;
use common::packages::objects::*;
use common::packages::primitives::*;
use common::packages::xsd::{
    ApplianceLoadReduction, ApplianceLoadReductionType, DateTimeInterval, DeviceCategoryTypeFlags,
    DutyCycle, Offset, OneHourRangeType, SetPoint, SubscribableType, TargetReduction, UnitType,
};
use yaserde::de::from_str;
use yaserde::ser::to_string_with_config;

#[test]
fn complex_serde() {
    let orig = EndDeviceControl {
        appliance_load_reduction: Some(ApplianceLoadReduction {
            _type: ApplianceLoadReductionType::TemporaryApplianceLoadReduction,
        }),
        device_category: HexBinary32(Uint32(DeviceCategoryTypeFlags::Sauna as u32)),
        dr_program_mandatory: true,
        duty_cycle: Some(DutyCycle {
            normal_value: Uint8(0),
        }),
        load_shift_forward: true,
        offset: Some(Offset {
            cooling_offset: Some(Uint8(0)),
            heating_offset: Some(Uint8(0)),
            load_adjustment_percentage_offset: Some(Uint16(20)),
        }),
        override_duration: Some(Uint16(20)),
        set_point: Some(SetPoint {
            cooling_setpoint: Some(Int16(0)),
            heating_setpoint: Some(Int16(0)),
        }),
        target_reduction: Some(TargetReduction {
            _type: UnitType::Watts,
            value: Uint16(12),
        }),
        randomize_duration: Some(OneHourRangeType(Int16(0))),
        randomize_start: Some(OneHourRangeType(Int16(0))),
        creation_time: Int64(0),
        event_status: EventStatus {
            current_status: Uint8(0),
            date_time: Int64(0),
            potentially_superseded: false,
            potentially_superseded_time: Some(Int64(0)),
            reason: Some(String192("asd".to_string())),
        },
        interval: DateTimeInterval {
            duration: Uint32(0),
            start: Int64(0),
        },
        m_rid: HexBinary128(0),
        description: Some(String32("".to_string())),
        version: Some(Uint16(0)),
        subscribable: Some(SubscribableType::AllSubscriptions),
        reply_to: Some("Test".to_string()),
        response_required: Some(HexBinary8(Uint8(0))),
        href: Some("test".to_string()),
    };
    let new: EndDeviceControl =
        from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EventStatus() {
    let orig = EventStatus::default();
    let new: EventStatus = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Event() {
    let orig = Event::default();
    let new: Event = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Error() {
    let orig = Error::default();
    let new: Error = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RandomizableEvent() {
    let orig = RandomizableEvent::default();
    let new: RandomizableEvent =
        from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERControl() {
    let orig = Dercontrol::default();
    let new: Dercontrol = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TextMessage() {
    let orig = TextMessage::default();
    let new: TextMessage = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeTariffInterval() {
    let orig = TimeTariffInterval::default();
    let new: TimeTariffInterval =
        from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EndDeviceControl() {
    let orig = EndDeviceControl::default();
    let new: EndDeviceControl =
        from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DemandResponseProgram() {
    let orig = DemandResponseProgram::default();
    let new: DemandResponseProgram =
        from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TariffProfile() {
    let orig = TariffProfile::default();
    let new: TariffProfile =
        from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MessagingProgram() {
    let orig = MessagingProgram::default();
    let new: MessagingProgram =
        from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PrimacyType() {
    let orig = PrimacyType::default();
    let new: PrimacyType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}