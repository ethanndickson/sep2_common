use macros::Link;
use macros::Resource;
use serde::Deserialize;
use serde::Serialize;

/*
 * Defines traits and structs described generally in section 8.2
 * and in detail in section B.2.3.2. Specifically, it describes the
 *  Resource trait and traits that extend it (e.g. List, IdentifiedObject)
 */
use super::primitives::*;
/// Attribute data
/// was defined as a struct to allow for storage format flexibility
/// but that was later decided to be pointless flexibility at the cost
/// of simplicty

/// anyURI mentioned in the specification can either be a relative
/// address or an absolute reference (in the subs/notf function set)
/// there is no mention of how it should be implemented, so for the moment
/// it will be an owned String type.
use super::types::*;

// @[future] Ethan:
// Consider switching to [inheritance-rs](https://github.com/danielhenrymantilla/inheritance-rs) instead of rolling our own & maintaining that (It may not support tuple structs?)
// Plausibility of ditching the ported-OOP with composition into some other design? Will depend on how we make use of polymorphism, likely here to stay.

// Traits
pub trait Resource {
    fn get_href(&self) -> Option<String> {
        None
    }
}

pub trait List {
    type Inner; // every struct can only implement this trait for 1 type.
    fn get_values(s: UInt16, a: Option<TimeType>, l: UInt32) -> Vec<Self::Inner>; // need query parameters.
}

pub trait Link {
    fn get_href(&self) -> String;
}

// TODO Ethan: Derive macro that uses RespondableData impl as a base
pub trait Respondable {
    fn get_replyTo(&self) -> Option<String>;
    fn get_responseRequired(&self) -> Option<HexBinary8>;
    // TODO: Do we need & and &mut versions?
}

// TODO Ethan: Derive macro that uses SubscribableData impl as a base
pub trait Subscribable {
    fn get_subscribable(&self) -> Option<SubscribableType>;
}

// TODO Ethan: Derive macro that uses IdentifiedData impl as a base
pub trait Identified {
    fn get_description(&self) -> Option<String32>;
    fn get_mrid(&self) -> mRIDType;
    fn get_version(&self) -> Option<VersionType>;
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
            if !href.starts_with("/") || href.len() > 255 {
                return ResourceObj { href: None };
            }
        }
        ResourceObj { href }
    }
}

impl Resource for ResourceObj {
    fn get_href(&self) -> Option<String> {
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
        if let Some(_) = output {
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
        if let Some(_) = output {
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
    fn get_result_value(&self) -> u32 {
        self.result
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct ListObj<T: Resource> {
    super_class: ResourceObj,
    list_data: ListData<T>,
}

impl<T: Resource> ListObj<T> {
    fn new(href: &str) -> ListObj<T> {
        ListObj {
            super_class: ResourceObj::new(Some(href.to_owned())),
            list_data: ListData::new(),
        }
    }
}

impl<T: Resource> Resource for ListObj<T> {
    fn get_href(&self) -> Option<String> {
        self.super_class.get_href()
    }
}

// «XSDattribute»
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LinkObj {
    href: String,
}

impl Link for LinkObj {
    fn get_href(&self) -> String {
        return self.href.clone();
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct RespondableData {
    reply_to: Option<String>,
    response_required: HexBinary8,
}

impl Respondable for RespondableData {
    fn get_replyTo(&self) -> Option<String> {
        self.reply_to.clone()
    }

    fn get_responseRequired(&self) -> Option<HexBinary8> {
        Some(self.response_required)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct SubscribableData {
    subscribable: Option<SubscribableType>,
}

impl Subscribable for SubscribableData {
    fn get_subscribable(&self) -> Option<SubscribableType> {
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
    fn get_description(&self) -> Option<String32> {
        Some(self.description.clone())
    }

    fn get_mrid(&self) -> mRIDType {
        self.mrid_type
    }

    fn get_version(&self) -> Option<VersionType> {
        Some(self.version)
    }
}

pub struct ListLink {
    super_class: LinkObj,
    all: Option<UInt32>,
}

impl Link for ListLink {
    fn get_href(&self) -> String {
        self.super_class.get_href()
    }
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct RespondableResource {
    super_class: ResourceObj,
    respondable_data: RespondableData,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct SubscribableResource {
    super_class: ResourceObj,
    subscribable_data: SubscribableData,
}

#[derive(Resource, Default, Debug, Serialize)]
pub struct IdentifiedObject {
    super_class: ResourceObj,
    identified_data: IdentifiedData,
}

#[derive(Resource, Default, Debug, Serialize)]
pub struct SubscribableIdentifiedObject {
    super_class: SubscribableResource,
    identified_data: IdentifiedData,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct SubscribableList {
    super_class: SubscribableResource,
    list_data: ListData<SubscribableResource>,
}

#[derive(Resource, Default, Debug, Serialize)]
pub struct RespondableSubscribableIdentifiedObject {
    super_class: RespondableResource,
    subscribable_data: SubscribableData,
    identified_data: IdentifiedData,
}

#[derive(Resource, Default, Debug, Serialize)]
pub struct RespondableIdentifiedObject {
    super_class: RespondableResource,
    identified_data: IdentifiedData,
}
