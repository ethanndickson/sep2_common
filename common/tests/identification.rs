use common::config::YASERDE_CFG;
use common::identification::List;
use yaserde::de::from_str;
use yaserde::ser::to_string_with_config;
#[test]
fn list_serde() {
    let orig = List::default();
    println!("{}", to_string_with_config(&orig, &YASERDE_CFG).unwrap());
    let new: List = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}
