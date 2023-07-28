use common::{
    config::YASERDE_CFG,
    packages::{
        primitives::{Int48, Int64, Uint32, Uint8},
        pubsub::Notification,
        xsd::{DateTimeInterval, Reading},
    },
};
use yaserde::ser::to_string_with_config;

/// resources/Examples/16_06_04_Notification.xml
#[test]
fn notification_example() {
    let inner: Reading = Reading {
        local_id: None,
        subscribable: None,
        consumption_block: None,
        quality_flags: None,
        time_period: Some(DateTimeInterval {
            duration: Uint32(0),
            start: Int64(12987364),
        }),
        tou_tier: None,
        value: Some(Int48(1001)),
        href: None,
    };
    let orig: Notification<Reading> = Notification {
        new_resource_uri: None,
        resource: Some(inner),
        status: Uint8(0),
        subscription_uri: "/dev/8/sub/5".to_owned(),
        subscribed_resource: "/upt/0/mr/4/r".to_owned(),
        href: None,
    };
    let expected = r#"<Notification xmlns="urn:ieee:std:2030.5:ns">
  <Resource xsi:type="Reading">
    <timePeriod>
      <duration>0</duration>
      <start>12987364</start>
    </timePeriod>
    <value>1001</value>
  </Resource>
  <subscriptionURI>/dev/8/sub/5</subscriptionURI>
  <subscribedResource>/upt/0/mr/4/r</subscribedResource>
</Notification>"#;
    let out = to_string_with_config(&orig, &YASERDE_CFG).unwrap();
    assert_eq!(expected, out);
}