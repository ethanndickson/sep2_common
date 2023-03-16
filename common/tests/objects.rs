use common::xsd::{
    ApplianceLoadReduction, ApplianceLoadReductionType, DateTimeInterval, DeviceCategoryType,
    DutyCycle, EndDeviceControl, EventStatus, HexBinary128, HexBinary8, Int16, Int64, Offset,
    OneHourRangeType, SetPoint, String192, String32, SubscribableType, TargetReduction, Uint16,
    Uint32, Uint8,
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
        appliance_load_reduction: ApplianceLoadReduction {
            _type: ApplianceLoadReductionType::TemporaryApplianceLoadReduction,
        },
        device_category: DeviceCategoryType::Sauna,
        dr_program_mandatory: true,
        duty_cycle: DutyCycle {
            normal_value: Uint8(0),
        },
        load_shift_forward: true,
        offset: Offset {
            cooling_offset: Uint8(0),
            heating_offset: Uint8(0),
            load_adjustment_percentage_offset: Uint16(20),
        },
        override_duration: Uint16(20),
        set_point: SetPoint {
            cooling_setpoint: Int16(0),
            heating_setpoint: Int16(0),
        },
        target_reduction: TargetReduction {
            _type: common::xsd::UnitType::Watts,
            value: Uint16(12),
        },
        randomize_duration: OneHourRangeType(Int16(0)),
        randomize_start: OneHourRangeType(Int16(0)),
        creation_time: Int64(0),
        event_status: EventStatus {
            current_status: Uint8(0),
            date_time: Int64(0),
            potentially_superseded: false,
            potentially_superseded_time: Int64(0),
            reason: String192("asd".to_string()),
        },
        interval: DateTimeInterval {
            duration: Uint32(0),
            start: Int64(0),
        },
        m_rid: HexBinary128("".to_string()),
        description: String32("".to_string()),
        version: Uint16(0),
        subscribable: SubscribableType::AllSubscriptions,
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
