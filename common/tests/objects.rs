use common::objects::{EndDeviceControl, EventStatus, PrimacyType};
use common::primitives::*;
use common::xsd::{
    ApplianceLoadReduction, ApplianceLoadReductionType, DateTimeInterval, DeviceCategoryTypeFlags,
    DutyCycle, Offset, OneHourRangeType, SetPoint, SubscribableType, TargetReduction, UnitType,
};
use yaserde::ser::Config;

const YASERDE_CFG: Config = yaserde::ser::Config {
    perform_indent: true,
    write_document_declaration: false,
    indent_string: None,
};

#[test]
fn reserved_enum() {
    let primacy = PrimacyType::InHomeEnergyManagementSystem;
    println!(
        "{}",
        yaserde::ser::to_string_with_config(&primacy, &YASERDE_CFG)
            .ok()
            .unwrap()
    );
}

#[test]
fn edc_deserialize() {
    let edc = EndDeviceControl {
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
    println!(
        "{}",
        yaserde::ser::to_string_with_config(&edc, &YASERDE_CFG)
            .ok()
            .unwrap()
    );
}
