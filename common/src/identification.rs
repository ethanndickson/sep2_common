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
/// Attribute data
/// was defined as a struct to allow for storage format flexibility
/// but that was later decided to be pointless flexibility at the cost
/// of simplicty

/// anyURI mentioned in the specification can either be a relative
/// address or an absolute reference (in the subs/notf function set)
/// there is no mention of how it should be implemented, so for the moment
/// it will be an owned String type.

// @[future] Ethan:
// Plausibility of ditching the ported-OOP with composition into some other design? Will depend on how we make use of polymorphism, likely here to stay.
// Determine if/how our 'getters' should borrow

// Traits
#[inheritable]
pub trait Resource {
    fn href(&self) -> Option<String> {
        None
    }
}
// TODO Ethan: We don't need macro inheritance for thhis since we only have List & SubscribableList; worth implementing anyway?
// inheritance-rs doesn't currently support inheritance of our inner type alias
pub trait List {
    type Inner; // every struct can only implement this trait for 1 type.
    fn values(s: UInt16, a: Option<TimeType>, l: UInt32) -> Vec<Self::Inner>; // need query parameters.
}
#[inheritable]
pub trait Link {
    fn href(&self) -> String;
}

#[inheritable]
pub trait Respondable {
    fn replyTo(&self) -> Option<String>;
    fn responseRequired(&self) -> Option<HexBinary8>;
}

#[inheritable]
pub trait Subscribable {
    fn subscribable(&self) -> Option<SubscribableType>;
}

#[inheritable]
pub trait Identified {
    fn description(&self) -> Option<String32>;
    fn mrid(&self) -> mRIDType;
    fn version(&self) -> Option<VersionType>;
}

// Data Containers
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResourceObj {
    href: Option<String>,
}

impl ResourceObj {
    /// input's that do not start with '/' or are >255 characters
    /// are ignored and href value will be None.
    pub fn new(href: Option<String>) -> ResourceObj {
        if let Some(ref href) = href {
            if !href.starts_with('/') || href.len() > 255 {
                return ResourceObj { href: None };
            }
        }
        ResourceObj { href }
    }
}

impl Resource for ResourceObj {
    fn href(&self) -> Option<String> {
        if let Some(output) = &self.href {
            return Some(output.to_owned());
        }
        None
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

// TODO Ethan: List inheritance for SubscribableList

#[derive(Default, Debug, Serialize, Deserialize)]
struct ListObj<T: Resource> {
    res: ResourceObj,
    list: ListData<T>,
}

impl<T: Resource> ListObj<T> {
    fn new(href: &str) -> ListObj<T> {
        ListObj {
            res: ResourceObj::new(Some(href.to_owned())),
            list: ListData::new(),
        }
    }
}

impl<T: Resource> Resource for ListObj<T> {
    fn href(&self) -> Option<String> {
        self.res.href()
    }
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RespondableData {
    reply_to: Option<String>,
    response_required: HexBinary8,
}

impl Respondable for RespondableData {
    fn replyTo(&self) -> Option<String> {
        self.reply_to.clone()
    }

    fn responseRequired(&self) -> Option<HexBinary8> {
        Some(self.response_required)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SubscribableData {
    subscribable: Option<SubscribableType>,
}

impl Subscribable for SubscribableData {
    fn subscribable(&self) -> Option<SubscribableType> {
        self.subscribable.clone()
    }
}

#[derive(Default, Debug, Serialize)]
pub struct IdentifiedData {
    description: String32,
    mrid_type: mRIDType,
    version: VersionType,
}

impl Identified for IdentifiedData {
    fn description(&self) -> Option<String32> {
        Some(self.description.clone())
    }

    fn mrid(&self) -> mRIDType {
        self.mrid_type
    }

    fn version(&self) -> Option<VersionType> {
        Some(self.version)
    }
}

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

// TODO Ethan: This needs to be redesigned to match the spec
// This will likely require a specialised implementation of subscribable & resource
#[derive(Inheritance, Default, Debug, Serialize, Deserialize)]
pub struct SubscribableList {
    #[inherits(Subscribable)]
    #[inherits(Resource)]
    subs: SubscribableResource,
    list_subs: ListData<SubscribableResource>,
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
