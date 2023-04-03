use common::config::YASERDE_CFG;
use common::{identification::List, primitives::Uint32};
#[test]
fn list_deserialize() {
    let list = List {
        all: Uint32(0),
        results: Uint32(0),
        href: Some(String::from("/sample/list/uri/")),
    };
    println!(
        "{}",
        yaserde::ser::to_string_with_config(&list, &YASERDE_CFG).unwrap()
    )
}
