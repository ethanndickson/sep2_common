#![allow(clippy::needless_arbitrary_self_type)] // Not needless, our macro uses a lifetime annotation
#![allow(non_snake_case)] // We match the specs camel case usage

use super::primitives::*;
use super::types::*;
use ::inheritance::{inheritable, Inheritance};
use serde::Deserialize;
use serde::Serialize;
/*
 * Defines traits and structs described generally in section 8.2
 * and in detail in section B.2.3.2. Specifically, it describes the
 *  Resource trait and traits that extend it (e.g. List, IdentifiedObject)
 */

// anyURI mentioned in the specification can either be a relative
// address or an absolute reference (in the subs/notf function set)
// there is no mention of how it should be implemented, so for the moment
// it will be an owned String type.

// @[future] Ethan:
// Plausibility of ditching the ported-OOP with composition into some other design?
// Seems like inheritance is here to stay

/*
=============
=== Traits ==
=============
*/
#[inheritable]
pub trait Resource {
    fn href(&self) -> Option<&str>;
}
#[inheritable]
pub trait Respondable {
    fn reply_to(&self) -> Option<&str>;
    fn response_required(&self) -> Option<HexBinary8>;
}

#[inheritable]
pub trait Subscribable {
    fn subscribable(&self) -> Option<&SubscribableType>;
}

#[inheritable]
pub trait Identified {
    fn description(&self) -> Option<&str>;
    fn mrid(&self) -> &mRIDType;
    fn version(&self) -> Option<VersionType>;
}

#[inheritable]
pub trait Link {
    fn href(&self) -> String;
}

// TODO Ethan: PR to inheritance-rs that adds trait generic support so we can derive List from anything that has ListData
// #[inheritable]
pub trait List<T: Resource> {
    // TODO Ethan: Figure out where Neel got this function signature from
    fn values(s: UInt16, a: Option<TimeType>, l: UInt32) -> Vec<T>;
}

/*
======================================
=== Concrete Trait Implementations ===
======================================
*/
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResourceObj {
    href: Option<String>,
}

impl ResourceObj {
    /// input's that do not start with '/' or are >255 characters
    /// are ignored and href value will be None.
    pub fn new(href: Option<&str>) -> ResourceObj {
        ResourceObj {
            href: href
                .filter(|s| s.starts_with('/') && s.len() <= 255)
                .map(String::from),
        }
    }
}

impl Resource for ResourceObj {
    fn href(&self) -> Option<&str> {
        self.href.as_deref()
    }
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RespondableData {
    reply_to: Option<String>,
    response_required: Option<HexBinary8>,
}

impl Respondable for RespondableData {
    fn reply_to(&self) -> Option<&str> {
        self.reply_to.as_deref()
    }

