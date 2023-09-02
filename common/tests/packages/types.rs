use common::packages::types::DstRuleType;

#[test]
fn basic_dstruletype() {
    // Documentation example
    let rule = DstRuleType::new(2700, 1, 5, 0, 4, 3);
    assert_eq!(rule.seconds(), 2700);
    assert_eq!(rule.hours(), 1);
    assert_eq!(rule.day_of_week(), 5);
    assert_eq!(rule.day_of_month(), 0);
    assert_eq!(rule.operator(), 4);
    assert_eq!(rule.month(), 3);
}
