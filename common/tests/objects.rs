use common::xsd::{
    ApplianceLoadReduction, ApplianceLoadReductionType, DateTimeInterval, DeviceCategoryTypeFlags,
    DutyCycle, EndDeviceControl, EventStatus, HexBinary128, HexBinary32, HexBinary8, Int16, Int64,
    Offset, OneHourRangeType, SetPoint, String192, String32, SubscribableType, TargetReduction,
    Uint16, Uint32, Uint8, UnitType,
};
use yaserde::ser::Config;

const yaserde_cfg: Config = yaserde::ser::Config {
    perform_indent: true,
    write_document_declaration: false,
    indent_string: None,
};

#[test]
fn edc_deserialize() {
    let edc = EndDeviceControl {
        appliance_load_reduction: Some(ApplianceLoadReduction {
            _type: ApplianceLoadReductionType::TemporaryApplianceLoadReduction,
        }),
        device_category: HexBinary32((DeviceCategoryTypeFlags::Sauna as u32).to_string()),
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
        m_rid: HexBinary128("".to_string()),
        description: Some(String32("".to_string())),
        version: Some(Uint16(0)),
        subscribable: Some(SubscribableType::AllSubscriptions),
        reply_to: Some("Test".to_string()),
        response_required: Some(HexBinary8("test".to_string())),
        href: Some("test".to_string()),
    };
    println!(
        "{}",
        yaserde::ser::to_string_with_config(&edc, &yaserde_cfg)
            .ok()
            .unwrap()
    );
}