    fn response_required(&self) -> Option<HexBinary8> {
        self.response_required
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SubscribableData {
    subscribable: Option<SubscribableType>,
}

impl Subscribable for SubscribableData {
    fn subscribable(&self) -> Option<&SubscribableType> {
        self.subscribable.as_ref()
    }
}

#[derive(Default, Debug, Serialize)]
pub struct IdentifiedData {
    description: Option<String32>,
    mrid_type: mRIDType,
    version: Option<VersionType>,
}

impl Identified for IdentifiedData {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn mrid(&self) -> &mRIDType {
        &self.mrid_type
    }

    fn version(&self) -> Option<VersionType> {
        self.version
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ListObj<T: Resource> {
    res: ResourceObj,
    list: ListData<T>,
}

impl<T: Resource> List<T> for ListObj<T> {
    fn values(s: UInt16, a: Option<TimeType>, l: UInt32) -> Vec<T> {
        todo!()
    }
}

impl<T: Resource> ListObj<T> {
    fn new(href: &str) -> ListObj<T> {
        ListObj {
            res: ResourceObj::new(Some(href)),
            list: ListData::new(),
        }
    }
}

impl<T: Resource> Resource for ListObj<T> {
    fn href(&self) -> Option<&str> {
        self.res.href()
    }
}

/*
==================
=== Inheritors ===
==================
*/

// TODO Ethan: Check this matches the spec alongside the remaining link & list code
#[derive(Inheritance)]
pub struct ListLink {
    #[inherits(Link)]
    link: LinkObj,
    all: Option<UInt32>,
}

#[derive(Inheritance, Default, Debug, Serialize, Deserialize)]
pub struct RespondableResource {
    #[inherits(Resource)]
    rsrc: ResourceObj,
    #[inherits(Respondable)]
    resp: RespondableData,
}

#[derive(Inheritance, Default, Debug, Serialize, Deserialize)]
pub struct SubscribableResource {
    #[inherits(Resource)]
    rsrc: ResourceObj,
    #[inherits(Subscribable)]
    subs: SubscribableData,
}

#[derive(Inheritance, Default, Debug, Serialize)]
pub struct IdentifiedObject {
    #[inherits(Resource)]
    rsrc: ResourceObj,
    #[inherits(Identified)]
    ident: IdentifiedData,
}

#[derive(Inheritance, Default, Debug, Serialize)]
pub struct SubscribableIdentifiedObject {
    #[inherits(Resource)]
    #[inherits(Subscribable)]
    subs: SubscribableResource,
    #[inherits(Identified)]
    ident: IdentifiedData,
}
#[derive(Inheritance, Default, Debug, Serialize)]
pub struct RespondableSubscribableIdentifiedObject {
    #[inherits(Respondable)]
    #[inherits(Resource)]
    resprsrc: RespondableResource,
    #[inherits(Subscribable)]
    subs: SubscribableData,
    #[inherits(Identified)]
    ident: IdentifiedData,
}

#[derive(Inheritance, Default, Debug, Serialize)]
pub struct RespondableIdentifiedObject {
    #[inherits(Respondable)]
    #[inherits(Resource)]
    resprsrc: RespondableResource,
    #[inherits(Identified)]
    ident: IdentifiedData,
}

#[derive(Inheritance, Default, Debug, Serialize, Deserialize)]
pub struct SubscribableList<T: Resource> {
    #[inherits(Subscribable)]
    #[inherits(Resource)]
    subs: SubscribableResource,
    // TODO Ethan: This will need to also derive a List implementation
    // #[inherits(List)]
    list_subs: ListData<T>,
}

// «XSDattribute»
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LinkObj {
    href: String,
}

impl Link for LinkObj {
    fn href(&self) -> String {
        self.href.clone()
    }
}

/// List fields as per IEEE 2030.5 specification.
#[derive(Default, Debug, Serialize, Deserialize)]
struct ListData<T: Resource> {
    all: u32,
    result: u32,
    items: Vec<T>,
}

impl<T: Resource> ListData<T> {
    fn new() -> ListData<T> {
        ListData {
            all: 0,
            result: 0,
            items: Vec::<T>::new(),
        }
    }

    /// adds item onto end of ListData collection of `T`
    /// if `all` is greater than `result`, it's value is not affected.
    fn push(&mut self, item: T) {
        self.items.push(item);
        self.result += 1;
        if self.all == self.result {
            self.all += 1
        }
    }
    /// adds item onto end of ListData collection of `T`
    /// incremenets `all` and `result` values. If
    fn push_and_increment(&mut self, item: T) {
        self.items.push(item);
        self.result += 1;
        self.all += 1;
    }
    /// returns `Some(ListData[index])` if it exists, else `None`
    /// Decrements `result`. DOES NOT decrement `all`.
    /// checks for index out of bounds based on `result` number.
    fn remove(&mut self, index: u32) -> Option<T> {
        if index < self.result {
            let output = Some(self.items.remove(index.try_into().unwrap()));
            self.result -= 1;
            output
        } else {
            None
        }
    }
    /// returns `Some(ListData[index])` if it exists, else `None`
    /// Decrements `result`. DOES decrement `all`.
    /// checks for index out of bounds based on `result` number.
    fn remove_and_decrement(&mut self, index: u32) -> Option<T> {
        if index < self.result {
            let output = Some(self.items.remove(index.try_into().unwrap()));
            self.result -= 1;
            self.all -= 1;
            output
        } else {
            None
        }
    }
    /// removes an item from the ListData if it exists within the
    fn remove_href(&mut self, href: &str) -> bool {
        true
    }
    /// returns `Some(ListData[index])` if it exists, else `None`
    /// Decrements `result`. DOES NOT decrement `all`.
    /// checks for index out of bounds based on `result` number.
    fn pop(&mut self) -> Option<T> {
        let output = self.items.pop();
        if output.is_some() {
            self.result -= 1;
        }
        output
    }
    /// returns `Some(ListData[index])` if it exists, else `None`
    /// Decrements `result`. DOES decrement `all`.
    /// checks for index out of bounds based on `result` number.
    fn pop_and_decrement(&mut self) -> Option<T> {
        // storing in output because I dunno if this fails or not.
        let output = self.items.pop();
        if output.is_some() {
            self.result -= 1;
            self.all -= 1;
        }
        output
    }
    /// sets `self.all` to `all` if `all >= self.result`
    fn set_all_value(&mut self, all: u32) {
        if all >= self.result {
            self.all = all;
        } else {
            self.all = self.result
        }
    }
    fn result_value(&self) -> u32 {
        self.result
    }
}
