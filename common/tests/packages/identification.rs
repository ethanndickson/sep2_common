#![allow(non_snake_case)]
// TODO Ethan: Temporary import all
use sep2_common::packages::identification::*;
use yaserde::de::from_str;
use yaserde::ser::to_string;

#[test]
fn default_Resource() {
    let orig = Resource::default();
    let new: Resource = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Response() {
    let orig = Response::default();
    let new: Response = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_List() {
    let orig = List::default();
    let new: List = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ListLink() {
    let orig = ListLink::default();
    let new: ListLink = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_IdentifiedObject() {
    let orig = IdentifiedObject::default();
    let new: IdentifiedObject = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RespondableResource() {
    let orig = RespondableResource::default();
    let new: RespondableResource = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RespondableIdentifiedObject() {
    let orig = RespondableIdentifiedObject::default();
    let new: RespondableIdentifiedObject = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RespondableSubscribableIdentifiedObject() {
    let orig = RespondableSubscribableIdentifiedObject::default();
    let new: RespondableSubscribableIdentifiedObject =
        from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SubscribableResource() {
    let orig = SubscribableResource::default();
    let new: SubscribableResource = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SubscribableList() {
    let orig = SubscribableList::default();
    let new: SubscribableList = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SubscribableIdentifiedObject() {
    let orig = SubscribableIdentifiedObject::default();
    let new: SubscribableIdentifiedObject = from_str(&to_string(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}
