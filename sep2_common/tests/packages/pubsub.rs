use sep2_common::packages::metering::Reading;
use sep2_common::packages::pubsub::NotificationList;
use sep2_common::packages::types::DateTimeInterval;
use sep2_common::traits::SEList;
use sep2_common::{deserialize, serialize};
use sep2_common::{
    examples::NTF_16_06_04,
    packages::{
        primitives::{Int48, Int64, Uint32, Uint8},
        pubsub::{get_notif_type, Notification},
    },
};

/// resources/Examples/16_06_04_Notification.xml
pub(crate) fn create_notif_example() -> Notification<Reading> {
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
    Notification {
        new_resource_uri: None,
        resource: Some(inner),
        status: Uint8(0),
        subscription_uri: "/edev/8/sub/5".to_owned(),
        subscribed_resource: "/upt/0/mr/4/r".to_owned(),
        href: None,
    }
}

#[test]
fn notification_example() {
    let orig = create_notif_example();
    let out = serialize(&orig).unwrap();
    let expected = r#"<Notification xmlns="urn:ieee:std:2030.5:ns" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <Resource xsi:type="Reading">
    <timePeriod>
      <duration>0</duration>
      <start>12987364</start>
    </timePeriod>
    <value>1001</value>
  </Resource>
  <status>0</status>
  <subscriptionURI>/edev/8/sub/5</subscriptionURI>
  <subscribedResource>/upt/0/mr/4/r</subscribedResource>
</Notification>"#;
    assert_eq!(expected, out);
    let new: Notification<Reading> = deserialize(&expected).unwrap();
    assert_eq!(orig, new);
    let example: Notification<Reading> = deserialize(NTF_16_06_04).unwrap();
    assert_eq!(example, new);
}

#[test]
fn notification_list_default() {
    let expected = r#"<NotificationList xmlns="urn:ieee:std:2030.5:ns" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" all="2" results="0">
  <Notification>
    <Resource xsi:type="Reading">
      <timePeriod>
        <duration>0</duration>
        <start>12987364</start>
      </timePeriod>
      <value>1001</value>
    </Resource>
    <status>0</status>
    <subscriptionURI>/edev/8/sub/5</subscriptionURI>
    <subscribedResource>/upt/0/mr/4/r</subscribedResource>
  </Notification>
  <Notification>
    <Resource xsi:type="Reading">
      <timePeriod>
        <duration>0</duration>
        <start>12987364</start>
      </timePeriod>
      <value>1001</value>
    </Resource>
    <status>0</status>
    <subscriptionURI>/edev/8/sub/5</subscriptionURI>
    <subscribedResource>/upt/0/mr/4/r</subscribedResource>
  </Notification>
</NotificationList>"#;
    let res: Notification<Reading> = create_notif_example();
    let mut list: NotificationList<Reading> = Default::default();
    list.add(res.clone());
    list.add(res);
    let out = serialize(&list).unwrap();
    assert_eq!(expected, out);
}

#[test]
fn peek_notif_example() {
    let orig = create_notif_example();
    let orig = serialize(&orig).unwrap();
    let name = get_notif_type(&orig).unwrap();
    assert_eq!(name, "Reading");
}
