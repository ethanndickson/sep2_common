// File auto-generated using xsd-parser-rs & IEEE 2030.5 sep-ordered-dep.xsd
// Types should eventually be put in a module corresponding to their package
use std::str::FromStr;
use xsd_macro_utils::{UtilsDefaultSerde, UtilsTupleIo};
use xsd_parser::generator::validator::Validate;
use yaserde_derive::{YaDeserialize, YaSerialize};

// An 8-bit field encoded as a hex string (2 hex characters). Where applicable,
// bit 0, or the least significant bit, goes on the right. Note that hexBinary
// requires pairs of hex characters, so an odd number of characters requires a
// leading "0".
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct HexBinary8(pub String);

impl Validate for HexBinary8 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 1 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 1 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// A 16-bit field encoded as a hex string (4 hex characters max). Where
// applicable, bit 0, or the least significant bit, goes on the right. Note that
// hexBinary requires pairs of hex characters, so an odd number of characters
// requires a leading "0".
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct HexBinary16(pub String);

impl Validate for HexBinary16 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 2 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 2 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// A 32-bit field encoded as a hex string (8 hex characters max). Where
// applicable, bit 0, or the least significant bit, goes on the right. Note that
// hexBinary requires pairs of hex characters, so an odd number of characters
// requires a leading "0".
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct HexBinary32(pub String);

impl Validate for HexBinary32 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 4 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 4 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// A 48-bit field encoded as a hex string (12 hex characters max). Where
// applicable, bit 0, or the least significant bit, goes on the right. Note that
// hexBinary requires pairs of hex characters, so an odd number of characters
// requires a leading "0".
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct HexBinary48(pub String);

impl Validate for HexBinary48 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 6 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 6 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// A 64-bit field encoded as a hex string (16 hex characters max). Where
// applicable, bit 0, or the least significant bit, goes on the right. Note that
// hexBinary requires pairs of hex characters, so an odd number of characters
// requires a leading "0".
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct HexBinary64(pub String);

impl Validate for HexBinary64 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 8 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 8 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// A 128-bit field encoded as a hex string (32 hex characters max). Where
// applicable, bit 0, or the least significant bit, goes on the right. Note that
// hexBinary requires pairs of hex characters, so an odd number of characters
// requires a leading "0".
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct HexBinary128(pub String);

impl Validate for HexBinary128 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 16 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 16 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// A 160-bit field encoded as a hex string (40 hex characters max). Where
// applicable, bit 0, or the least significant bit, goes on the right. Note that
// hexBinary requires pairs of hex characters, so an odd number of characters
// requires a leading "0".
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct HexBinary160(pub String);

impl Validate for HexBinary160 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 20 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 20 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Character string of max length 6. In order to limit internal storage,
// implementations SHALL reduce the length of strings using multi-byte
// characters so that the string may be stored using "maxLength" octets in the
// given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct String6(pub String);

impl Validate for String6 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 6 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 6 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Character string of max length 16. In order to limit internal storage,
// implementations SHALL reduce the length of strings using multi-byte
// characters so that the string may be stored using "maxLength" octets in the
// given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct String16(pub String);

impl Validate for String16 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 16 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 16 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Character string of max length 20. In order to limit internal storage,
// implementations SHALL reduce the length of strings using multi-byte
// characters so that the string may be stored using "maxLength" octets in the
// given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct String20(pub String);

impl Validate for String20 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 20 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 20 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Character string of max length 32. In order to limit internal storage,
// implementations SHALL reduce the length of strings using multi-byte
// characters so that the string may be stored using "maxLength" octets in the
// given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct String32(pub String);

impl Validate for String32 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 32 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 32 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Character string of max length 42. In order to limit internal storage,
// implementations SHALL reduce the length of strings using multi-byte
// characters so that the string may be stored using "maxLength" octets in the
// given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct String42(pub String);

impl Validate for String42 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 42 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 42 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// Character string of max length 192. For all string types, in order to limit
// internal storage, implementations SHALL reduce the length of strings using
// multi-byte characters so that the string may be stored using "maxLength"
// octets in the given encoding.
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct String192(pub String);

impl Validate for String192 {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() > 192 {
            return Err(format!(
                "MaxLength validation error. \nExpected: 0 length <= 192 \nActual: 0 length == {}",
                self.0.len()
            ));
        }
        Ok(())
    }
}

// TODO: These can likely be made type aliases, not newtypes
// Unsigned integer, max inclusive 255 (2^8-1)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint8(pub u8);

impl Validate for Uint8 {}
// Unsigned integer, max inclusive 65535 (2^16-1)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint16(pub u16);

impl Validate for Uint16 {}
// Unsigned integer, max inclusive 4294967295 (2^32-1)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint32(pub u32);

impl Validate for Uint32 {}
// Unsigned integer, max inclusive 1099511627775 (2^40-1)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint40(pub u64);

impl Validate for Uint40 {
    fn validate(&self) -> Result<(), String> {
        if self.0 > "281474976710655".parse::<u64>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 281474976710655.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

// Unsigned integer, max inclusive 281474976710655 (2^48-1)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint48(pub u64);

impl Validate for Uint48 {
    fn validate(&self) -> Result<(), String> {
        if self.0 > "281474976710655".parse::<u64>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 281474976710655.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

// Unsigned integer, max inclusive 18446744073709551615 (2^64-1)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Uint64(pub u64);

impl Validate for Uint64 {}
// Signed integer, min -128 max +127
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Int8(pub i8);

impl Validate for Int8 {}
// Signed integer, min -32768 max +32767
#[derive(Default, PartialEq, PartialOrd, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Int16(pub i16);

impl Validate for Int16 {}
// Signed integer, max inclusive 2147483647 (2^31), min inclusive -2147483647
// (same as xs:int)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Int32(pub i32);

impl Validate for Int32 {}
// Signed integer, max inclusive 140737488355328 (2^47), min inclusive
// -140737488355328
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Int48(pub i64);

impl Validate for Int48 {
    fn validate(&self) -> Result<(), String> {
        if self.0 > "140737488355328".parse::<i64>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 140737488355328.\nActual: 0 == {}", self.0));
        }
        if self.0 < "-140737488355328".parse::<i64>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= -140737488355328.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

// Signed integer, max inclusive 9223372036854775807 (2^63), min inclusive
// -9223372036854775808 (same as xs:long)
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Int64(pub i64);

impl Validate for Int64 {}
// A resource is an addressable unit of information, either a collection (List)
// or instance of an object (identifiedObject, or simply, Resource)
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Resource {
    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Resource {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Response {
    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<Uint8>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Response {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct List {
    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for List {}

// Links provide a reference, via URI, to another resource.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Link {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for Link {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IdentifiedObject {
    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for IdentifiedObject {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RespondableResource {
    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RespondableResource {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RespondableIdentifiedObject {
    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RespondableIdentifiedObject {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RespondableSubscribableIdentifiedObject {
    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RespondableSubscribableIdentifiedObject {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscribableResource {
    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for SubscribableResource {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscribableList {
    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for SubscribableList {}

// Contains information about the nature of an error if a request could not be
// completed successfully.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Error {
    // Contains the number of seconds the client SHOULD wait before retrying the
    // request.
    #[yaserde(rename = "maxRetryDuration")]
    pub max_retry_duration: Option<Uint16>,

    // Code indicating the reason for failure.
    // 0 - Invalid request format
    // 1 - Invalid request values (e.g. invalid threshold values)
    // 2 - Resource limit reached
    // 3 - Conditional subscription field not supported
    // 4 - Maximum request frequency exceeded
    // All other values reserved
    #[yaserde(rename = "reasonCode")]
    pub reason_code: Uint16,
}

impl Validate for Error {}

// Current status information relevant to a specific object. The Status object
// is used to indicate the current status of an Event. Devices can read the
// containing resource (e.g. TextMessage) to get the most up to date status of
// the event. Devices can also subscribe to a specific resource instance to get
// updates when any of its attributes change, including the Status object.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EventStatus {
    // Field representing the current status type.
    // 0 = Scheduled
    // This status indicates that the event has been scheduled and the event has
    // not yet started. The server SHALL set the event to this status when the
    // event is first scheduled and persist until the event has become active or
    // has been cancelled. For events with a start time less than or equal to
    // the current time, this status SHALL never be indicated, the event SHALL
    // start with a status of “Active”.
    // 1 = Active
    // This status indicates that the event is currently active. The server
    // SHALL set the event to this status when the event reaches its earliest
    // Effective Start Time.
    // 2 = Cancelled
    // When events are cancelled, the Status.dateTime attribute SHALL be set to
    // the time the cancellation occurred, which cannot be in the future. The
    // server is responsible for maintaining the cancelled event in its
    // collection for the duration of the original event, or until the server
    // has run out of space and needs to store a new event. Client devices SHALL
    // be aware of Cancelled events, determine if the Cancelled event applies to
    // them, and cancel the event immediately if applicable.
    // 3 = Cancelled with Randomization
    // The server is responsible for maintaining the cancelled event in its
    // collection for the duration of the Effective Scheduled Period. Client
    // devices SHALL be aware of Cancelled with Randomization events, determine
    // if the Cancelled event applies to them, and cancel the event immediately,
    // using the larger of (absolute value of randomizeStart) and (absolute
    // value of randomizeDuration) as the end randomization, in seconds. This
    // Status.type SHALL NOT be used with "regular" Events, only with
    // specializations of RandomizableEvent.
    // 4 = Superseded
    // Events marked as Superseded by servers are events that may have been
    // replaced by new events from the same program that target the exact same
    // set of deviceCategory's (if applicable) AND DERControl controls (e.g.,
    // opModTargetW) (if applicable) and overlap for a given period of time.
    // Servers SHALL mark an event as Superseded at the earliest Effective Start
    // Time of the overlapping event. Servers are responsible for maintaining
    // the Superseded event in their collection for the duration of the
    // Effective Scheduled Period.
    // Client devices encountering a Superseded event SHALL terminate execution
    // of the event immediately and commence execution of the new event
    // immediately, unless the current time is within the start randomization
    // window of the superseded event, in which case the client SHALL obey the
    // start randomization of the new event. This Status.type SHALL NOT be used
    // with TextMessage, since multiple text messages can be active.
    // All other values reserved.
    #[yaserde(rename = "currentStatus")]
    pub current_status: Uint8,

    // The dateTime attribute will provide a timestamp of when the current
    // status was defined. dateTime MUST be set to the time at which the status
    // change occurred, not a time in the future or past.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    // Set to true by a server of this event if there are events that overlap
    // this event in time and also overlap in some, but not all,
    // deviceCategory's (if applicable) AND DERControl controls (e.g.,
    // opModTargetW) (if applicable) in the same function set instance.
    #[yaserde(rename = "potentiallySuperseded")]
    pub potentially_superseded: bool,

    // Indicates the time that the potentiallySuperseded flag was set.
    #[yaserde(rename = "potentiallySupersededTime")]
    pub potentially_superseded_time: Option<TimeType>,

    // The Reason attribute allows a Service provider to provide a textual
    // explanation of the status.
    #[yaserde(rename = "reason")]
    pub reason: Option<String192>,
}

impl Validate for EventStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Event {
    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Event {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RandomizableEvent {
    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval duration, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeDuration")]
    pub randomize_duration: Option<OneHourRangeType>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval start time, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeStart")]
    pub randomize_start: Option<OneHourRangeType>,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RandomizableEvent {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscribableIdentifiedObject {
    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for SubscribableIdentifiedObject {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FunctionSetAssignmentsBase {
    #[yaserde(rename = "CustomerAccountListLink")]
    pub customer_account_list_link: Option<CustomerAccountListLink>,

    #[yaserde(rename = "DemandResponseProgramListLink")]
    pub demand_response_program_list_link: Option<DemandResponseProgramListLink>,

    #[yaserde(rename = "DERProgramListLink")]
    pub der_program_list_link: Option<DerprogramListLink>,

    #[yaserde(rename = "FileListLink")]
    pub file_list_link: Option<FileListLink>,

    #[yaserde(rename = "MessagingProgramListLink")]
    pub messaging_program_list_link: Option<MessagingProgramListLink>,

    #[yaserde(rename = "PrepaymentListLink")]
    pub prepayment_list_link: Option<PrepaymentListLink>,

    #[yaserde(rename = "ResponseSetListLink")]
    pub response_set_list_link: Option<ResponseSetListLink>,

    #[yaserde(rename = "TariffProfileListLink")]
    pub tariff_profile_list_link: Option<TariffProfileListLink>,

    #[yaserde(rename = "TimeLink")]
    pub time_link: Option<TimeLink>,

    #[yaserde(rename = "UsagePointListLink")]
    pub usage_point_list_link: Option<UsagePointListLink>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FunctionSetAssignmentsBase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceCapability {
    #[yaserde(rename = "EndDeviceListLink")]
    pub end_device_list_link: Option<EndDeviceListLink>,

    #[yaserde(rename = "MirrorUsagePointListLink")]
    pub mirror_usage_point_list_link: Option<MirrorUsagePointListLink>,

    #[yaserde(rename = "SelfDeviceLink")]
    pub self_device_link: Option<SelfDeviceLink>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    #[yaserde(rename = "CustomerAccountListLink")]
    pub customer_account_list_link: Option<CustomerAccountListLink>,

    #[yaserde(rename = "DemandResponseProgramListLink")]
    pub demand_response_program_list_link: Option<DemandResponseProgramListLink>,

    #[yaserde(rename = "DERProgramListLink")]
    pub der_program_list_link: Option<DerprogramListLink>,

    #[yaserde(rename = "FileListLink")]
    pub file_list_link: Option<FileListLink>,

    #[yaserde(rename = "MessagingProgramListLink")]
    pub messaging_program_list_link: Option<MessagingProgramListLink>,

    #[yaserde(rename = "PrepaymentListLink")]
    pub prepayment_list_link: Option<PrepaymentListLink>,

    #[yaserde(rename = "ResponseSetListLink")]
    pub response_set_list_link: Option<ResponseSetListLink>,

    #[yaserde(rename = "TariffProfileListLink")]
    pub tariff_profile_list_link: Option<TariffProfileListLink>,

    #[yaserde(rename = "TimeLink")]
    pub time_link: Option<TimeLink>,

    #[yaserde(rename = "UsagePointListLink")]
    pub usage_point_list_link: Option<UsagePointListLink>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DeviceCapability {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AbstractDevice {
    #[yaserde(rename = "ConfigurationLink")]
    pub configuration_link: Option<ConfigurationLink>,

    #[yaserde(rename = "DERListLink")]
    pub der_list_link: Option<DerlistLink>,

    // This field is for use in devices that can adjust energy usage (e.g.,
    // demand response, distributed energy resources). For devices that do not
    // respond to EndDeviceControls or DERControls (for instance, an ESI), this
    // field should not have any bits set.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: Option<DeviceCategoryType>,

    #[yaserde(rename = "DeviceInformationLink")]
    pub device_information_link: Option<DeviceInformationLink>,

    #[yaserde(rename = "DeviceStatusLink")]
    pub device_status_link: Option<DeviceStatusLink>,

    #[yaserde(rename = "FileStatusLink")]
    pub file_status_link: Option<FileStatusLink>,

    #[yaserde(rename = "IPInterfaceListLink")]
    pub ip_interface_list_link: Option<IpinterfaceListLink>,

    // Long form of device identifier. See the Security section for additional
    // details.
    #[yaserde(rename = "lFDI")]
    pub l_fdi: Option<HexBinary160>,

    #[yaserde(rename = "LoadShedAvailabilityListLink")]
    pub load_shed_availability_list_link: Option<LoadShedAvailabilityListLink>,

    #[yaserde(rename = "LogEventListLink")]
    pub log_event_list_link: Option<LogEventListLink>,

    #[yaserde(rename = "PowerStatusLink")]
    pub power_status_link: Option<PowerStatusLink>,

    // Short form of device identifier, WITH the checksum digit. See the
    // Security section for additional details.
    #[yaserde(rename = "sFDI")]
    pub s_fdi: Sfditype,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for AbstractDevice {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceStatus {
    // The time at which the reported values were recorded.
    #[yaserde(rename = "changedTime")]
    pub changed_time: TimeType,

    // The number of times that the device has been turned on: Count of "device
    // on" times, since the last time the counter was reset
    #[yaserde(rename = "onCount")]
    pub on_count: Option<Uint16>,

    // Device operational state:
    // 0 - Not applicable / Unknown
    // 1 - Not operating
    // 2 - Operating
    // 3 - Starting up
    // 4 - Shutting down
    // 5 - At disconnect level
    // 6 - kW ramping
    // 7 - kVar ramping
    #[yaserde(rename = "opState")]
    pub op_state: Option<Uint8>,

    // Total time device has operated: re-settable: Accumulated time in seconds
    // since the last time the counter was reset.
    #[yaserde(rename = "opTime")]
    pub op_time: Option<Uint32>,

    #[yaserde(rename = "Temperature")]
    pub temperature: Vec<Temperature>,

    #[yaserde(rename = "TimeLink")]
    pub time_link: Option<TimeLink>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DeviceStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDevice {
    // The time at which this resource was last modified or created.
    #[yaserde(rename = "changedTime")]
    pub changed_time: TimeType,

    // This attribute indicates whether or not an EndDevice is enabled, or
    // registered, on the server. If a server sets this attribute to false, the
    // device is no longer registered. It should be noted that servers can
    // delete EndDevice instances, but using this attribute for some time is
    // more convenient for clients.
    #[yaserde(rename = "enabled")]
    pub enabled: Option<bool>,

    #[yaserde(rename = "FlowReservationRequestListLink")]
    pub flow_reservation_request_list_link: Option<FlowReservationRequestListLink>,

    #[yaserde(rename = "FlowReservationResponseListLink")]
    pub flow_reservation_response_list_link: Option<FlowReservationResponseListLink>,

    #[yaserde(rename = "FunctionSetAssignmentsListLink")]
    pub function_set_assignments_list_link: Option<FunctionSetAssignmentsListLink>,

    // POST rate, or how often EndDevice and subordinate resources should be
    // POSTed, in seconds. A client MAY indicate a preferred postRate when
    // POSTing EndDevice. A server MAY add or modify postRate to indicate its
    // preferred posting rate.
    #[yaserde(rename = "postRate")]
    pub post_rate: Option<Uint32>,

    #[yaserde(rename = "RegistrationLink")]
    pub registration_link: Option<RegistrationLink>,

    #[yaserde(rename = "SubscriptionListLink")]
    pub subscription_list_link: Option<SubscriptionListLink>,

    #[yaserde(rename = "ConfigurationLink")]
    pub configuration_link: Option<ConfigurationLink>,

    #[yaserde(rename = "DERListLink")]
    pub der_list_link: Option<DerlistLink>,

    // This field is for use in devices that can adjust energy usage (e.g.,
    // demand response, distributed energy resources). For devices that do not
    // respond to EndDeviceControls or DERControls (for instance, an ESI), this
    // field should not have any bits set.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: Option<DeviceCategoryType>,

    #[yaserde(rename = "DeviceInformationLink")]
    pub device_information_link: Option<DeviceInformationLink>,

    #[yaserde(rename = "DeviceStatusLink")]
    pub device_status_link: Option<DeviceStatusLink>,

    #[yaserde(rename = "FileStatusLink")]
    pub file_status_link: Option<FileStatusLink>,

    #[yaserde(rename = "IPInterfaceListLink")]
    pub ip_interface_list_link: Option<IpinterfaceListLink>,

    // Long form of device identifier. See the Security section for additional
    // details.
    #[yaserde(rename = "lFDI")]
    pub l_fdi: Option<HexBinary160>,

    #[yaserde(rename = "LoadShedAvailabilityListLink")]
    pub load_shed_availability_list_link: Option<LoadShedAvailabilityListLink>,

    #[yaserde(rename = "LogEventListLink")]
    pub log_event_list_link: Option<LogEventListLink>,

    #[yaserde(rename = "PowerStatusLink")]
    pub power_status_link: Option<PowerStatusLink>,

    // Short form of device identifier, WITH the checksum digit. See the
    // Security section for additional details.
    #[yaserde(rename = "sFDI")]
    pub s_fdi: Sfditype,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for EndDevice {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceList {
    #[yaserde(rename = "EndDevice")]
    pub end_device: Vec<EndDevice>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for EndDeviceList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Registration {
    // Contains the time at which this registration was created, by which
    // clients MAY prioritize information providers with the most recent
    // registrations, when no additional direction from the consumer is
    // available.
    #[yaserde(rename = "dateTimeRegistered")]
    pub date_time_registered: TimeType,

    // Contains the registration PIN number associated with the device,
    // including the checksum digit.
    #[yaserde(rename = "pIN")]
    pub p_in: Pintype,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Registration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SelfDevice {
    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    #[yaserde(rename = "ConfigurationLink")]
    pub configuration_link: Option<ConfigurationLink>,

    #[yaserde(rename = "DERListLink")]
    pub der_list_link: Option<DerlistLink>,

    // This field is for use in devices that can adjust energy usage (e.g.,
    // demand response, distributed energy resources). For devices that do not
    // respond to EndDeviceControls or DERControls (for instance, an ESI), this
    // field should not have any bits set.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: Option<DeviceCategoryType>,

    #[yaserde(rename = "DeviceInformationLink")]
    pub device_information_link: Option<DeviceInformationLink>,

    #[yaserde(rename = "DeviceStatusLink")]
    pub device_status_link: Option<DeviceStatusLink>,

    #[yaserde(rename = "FileStatusLink")]
    pub file_status_link: Option<FileStatusLink>,

    #[yaserde(rename = "IPInterfaceListLink")]
    pub ip_interface_list_link: Option<IpinterfaceListLink>,

    // Long form of device identifier. See the Security section for additional
    // details.
    #[yaserde(rename = "lFDI")]
    pub l_fdi: Option<HexBinary160>,

    #[yaserde(rename = "LoadShedAvailabilityListLink")]
    pub load_shed_availability_list_link: Option<LoadShedAvailabilityListLink>,

    #[yaserde(rename = "LogEventListLink")]
    pub log_event_list_link: Option<LogEventListLink>,

    #[yaserde(rename = "PowerStatusLink")]
    pub power_status_link: Option<PowerStatusLink>,

    // Short form of device identifier, WITH the checksum digit. See the
    // Security section for additional details.
    #[yaserde(rename = "sFDI")]
    pub s_fdi: Sfditype,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for SelfDevice {}

// Specification of a temperature.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Temperature {
    // Multiplier for 'unit'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // The subject of the temperature measurement
    // 0 - Enclosure
    // 1 - Transformer
    // 2 - HeatSink
    #[yaserde(rename = "subject")]
    pub subject: Uint8,

    // Value in Degrees Celsius (uom 23).
    #[yaserde(rename = "value")]
    pub value: Int16,
}

impl Validate for Temperature {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FunctionSetAssignments {
    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    #[yaserde(rename = "CustomerAccountListLink")]
    pub customer_account_list_link: Option<CustomerAccountListLink>,

    #[yaserde(rename = "DemandResponseProgramListLink")]
    pub demand_response_program_list_link: Option<DemandResponseProgramListLink>,

    #[yaserde(rename = "DERProgramListLink")]
    pub der_program_list_link: Option<DerprogramListLink>,

    #[yaserde(rename = "FileListLink")]
    pub file_list_link: Option<FileListLink>,

    #[yaserde(rename = "MessagingProgramListLink")]
    pub messaging_program_list_link: Option<MessagingProgramListLink>,

    #[yaserde(rename = "PrepaymentListLink")]
    pub prepayment_list_link: Option<PrepaymentListLink>,

    #[yaserde(rename = "ResponseSetListLink")]
    pub response_set_list_link: Option<ResponseSetListLink>,

    #[yaserde(rename = "TariffProfileListLink")]
    pub tariff_profile_list_link: Option<TariffProfileListLink>,

    #[yaserde(rename = "TimeLink")]
    pub time_link: Option<TimeLink>,

    #[yaserde(rename = "UsagePointListLink")]
    pub usage_point_list_link: Option<UsagePointListLink>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FunctionSetAssignments {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FunctionSetAssignmentsList {
    #[yaserde(rename = "FunctionSetAssignments")]
    pub function_set_assignments: Vec<FunctionSetAssignments>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FunctionSetAssignmentsList {}

// Indicates a condition that must be satisfied for the Notification to be
// triggered.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Condition {
    // 0 = Reading value
    // 1-255 = Reserved
    #[yaserde(rename = "attributeIdentifier")]
    pub attribute_identifier: Uint8,

    // The value of the lower threshold
    #[yaserde(rename = "lowerThreshold")]
    pub lower_threshold: Int48,

    // The value of the upper threshold
    #[yaserde(rename = "upperThreshold")]
    pub upper_threshold: Int48,
}

impl Validate for Condition {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscriptionBase {
    // The resource for which the subscription applies. Query string parameters
    // SHALL NOT be specified when subscribing to list resources. Should a query
    // string parameter be specified, servers SHALL ignore them.
    #[yaserde(rename = "subscribedResource")]
    pub subscribed_resource: String,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for SubscriptionBase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Subscription {
    #[yaserde(rename = "Condition")]
    pub condition: Option<Condition>,

    // 0 - application/sep+xml
    // 1 - application/sep-exi
    // 2-255 - reserved
    #[yaserde(rename = "encoding")]
    pub encoding: Uint8,

    // Contains the preferred schema and extensibility level indication such as
    // "+S1"
    #[yaserde(rename = "level")]
    pub level: String16,

    // This element is used to indicate the maximum number of list items that
    // should be included in a notification when the subscribed resource
    // changes. This limit is meant to be functionally equivalent to the
    // ‘limit’ query string parameter, but applies to both list resources as
    // well as other resources. For list resources, if a limit of ‘0’ is
    // specified, then notifications SHALL contain a list resource with
    // results=’0’ (equivalent to a simple change notification). For list
    // resources, if a limit greater than ‘0’ is specified, then
    // notifications SHALL contain a list resource with results equal to the
    // limit specified (or less, should the list contain fewer items than the
    // limit specified or should the server be unable to provide the requested
    // number of items for any reason) and follow the same rules for list
    // resources (e.g., ordering). For non-list resources, if a limit of ‘0’
    // is specified, then notifications SHALL NOT contain a resource
    // representation (equivalent to a simple change notification). For non-list
    // resources, if a limit greater than ‘0’ is specified, then
    // notifications SHALL contain the representation of the changed resource.
    #[yaserde(rename = "limit")]
    pub limit: Uint32,

    // The resource to which to post the notifications about the requested
    // subscribed resource. Because this URI will exist on a server other than
    // the one being POSTed to, this attribute SHALL be a fully-qualified
    // absolute URI, not a relative reference.
    #[yaserde(rename = "notificationURI")]
    pub notification_uri: String,

    // The resource for which the subscription applies. Query string parameters
    // SHALL NOT be specified when subscribing to list resources. Should a query
    // string parameter be specified, servers SHALL ignore them.
    #[yaserde(rename = "subscribedResource")]
    pub subscribed_resource: String,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Subscription {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscriptionList {
    #[yaserde(rename = "Subscription")]
    pub subscription: Vec<Subscription>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for SubscriptionList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Notification {
    // The new location of the resource, if moved. This attribute SHALL be a
    // fully-qualified absolute URI, not a relative reference.
    #[yaserde(rename = "newResourceURI")]
    pub new_resource_uri: Option<String>,

    #[yaserde(rename = "Resource")]
    pub resource: Option<Resource>,

    // 0 = Default Status
    // 1 = Subscription canceled, no additional information
    // 2 = Subscription canceled, resource moved
    // 3 = Subscription canceled, resource definition changed (e.g., a new
    // version of IEEE 2030.5)
    // 4 = Subscription canceled, resource deleted
    // All other values reserved.
    #[yaserde(rename = "status")]
    pub status: Uint8,

    // The subscription from which this notification was triggered. This
    // attribute SHALL be a fully-qualified absolute URI, not a relative
    // reference.
    #[yaserde(rename = "subscriptionURI")]
    pub subscription_uri: String,

    // The resource for which the subscription applies. Query string parameters
    // SHALL NOT be specified when subscribing to list resources. Should a query
    // string parameter be specified, servers SHALL ignore them.
    #[yaserde(rename = "subscribedResource")]
    pub subscribed_resource: String,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Notification {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct NotificationList {
    #[yaserde(rename = "Notification")]
    pub notification: Vec<Notification>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for NotificationList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DercontrolResponse {
    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<Uint8>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DercontrolResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationResponseResponse {
    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<Uint8>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FlowReservationResponseResponse {}

// Specifies the value of the TargetReduction applied by the device.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AppliedTargetReduction {
    // Enumerated field representing the type of reduction requested.
    #[yaserde(rename = "type")]
    pub _type: UnitType,

    // Indicates the requested amount of the relevant commodity to be reduced.
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for AppliedTargetReduction {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DrResponse {
    #[yaserde(rename = "ApplianceLoadReduction")]
    pub appliance_load_reduction: Option<ApplianceLoadReduction>,

    #[yaserde(rename = "AppliedTargetReduction")]
    pub applied_target_reduction: Option<AppliedTargetReduction>,

    #[yaserde(rename = "DutyCycle")]
    pub duty_cycle: Option<DutyCycle>,

    #[yaserde(rename = "Offset")]
    pub offset: Option<Offset>,

    // Indicates the amount of time, in seconds, that the client partially
    // opts-out during the demand response event. When overriding within the
    // allowed override duration, the client SHALL send a partial opt-out
    // (Response status code 8) for partial opt-out upon completion, with the
    // total time the event was overridden (this attribute) populated. The
    // client SHALL send a no participation status response (status type 10) if
    // the user partially opts-out for longer than
    // EndDeviceControl.overrideDuration.
    #[yaserde(rename = "overrideDuration")]
    pub override_duration: Option<Uint16>,

    #[yaserde(rename = "SetPoint")]
    pub set_point: Option<SetPoint>,

    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<Uint8>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DrResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PriceResponse {
    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<Uint8>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for PriceResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ResponseList {
    #[yaserde(rename = "Response")]
    pub response: Vec<Response>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ResponseList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ResponseSet {
    #[yaserde(rename = "ResponseListLink")]
    pub response_list_link: Option<ResponseListLink>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ResponseSet {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ResponseSetList {
    #[yaserde(rename = "ResponseSet")]
    pub response_set: Vec<ResponseSet>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ResponseSetList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TextResponse {
    // The createdDateTime field contains the date and time when the
    // acknowledgement/status occurred in the client. The client will provide
    // the timestamp to ensure the proper time is captured in case the response
    // is delayed in reaching the server (server receipt time would not be the
    // same as the actual confirmation time). The time reported from the client
    // should be relative to the time server indicated by the
    // FunctionSetAssignment that also indicated the event resource; if no
    // FunctionSetAssignment exists, the time of the server where the event
    // resource was hosted.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: Option<TimeType>,

    // Contains the LFDI of the device providing the response.
    #[yaserde(rename = "endDeviceLFDI")]
    pub end_device_lfdi: HexBinary160,

    // The status field contains the acknowledgement or status. Each event type
    // (DRLC, DER, Price, or Text) can return different status information (e.g.
    // an Acknowledge will be returned for a Price event where a DRLC event can
    // return Event Received, Event Started, and Event Completed). The Status
    // field value definitions are defined in Table 27: Response Types by
    // Function Set.
    #[yaserde(rename = "status")]
    pub status: Option<Uint8>,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the original object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for TextResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Time {
    // The current time, in the format defined by TimeType.
    #[yaserde(rename = "currentTime")]
    pub current_time: TimeType,

    // Time at which daylight savings ends (dstOffset no longer applied). Result
    // of dstEndRule calculation.
    #[yaserde(rename = "dstEndTime")]
    pub dst_end_time: TimeType,

    // Daylight savings time offset from local standard time. A typical practice
    // is advancing clocks one hour when daylight savings time is in effect,
    // which would result in a positive dstOffset.
    #[yaserde(rename = "dstOffset")]
    pub dst_offset: TimeOffsetType,

    // Time at which daylight savings begins (apply dstOffset). Result of
    // dstStartRule calculation.
    #[yaserde(rename = "dstStartTime")]
    pub dst_start_time: TimeType,

    // Local time: localTime = currentTime + tzOffset (+ dstOffset when in
    // effect).
    #[yaserde(rename = "localTime")]
    pub local_time: Option<TimeType>,

    // Metric indicating the quality of the time source from which the service
    // acquired time. Lower (smaller) quality enumeration values are assumed to
    // be more accurate.
    // 3 - time obtained from external authoritative source such as NTP
    // 4 - time obtained from level 3 source
    // 5 - time manually set or obtained from level 4 source
    // 6 - time obtained from level 5 source
    // 7 - time intentionally uncoordinated
    // All other values are reserved for future use.
    #[yaserde(rename = "quality")]
    pub quality: Uint8,

    // Local time zone offset from currentTime. Does not include any daylight
    // savings time offsets. For American time zones, a negative tzOffset SHALL
    // be used (eg, EST = GMT-5 which is -18000).
    #[yaserde(rename = "tzOffset")]
    pub tz_offset: TimeOffsetType,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Time {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceInformation {
    #[yaserde(rename = "DRLCCapabilities")]
    pub drlc_capabilities: Option<Drlccapabilities>,

    // Bitmap indicating the function sets used by the device as a client.
    // 0 - Device Capability
    // 1 - Self Device Resource
    // 2 - End Device Resource
    // 3 - Function Set Assignments
    // 4 - Subscription/Notification Mechanism
    // 5 - Response
    // 6 - Time
    // 7 - Device Information
    // 8 - Power Status
    // 9 - Network Status
    // 10 - Log Event
    // 11 - Configuration Resource
    // 12 - Software Download
    // 13 - DRLC
    // 14 - Metering
    // 15 - Pricing
    // 16 - Messaging
    // 17 - Billing
    // 18 - Prepayment
    // 19 - Flow Reservation
    // 20 - DER Control
    #[yaserde(rename = "functionsImplemented")]
    pub functions_implemented: Option<HexBinary64>,

    // GPS location of this device.
    #[yaserde(rename = "gpsLocation")]
    pub gps_location: Option<GpslocationType>,

    // Long form device identifier. See the Security section for full details.
    #[yaserde(rename = "lFDI")]
    pub l_fdi: HexBinary160,

    // Date/time of manufacture
    #[yaserde(rename = "mfDate")]
    pub mf_date: TimeType,

    // Manufacturer hardware version
    #[yaserde(rename = "mfHwVer")]
    pub mf_hw_ver: String32,

    // The manufacturer's IANA Enterprise Number.
    #[yaserde(rename = "mfID")]
    pub mf_id: Pentype,

    // Manufacturer dependent information related to the manufacture of this
    // device
    #[yaserde(rename = "mfInfo")]
    pub mf_info: Option<String32>,

    // Manufacturer's model number
    #[yaserde(rename = "mfModel")]
    pub mf_model: String32,

    // Manufacturer assigned serial number
    #[yaserde(rename = "mfSerNum")]
    pub mf_ser_num: String32,

    // Primary source of power.
    #[yaserde(rename = "primaryPower")]
    pub primary_power: PowerSourceType,

    // Secondary source of power
    #[yaserde(rename = "secondaryPower")]
    pub secondary_power: PowerSourceType,

    #[yaserde(rename = "SupportedLocaleListLink")]
    pub supported_locale_list_link: Option<SupportedLocaleListLink>,

    // Activation date/time of currently running software
    #[yaserde(rename = "swActTime")]
    pub sw_act_time: TimeType,

    // Currently running software version
    #[yaserde(rename = "swVer")]
    pub sw_ver: String32,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DeviceInformation {}

// Contains information about the static capabilities of the device, to allow
// service providers to know what types of functions are supported, what the
// normal operating ranges and limits are, and other similar information, in
// order to provide better suggestions of applicable programs to receive the
// maximum benefit.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Drlccapabilities {
    // The average hourly energy usage when in normal operating mode.
    #[yaserde(rename = "averageEnergy")]
    pub average_energy: RealEnergy,

    // The maximum demand rating of this end device.
    #[yaserde(rename = "maxDemand")]
    pub max_demand: ActivePower,

    // Bitmap indicating the DRLC options implemented by the device.
    // 0 - Target reduction (kWh)
    // 1 - Target reduction (kW)
    // 2 - Target reduction (Watts)
    // 3 - Target reduction (Cubic Meters)
    // 4 - Target reduction (Cubic Feet)
    // 5 - Target reduction (US Gallons)
    // 6 - Target reduction (Imperial Gallons)
    // 7 - Target reduction (BTUs)
    // 8 - Target reduction (Liters)
    // 9 - Target reduction (kPA (gauge))
    // 10 - Target reduction (kPA (absolute))
    // 11 - Target reduction (Mega Joule)
    // 12 - Target reduction (Unitless)
    // 13-15 - Reserved
    // 16 - Temperature set point
    // 17 - Temperature offset
    // 18 - Duty cycle
    // 19 - Load adjustment percentage
    // 20 - Appliance load reduction
    // 21-31 - Reserved
    #[yaserde(rename = "optionsImplemented")]
    pub options_implemented: HexBinary32,
}

impl Validate for Drlccapabilities {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupportedLocale {
    // The code for a locale that is supported
    #[yaserde(rename = "locale")]
    pub locale: LocaleType,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for SupportedLocale {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupportedLocaleList {
    #[yaserde(rename = "SupportedLocale")]
    pub supported_locale: Vec<SupportedLocale>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for SupportedLocaleList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PowerStatus {
    // Battery system status
    // 0 = unknown
    // 1 = normal (more than LowChargeThreshold remaining)
    // 2 = low (less than LowChargeThreshold remaining)
    // 3 = depleted (0% charge remaining)
    // 4 = not applicable (mains powered only)
    #[yaserde(rename = "batteryStatus")]
    pub battery_status: Uint8,

    // The time at which the reported values were recorded.
    #[yaserde(rename = "changedTime")]
    pub changed_time: TimeType,

    // This value will be fixed for devices powered by a single source. This
    // value may change for devices able to transition between multiple power
    // sources (mains to battery backup, etc.).
    #[yaserde(rename = "currentPowerSource")]
    pub current_power_source: PowerSourceType,

    // Estimate of remaining battery charge as a percent of full charge.
    #[yaserde(rename = "estimatedChargeRemaining")]
    pub estimated_charge_remaining: Option<PerCent>,

    // Estimated time (in seconds) to total battery charge depletion (under
    // current load)
    #[yaserde(rename = "estimatedTimeRemaining")]
    pub estimated_time_remaining: Option<Uint32>,

    #[yaserde(rename = "PEVInfo")]
    pub pev_info: Option<Pevinfo>,

    // If the device has a battery, this is the time since the device last
    // switched to battery power, or the time since the device was restarted,
    // whichever is less, in seconds.
    #[yaserde(rename = "sessionTimeOnBattery")]
    pub session_time_on_battery: Option<Uint32>,

    // If the device has a battery, this is the total time the device has been
    // on battery power, in seconds. It may be reset when the battery is
    // replaced.
    #[yaserde(rename = "totalTimeOnBattery")]
    pub total_time_on_battery: Option<Uint32>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for PowerStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum PowerSourceType {
    #[default]
    None = 0,
    Mains = 1,
    LocalGeneration = 3,
    Emergency = 4,
    Unknown = 5,
    // 6-255 RESERVED
}

impl Validate for PowerSourceType {}

// Contains attributes that can be exposed by PEVs and other devices that have
// charging requirements.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Pevinfo {
    // This is the actual power flow in or out of the charger or inverter. This
    // is calculated by the vehicle based on actual measurements. This number is
    // positive for charging.
    #[yaserde(rename = "chargingPowerNow")]
    pub charging_power_now: ActivePower,

    // This is the amount of energy that must be transferred from the grid to
    // EVSE and PEV to achieve the target state of charge allowing for charger
    // efficiency and any vehicle and EVSE parasitic loads. This is calculated
    // by the vehicle and changes throughout the connection as forward or
    // reverse power flow change the battery state of charge. This number is
    // positive for charging.
    #[yaserde(rename = "energyRequestNow")]
    pub energy_request_now: RealEnergy,

    // This is maximum power transfer capability that could be used for charging
    // the PEV to perform the requested energy transfer. It is the lower of the
    // vehicle or EVSE physical power limitations. It is not based on economic
    // considerations. The vehicle may draw less power than this value based on
    // its charging cycle. The vehicle defines this parameter. This number is
    // positive for charging power flow.
    #[yaserde(rename = "maxForwardPower")]
    pub max_forward_power: ActivePower,

    // This is computed by the PEV based on the charging profile to complete the
    // energy transfer if the maximum power is authorized. The value will never
    // be smaller than the ratio of the energy request to the power request
    // because the charging profile may not allow the maximum power to be used
    // throughout the transfer. This is a critical parameter for determining
    // whether any slack time exists in the charging cycle between the current
    // time and the TCIN.
    #[yaserde(rename = "minimumChargingDuration")]
    pub minimum_charging_duration: Uint32,

    // This is the target state of charge that is to be achieved during charging
    // before the time of departure (TCIN). The default value is 100%. The value
    // cannot be set to a value less than the actual state of charge.
    #[yaserde(rename = "targetStateOfCharge")]
    pub target_state_of_charge: PerCent,

    // Time Charge is Needed (TCIN) is the time that the PEV is expected to
    // depart. The value is manually entered using controls and displays in the
    // vehicle or on the EVSE or using a mobile device. It is authenticated and
    // saved by the PEV. This value may be updated during a charging session.
    #[yaserde(rename = "timeChargeIsNeeded")]
    pub time_charge_is_needed: TimeType,

    // This is the time that the parameters are updated, except for changes to
    // TCIN.
    #[yaserde(rename = "timeChargingStatusPEV")]
    pub time_charging_status_pev: TimeType,
}

impl Validate for Pevinfo {}

// Contains 802.15.4 link layer specific attributes.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Ieee802154 {
    // As defined by IEEE 802.15.4
    #[yaserde(rename = "capabilityInfo")]
    pub capability_info: HexBinary8,

    #[yaserde(rename = "NeighborListLink")]
    pub neighbor_list_link: Option<NeighborListLink>,

    // As defined by IEEE 802.15.4
    #[yaserde(rename = "shortAddress")]
    pub short_address: Uint16,
}

impl Validate for Ieee802154 {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Ipaddr {
    // An IP address value.
    #[yaserde(rename = "address")]
    pub address: HexBinary128,

    #[yaserde(rename = "RPLInstanceListLink")]
    pub rpl_instance_list_link: Option<RplinstanceListLink>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Ipaddr {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IpaddrList {
    #[yaserde(rename = "IPAddr")]
    pub ip_addr: Vec<Ipaddr>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for IpaddrList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Ipinterface {
    // Use rules from [RFC 2863].
    #[yaserde(rename = "ifDescr")]
    pub if_descr: Option<String192>,

    // Use rules from [RFC 2863].
    #[yaserde(rename = "ifHighSpeed")]
    pub if_high_speed: Option<Uint32>,

    // Use rules from [RFC 2863].
    #[yaserde(rename = "ifInBroadcastPkts")]
    pub if_in_broadcast_pkts: Option<Uint32>,

    // Use rules from [RFC 2863].
    #[yaserde(rename = "ifIndex")]
    pub if_index: Option<Uint32>,

    // Use rules from [RFC 2863]. Can be thought of as Input Datagrams
    // Discarded.
    #[yaserde(rename = "ifInDiscards")]
    pub if_in_discards: Option<Uint32>,

    // Use rules from [RFC 2863].
    #[yaserde(rename = "ifInErrors")]
    pub if_in_errors: Option<Uint32>,

    // Use rules from [RFC 2863]. Can be thought of as Multicast Datagrams
    // Received.
    #[yaserde(rename = "ifInMulticastPkts")]
    pub if_in_multicast_pkts: Option<Uint32>,

    // Use rules from [RFC 2863]. Can be thought of as Bytes Received.
    #[yaserde(rename = "ifInOctets")]
    pub if_in_octets: Option<Uint32>,

    // Use rules from [RFC 2863]. Can be thought of as Datagrams Received.
    #[yaserde(rename = "ifInUcastPkts")]
    pub if_in_ucast_pkts: Option<Uint32>,

    // Use rules from [RFC 2863]. Can be thought of as Datagrams with Unknown
    // Protocol Received.
    #[yaserde(rename = "ifInUnknownProtos")]
    pub if_in_unknown_protos: Option<Uint32>,

    // Use rules from [RFC 2863].
    #[yaserde(rename = "ifMtu")]
    pub if_mtu: Option<Uint32>,

    // Use rules from [RFC 2863].
    #[yaserde(rename = "ifName")]
    pub if_name: Option<String16>,

    // Use rules and assignments from [RFC 2863].
    #[yaserde(rename = "ifOperStatus")]
    pub if_oper_status: Option<Uint8>,

    // Use rules from [RFC 2863]. Can be thought of as Broadcast Datagrams Sent.
    #[yaserde(rename = "ifOutBroadcastPkts")]
    pub if_out_broadcast_pkts: Option<Uint32>,

    // Use rules from [RFC 2863]. Can be thought of as Output Datagrams
    // Discarded.
    #[yaserde(rename = "ifOutDiscards")]
    pub if_out_discards: Option<Uint32>,

    // Use rules from [RFC 2863].
    #[yaserde(rename = "ifOutErrors")]
    pub if_out_errors: Option<Uint32>,

    // Use rules from [RFC 2863]. Can be thought of as Multicast Datagrams Sent.
    #[yaserde(rename = "ifOutMulticastPkts")]
    pub if_out_multicast_pkts: Option<Uint32>,

    // Use rules from [RFC 2863]. Can be thought of as Bytes Sent.
    #[yaserde(rename = "ifOutOctets")]
    pub if_out_octets: Option<Uint32>,

    // Use rules from [RFC 2863]. Can be thought of as Datagrams Sent.
    #[yaserde(rename = "ifOutUcastPkts")]
    pub if_out_ucast_pkts: Option<Uint32>,

    // Use rules from [RFC 2863].
    #[yaserde(rename = "ifPromiscuousMode")]
    pub if_promiscuous_mode: Option<bool>,

    // Use rules from [RFC 2863].
    #[yaserde(rename = "ifSpeed")]
    pub if_speed: Option<Uint32>,

    // Use rules and assignments from [RFC 2863].
    #[yaserde(rename = "ifType")]
    pub if_type: Option<Uint16>,

    #[yaserde(rename = "IPAddrListLink")]
    pub ip_addr_list_link: Option<IpaddrListLink>,

    // Similar to ifLastChange in [RFC 2863].
    #[yaserde(rename = "lastResetTime")]
    pub last_reset_time: Option<Int64>,

    // The date/time of the reported status.
    #[yaserde(rename = "lastUpdatedTime")]
    pub last_updated_time: Option<Int64>,

    #[yaserde(rename = "LLInterfaceListLink")]
    pub ll_interface_list_link: Option<LlinterfaceListLink>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Ipinterface {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IpinterfaceList {
    #[yaserde(rename = "IPInterface")]
    pub ip_interface: Vec<Ipinterface>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for IpinterfaceList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Llinterface {
    // Contains the number of CRC errors since reset.
    #[yaserde(rename = "CRCerrors")]
    pub cr_cerrors: Uint32,

    // Contains the EUI-64 of the link layer interface. 48 bit MAC addresses
    // SHALL be changed into an EUI-64 using the method defined in [RFC 4291],
    // Appendix A. (The method is to insert "0xFFFE" as described in the
    // reference.)
    #[yaserde(rename = "EUI64")]
    pub eui64: HexBinary64,

    #[yaserde(rename = "IEEE_802_15_4")]
    pub ieee_802_15_4: Option<Ieee802154>,

    // Specifies the type of link layer interface associated with the
    // IPInterface. Values are below.
    // 0 = Unspecified
    // 1 = IEEE 802.3 (Ethernet)
    // 2 = IEEE 802.11 (WLAN)
    // 3 = IEEE 802.15 (PAN)
    // 4 = IEEE 1901 (PLC)
    // All other values reserved.
    #[yaserde(rename = "linkLayerType")]
    pub link_layer_type: Uint8,

    // Number of times an ACK was not received for a frame transmitted (when ACK
    // was requested).
    #[yaserde(rename = "LLAckNotRx")]
    pub ll_ack_not_rx: Option<Uint32>,

    // Number of times CSMA failed.
    #[yaserde(rename = "LLCSMAFail")]
    pub llcsma_fail: Option<Uint32>,

    // Number of dropped receive frames.
    #[yaserde(rename = "LLFramesDropRx")]
    pub ll_frames_drop_rx: Option<Uint32>,

    // Number of dropped transmit frames.
    #[yaserde(rename = "LLFramesDropTx")]
    pub ll_frames_drop_tx: Option<Uint32>,

    // Number of link layer frames received.
    #[yaserde(rename = "LLFramesRx")]
    pub ll_frames_rx: Option<Uint32>,

    // Number of link layer frames transmitted.
    #[yaserde(rename = "LLFramesTx")]
    pub ll_frames_tx: Option<Uint32>,

    // Number of times access to media failed.
    #[yaserde(rename = "LLMediaAccessFail")]
    pub ll_media_access_fail: Option<Uint32>,

    // Number of Bytes received.
    #[yaserde(rename = "LLOctetsRx")]
    pub ll_octets_rx: Option<Uint32>,

    // Number of Bytes transmitted.
    #[yaserde(rename = "LLOctetsTx")]
    pub ll_octets_tx: Option<Uint32>,

    // Number of MAC transmit retries.
    #[yaserde(rename = "LLRetryCount")]
    pub ll_retry_count: Option<Uint32>,

    // Number of receive security errors.
    #[yaserde(rename = "LLSecurityErrorRx")]
    pub ll_security_error_rx: Option<Uint32>,

    #[yaserde(rename = "loWPAN")]
    pub lo_wpan: Option<LoWPAN>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Llinterface {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LlinterfaceList {
    #[yaserde(rename = "LLInterface")]
    pub ll_interface: Vec<Llinterface>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for LlinterfaceList {}

// Contains information specific to 6LoWPAN.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LoWPAN {
    // Number of Bytes received
    #[yaserde(rename = "octetsRx")]
    pub octets_rx: Option<Uint32>,

    // Number of Bytes transmitted
    #[yaserde(rename = "octetsTx")]
    pub octets_tx: Option<Uint32>,

    // Number of packets received
    #[yaserde(rename = "packetsRx")]
    pub packets_rx: Uint32,

    // Number of packets transmitted
    #[yaserde(rename = "packetsTx")]
    pub packets_tx: Uint32,

    // Number of errors receiving fragments
    #[yaserde(rename = "rxFragError")]
    pub rx_frag_error: Uint32,
}

impl Validate for LoWPAN {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Neighbor {
    // True if the neighbor is a child.
    #[yaserde(rename = "isChild")]
    pub is_child: bool,

    // The quality of the link, as defined by 802.15.4
    #[yaserde(rename = "linkQuality")]
    pub link_quality: Uint8,

    // As defined by IEEE 802.15.4
    #[yaserde(rename = "shortAddress")]
    pub short_address: Uint16,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Neighbor {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct NeighborList {
    #[yaserde(rename = "Neighbor")]
    pub neighbor: Vec<Neighbor>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for NeighborList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Rplinstance {
    // See [RFC 6550].
    #[yaserde(rename = "DODAGid")]
    pub doda_gid: Uint8,

    // See [RFC 6550].
    #[yaserde(rename = "DODAGroot")]
    pub doda_groot: bool,

    // See [RFC 6550].
    #[yaserde(rename = "flags")]
    pub flags: Uint8,

    // See [RFC 6550].
    #[yaserde(rename = "groundedFlag")]
    pub grounded_flag: bool,

    // See [RFC 6550].
    #[yaserde(rename = "MOP")]
    pub mop: Uint8,

    // See [RFC 6550].
    #[yaserde(rename = "PRF")]
    pub prf: Uint8,

    // See [RFC 6550].
    #[yaserde(rename = "rank")]
    pub rank: Uint16,

    // See [RFC 6550].
    #[yaserde(rename = "RPLInstanceID")]
    pub rpl_instance_id: Uint8,

    #[yaserde(rename = "RPLSourceRoutesListLink")]
    pub rpl_source_routes_list_link: Option<RplsourceRoutesListLink>,

    // See [RFC 6550].
    #[yaserde(rename = "versionNumber")]
    pub version_number: Uint8,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Rplinstance {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RplinstanceList {
    #[yaserde(rename = "RPLInstance")]
    pub rpl_instance: Vec<Rplinstance>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RplinstanceList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RplsourceRoutes {
    // See [RFC 6554].
    #[yaserde(rename = "DestAddress")]
    pub dest_address: HexBinary128,

    // See [RFC 6554].
    #[yaserde(rename = "SourceRoute")]
    pub source_route: HexBinary128,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RplsourceRoutes {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RplsourceRoutesList {
    #[yaserde(rename = "RPLSourceRoutes")]
    pub rpl_source_routes: Vec<RplsourceRoutes>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RplsourceRoutesList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LogEvent {
    // The date and time that the event occurred.
    #[yaserde(rename = "createdDateTime")]
    pub created_date_time: TimeType,

    // Human readable text that MAY be used to transmit additional details about
    // the event. A host MAY remove this field when received.
    #[yaserde(rename = "details")]
    pub details: Option<String32>,

    // May be used to transmit additional details about the event.
    #[yaserde(rename = "extendedData")]
    pub extended_data: Option<Uint32>,

    // If the profileID indicates this is IEEE 2030.5, the functionSet is
    // defined by IEEE 2030.5 and SHALL be one of the values from the table
    // below (IEEE 2030.5 function set identifiers). If the profileID is
    // anything else, the functionSet is defined by the identified profile.
    // 0 General (not specific to a function set)
    // 1 Publish and Subscribe
    // 2 End Device
    // 3 Function Set Assignment
    // 4 Response
    // 5 Demand Response and Load Control
    // 6 Metering
    // 7 Pricing
    // 8 Messaging
    // 9 Billing
    // 10 Prepayment
    // 11 Distributed Energy Resources
    // 12 Time
    // 13 Software Download
    // 14 Device Information
    // 15 Power Status
    // 16 Network Status
    // 17 Log Event List
    // 18 Configuration
    // 19 Security
    // All other values are reserved.
    #[yaserde(rename = "functionSet")]
    pub function_set: Uint8,

    // An 8 bit unsigned integer. logEventCodes are scoped to a profile and a
    // function set. If the profile is IEEE 2030.5, the logEventCode is defined
    // by IEEE 2030.5 within one of the function sets of IEEE 2030.5. If the
    // profile is anything else, the logEventCode is defined by the specified
    // profile.
    #[yaserde(rename = "logEventCode")]
    pub log_event_code: Uint8,

    // This 16-bit value, combined with createdDateTime, profileID, and
    // logEventPEN, should provide a reasonable level of uniqueness.
    #[yaserde(rename = "logEventID")]
    pub log_event_id: Uint16,

    // The Private Enterprise Number(PEN) of the entity that defined the
    // profileID, functionSet, and logEventCode of the logEvent. IEEE
    // 2030.5-assigned logEventCodes SHALL use the IEEE 2030.5 PEN. Combinations
    // of profileID, functionSet, and logEventCode SHALL have unique meaning
    // within a logEventPEN and are defined by the owner of the PEN.
    #[yaserde(rename = "logEventPEN")]
    pub log_event_pen: Pentype,

    // The profileID identifies which profile (HA, BA, SE, etc) defines the
    // following event information.
    // 0 Not profile specific.
    // 1 Vendor Defined
    // 2 IEEE 2030.5
    // 3 Home Automation
    // 4 Building Automation
    // All other values are reserved.
    #[yaserde(rename = "profileID")]
    pub profile_id: Uint8,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for LogEvent {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LogEventList {
    #[yaserde(rename = "LogEvent")]
    pub log_event: Vec<LogEvent>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for LogEventList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Configuration {
    // [RFC 4646] identifier of the language-region currently in use.
    #[yaserde(rename = "currentLocale")]
    pub current_locale: LocaleType,

    #[yaserde(rename = "PowerConfiguration")]
    pub power_configuration: Vec<PowerConfiguration>,

    #[yaserde(rename = "PriceResponseCfgListLink")]
    pub price_response_cfg_list_link: Option<PriceResponseCfgListLink>,

    #[yaserde(rename = "TimeConfiguration")]
    pub time_configuration: Vec<TimeConfiguration>,

    // User assigned, convenience name used for network browsing displays, etc.
    // Example "My Thermostat"
    #[yaserde(rename = "userDeviceName")]
    pub user_device_name: String32,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Configuration {}

// Contains configuration related to the device's power sources
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PowerConfiguration {
    // Time/Date at which battery was installed,
    #[yaserde(rename = "batteryInstallTime")]
    pub battery_install_time: Option<TimeType>,

    // In context of the PowerStatus resource, this is the value of
    // EstimatedTimeRemaining below which BatteryStatus "low" is indicated and
    // the PS_LOW_BATTERY is raised.
    #[yaserde(rename = "lowChargeThreshold")]
    pub low_charge_threshold: Option<Uint32>,
}

impl Validate for PowerConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PriceResponseCfg {
    // Price responsive clients acting upon the associated RateComponent SHOULD
    // consume the associated commodity while the price is less than this
    // threshold.
    #[yaserde(rename = "consumeThreshold")]
    pub consume_threshold: Int32,

    // Price responsive clients acting upon the associated RateComponent SHOULD
    // reduce consumption to the maximum extent possible while the price is
    // greater than this threshold.
    #[yaserde(rename = "maxReductionThreshold")]
    pub max_reduction_threshold: Int32,

    #[yaserde(rename = "RateComponentLink")]
    pub rate_component_link: RateComponentLink,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for PriceResponseCfg {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PriceResponseCfgList {
    #[yaserde(rename = "PriceResponseCfg")]
    pub price_response_cfg: Vec<PriceResponseCfg>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for PriceResponseCfgList {}

// Contains attributes related to the configuration of the time service.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TimeConfiguration {
    // Rule to calculate end of daylight savings time in the current year.
    // Result of dstEndRule must be greater than result of dstStartRule.
    #[yaserde(rename = "dstEndRule")]
    pub dst_end_rule: DstRuleType,

    // Daylight savings time offset from local standard time.
    #[yaserde(rename = "dstOffset")]
    pub dst_offset: TimeOffsetType,

    // Rule to calculate start of daylight savings time in the current year.
    // Result of dstEndRule must be greater than result of dstStartRule.
    #[yaserde(rename = "dstStartRule")]
    pub dst_start_rule: DstRuleType,

    // Local time zone offset from UTCTime. Does not include any daylight
    // savings time offsets.
    #[yaserde(rename = "tzOffset")]
    pub tz_offset: TimeOffsetType,
}

impl Validate for TimeConfiguration {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct File {
    // This element MUST be set to the date/time at which this file is
    // activated. If the activation time is less than or equal to current time,
    // the LD MUST immediately place the file into the activated state (in the
    // case of a firmware file, the file is now the running image). If the
    // activation time is greater than the current time, the LD MUST wait until
    // the specified activation time is reached, then MUST place the file into
    // the activated state. Omission of this element means that the LD MUST NOT
    // take any action to activate the file until a subsequent GET to this File
    // resource provides an activateTime.
    #[yaserde(rename = "activateTime")]
    pub activate_time: Option<TimeType>,

    // This element MUST be set to the URI location of the file binary artifact.
    // This is the BLOB (binary large object) that is actually loaded by the LD
    #[yaserde(rename = "fileURI")]
    pub file_uri: String,

    // This element MUST be set to the LFDI of the device for which this file in
    // targeted.
    #[yaserde(rename = "lFDI")]
    pub l_fdi: Option<HexBinary160>,

    // This element MUST be set to the hardware version for which this file is
    // targeted.
    #[yaserde(rename = "mfHwVer")]
    pub mf_hw_ver: Option<String32>,

    // This element MUST be set to the manufacturer's Private Enterprise Number
    // (assigned by IANA).
    #[yaserde(rename = "mfID")]
    pub mf_id: Pentype,

    // This element MUST be set to the manufacturer model number for which this
    // file is targeted. The syntax and semantics are left to the manufacturer.
    #[yaserde(rename = "mfModel")]
    pub mf_model: String32,

    // This element MUST be set to the manufacturer serial number for which this
    // file is targeted. The syntax and semantics are left to the manufacturer.
    #[yaserde(rename = "mfSerNum")]
    pub mf_ser_num: Option<String32>,

    // This element MUST be set to the software version information for this
    // file. The syntax and semantics are left to the manufacturer.
    #[yaserde(rename = "mfVer")]
    pub mf_ver: String16,

    // This element MUST be set to the total size (in bytes) of the file
    // referenced by fileURI.
    #[yaserde(rename = "size")]
    pub size: Uint32,

    // A value indicating the type of the file. MUST be one of the following
    // values:
    // 00 = Software Image
    // 01 = Security Credential
    // 02 = Configuration
    // 03 = Log
    // 04–7FFF = reserved
    // 8000-FFFF = Manufacturer defined
    #[yaserde(rename = "type")]
    pub _type: HexBinary16,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for File {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FileList {
    #[yaserde(rename = "File")]
    pub file: Vec<File>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FileList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FileStatus {
    // Date/time at which this File, referred to by FileLink, will be activated.
    // Omission of or presence and value of this element MUST exactly match
    // omission or presence and value of the activateTime element from the File
    // resource.
    #[yaserde(rename = "activateTime")]
    pub activate_time: Option<TimeType>,

    #[yaserde(rename = "FileLink")]
    pub file_link: Option<FileLink>,

    // This element MUST be set to the percentage of the file, indicated by
    // FileLink, that was loaded during the latest load attempt. This value MUST
    // be reset to 0 each time a load attempt is started for the File indicated
    // by FileLink. This value MUST be increased when an LD receives HTTP
    // response containing file content. This value MUST be set to 100 when the
    // full content of the file has been received by the LD
    #[yaserde(rename = "loadPercent")]
    pub load_percent: Uint8,

    // This element MUST be set to the time at which the LD will issue its next
    // GET request for file content from the File indicated by FileLink
    #[yaserde(rename = "nextRequestAttempt")]
    pub next_request_attempt: TimeType,

    // This value MUST be reset to 0 when FileLink is first pointed at a new
    // File. This value MUST be incremented each time an
    // LD receives a 503 error from the FS.
    #[yaserde(rename = "request503Count")]
    pub request_503_count: Uint16,

    // This value MUST be reset to 0 when FileLink is first pointed at a new
    // File. This value MUST be incremented each time a GET request for file
    // content failed. 503 errors MUST be excluded from this counter.
    #[yaserde(rename = "requestFailCount")]
    pub request_fail_count: Uint16,

    // Current loading status of the file indicated by FileLink. This element
    // MUST be set to one of the following values:
    // 0 - No load operation in progress
    // 1 - File load in progress (first request for file content has been issued
    // by LD)
    // 2 - File load failed
    // 3 - File loaded successfully (full content of file has been received by
    // the LD), signature verification in progress
    // 4 - File signature verification failed
    // 5 - File signature verified, waiting to activate file.
    // 6 - File activation failed
    // 7 - File activation in progress
    // 8 - File activated successfully (this state may not be reached/persisted
    // through an image activation)
    // 9-255 - Reserved for future use.
    #[yaserde(rename = "status")]
    pub status: Uint8,

    // This element MUST be set to the time at which file status transitioned to
    // the value indicated in the status element.
    #[yaserde(rename = "statusTime")]
    pub status_time: TimeType,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FileStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LoadShedAvailabilityList {
    #[yaserde(rename = "LoadShedAvailability")]
    pub load_shed_availability: Vec<LoadShedAvailability>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for LoadShedAvailabilityList {}

// The ApplianceLoadReduction object is used by a Demand Response service
// provider to provide signals for ENERGY STAR compliant appliances. See the
// definition of ApplianceLoadReductionType for more information.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ApplianceLoadReduction {
    // Indicates the type of appliance load reduction requested.
    #[yaserde(rename = "type")]
    pub _type: ApplianceLoadReductionType,
}

impl Validate for ApplianceLoadReduction {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DemandResponseProgram {
    #[yaserde(rename = "ActiveEndDeviceControlListLink")]
    pub active_end_device_control_list_link: Option<ActiveEndDeviceControlListLink>,

    // This attribute allows program providers to specify the requested
    // granularity of updates to LoadShedAvailability sheddablePercent. If not
    // present, or set to 0, then updates to LoadShedAvailability SHALL NOT be
    // provided. If present and greater than zero, then clients SHALL provide
    // their LoadShedAvailability if it has not previously been provided, and
    // thereafter if the difference between the previously provided value and
    // the current value of LoadShedAvailability sheddablePercent is greater
    // than availabilityUpdatePercentChangeThreshold.
    #[yaserde(rename = "availabilityUpdatePercentChangeThreshold")]
    pub availability_update_percent_change_threshold: Option<PerCent>,

    // This attribute allows program providers to specify the requested
    // granularity of updates to LoadShedAvailability sheddablePower. If not
    // present, or set to 0, then updates to LoadShedAvailability SHALL NOT be
    // provided. If present and greater than zero, then clients SHALL provide
    // their LoadShedAvailability if it has not previously been provided, and
    // thereafter if the difference between the previously provided value and
    // the current value of LoadShedAvailability sheddablePower is greater than
    // availabilityUpdatePowerChangeThreshold.
    #[yaserde(rename = "availabilityUpdatePowerChangeThreshold")]
    pub availability_update_power_change_threshold: Option<ActivePower>,

    #[yaserde(rename = "EndDeviceControlListLink")]
    pub end_device_control_list_link: Option<EndDeviceControlListLink>,

    // Indicates the relative primacy of the provider of this program.
    #[yaserde(rename = "primacy")]
    pub primacy: PrimacyType,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DemandResponseProgram {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DemandResponseProgramList {
    #[yaserde(rename = "DemandResponseProgram")]
    pub demand_response_program: Vec<DemandResponseProgram>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DemandResponseProgramList {}

// Duty cycle control is a device specific issue and is managed by the device.
// The duty cycle of the device under control should span the shortest practical
// time period in accordance with the nature of the device under control and the
// intent of the request for demand reduction. The default factory setting
// SHOULD be three minutes for each 10% of duty cycle. This indicates that the
// default time period over which a duty cycle is applied is 30 minutes, meaning
// a 10% duty cycle would cause a device to be ON for 3 minutes. The “off
// state” SHALL precede the “on state”.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DutyCycle {
    // Contains the maximum On state duty cycle applied by the end device, as a
    // percentage of time. The field not present indicates that this field has
    // not been used by the end device.
    #[yaserde(rename = "normalValue")]
    pub normal_value: Uint8,
}

impl Validate for DutyCycle {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceControl {
    #[yaserde(rename = "ApplianceLoadReduction")]
    pub appliance_load_reduction: Option<ApplianceLoadReduction>,

    // Specifies the bitmap indicating the categories of devices that SHOULD
    // respond. Devices SHOULD ignore events that do not indicate their device
    // category.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: DeviceCategoryType,

    // A flag to indicate if the EndDeviceControl is considered a mandatory
    // event as defined by the service provider issuing the EndDeviceControl.
    // The drProgramMandatory flag alerts the client/user that they will be
    // subject to penalty or ineligibility based on the service provider’s
    // program rules for that deviceCategory.
    #[yaserde(rename = "drProgramMandatory")]
    pub dr_program_mandatory: bool,

    #[yaserde(rename = "DutyCycle")]
    pub duty_cycle: Option<DutyCycle>,

    // Indicates that the event intends to increase consumption. A value of true
    // indicates the intention to increase usage value, and a value of false
    // indicates the intention to decrease usage.
    #[yaserde(rename = "loadShiftForward")]
    pub load_shift_forward: bool,

    #[yaserde(rename = "Offset")]
    pub offset: Option<Offset>,

    // The overrideDuration attribute provides a duration, in seconds, for which
    // a client device is allowed to override this EndDeviceControl and still
    // meet the contractual agreement with a service provider without opting
    // out. If overrideDuration is not specified, then it SHALL default to 0.
    #[yaserde(rename = "overrideDuration")]
    pub override_duration: Option<Uint16>,

    #[yaserde(rename = "SetPoint")]
    pub set_point: Option<SetPoint>,

    #[yaserde(rename = "TargetReduction")]
    pub target_reduction: Option<TargetReduction>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval duration, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeDuration")]
    pub randomize_duration: Option<OneHourRangeType>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval start time, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeStart")]
    pub randomize_start: Option<OneHourRangeType>,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for EndDeviceControl {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceControlList {
    #[yaserde(rename = "EndDeviceControl")]
    pub end_device_control: Vec<EndDeviceControl>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for EndDeviceControlList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LoadShedAvailability {
    // Indicates for how many seconds the consuming device will be able to
    // reduce consumption at the maximum response level.
    #[yaserde(rename = "availabilityDuration")]
    pub availability_duration: Option<Uint32>,

    #[yaserde(rename = "DemandResponseProgramLink")]
    pub demand_response_program_link: Option<DemandResponseProgramLink>,

    // Maximum percent of current operating load that is estimated to be
    // sheddable.
    #[yaserde(rename = "sheddablePercent")]
    pub sheddable_percent: Option<PerCent>,

    // Maximum amount of current operating load that is estimated to be
    // sheddable, in Watts.
    #[yaserde(rename = "sheddablePower")]
    pub sheddable_power: Option<ActivePower>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for LoadShedAvailability {}

// If a temperature offset is sent that causes the heating or cooling
// temperature set point to exceed the limit boundaries that are programmed into
// the device, the device SHALL respond by setting the temperature at the limit.
// If an EDC is being targeted at multiple devices or to a device that controls
// multiple devices (e.g., EMS), it can provide multiple Offset types within one
// EDC. For events with multiple Offset types, a client SHALL select the Offset
// that best fits their operating function.
// Alternatively, an event with a single Offset type can be targeted at an EMS
// in order to request a percentage load reduction on the average energy usage
// of the entire premise. An EMS SHOULD use the Metering function set to
// determine the initial load in the premise, reduce energy consumption by
// controlling devices at its disposal, and at the conclusion of the event, once
// again use the Metering function set to determine if the desired load
// reduction was achieved.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Offset {
    // The value change requested for the cooling offset, in degree C / 10. The
    // value should be added to the normal set point for cooling, or if
    // loadShiftForward is true, then the value should be subtracted from the
    // normal set point.
    #[yaserde(rename = "coolingOffset")]
    pub cooling_offset: Option<Uint8>,

    // The value change requested for the heating offset, in degree C / 10. The
    // value should be subtracted for heating, or if loadShiftForward is true,
    // then the value should be added to the normal set point.
    #[yaserde(rename = "heatingOffset")]
    pub heating_offset: Option<Uint8>,

    // The value change requested for the load adjustment percentage. The value
    // should be subtracted from the normal setting, or if loadShiftForward is
    // true, then the value should be added to the normal setting.
    #[yaserde(rename = "loadAdjustmentPercentageOffset")]
    pub load_adjustment_percentage_offset: Option<PerCent>,
}

impl Validate for Offset {}

// The SetPoint object is used to apply specific temperature set points to a
// temperature control device. The values of the heatingSetpoint and
// coolingSetpoint attributes SHALL be calculated as follows:
// Cooling/Heating Temperature Set Point / 100 = temperature in degrees Celsius
// where -273.15°C &lt;= temperature &lt;= 327.67°C, corresponding to a
// Cooling and/or Heating Temperature Set Point. The maximum resolution this
// format allows is 0.01°C.
// The field not present in a Response indicates that this field has not been
// used by the end device.
// If a temperature is sent that exceeds the temperature limit boundaries that
// are programmed into the device, the device SHALL respond by setting the
// temperature at the limit.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SetPoint {
    // This attribute represents the cooling temperature set point in degrees
    // Celsius / 100. (Hundredths of a degree C)
    #[yaserde(rename = "coolingSetpoint")]
    pub cooling_setpoint: Option<Int16>,

    // This attribute represents the heating temperature set point in degrees
    // Celsius / 100. (Hundredths of a degree C)
    #[yaserde(rename = "heatingSetpoint")]
    pub heating_setpoint: Option<Int16>,
}

impl Validate for SetPoint {}

// The TargetReduction object is used by a Demand Response service provider to
// provide a RECOMMENDED threshold that a device/premises should maintain its
// consumption below. For example, a service provider can provide a RECOMMENDED
// threshold of some kWh for a 3-hour event. This means that the device/premises
// would maintain its consumption below the specified limit for the specified
// period.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TargetReduction {
    // Indicates the type of reduction requested.
    #[yaserde(rename = "type")]
    pub _type: UnitType,

    // Indicates the requested amount of the relevant commodity to be reduced.
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for TargetReduction {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MeterReadingBase {
    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MeterReadingBase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MeterReading {
    #[yaserde(rename = "RateComponentListLink")]
    pub rate_component_list_link: Option<RateComponentListLink>,

    #[yaserde(rename = "ReadingLink")]
    pub reading_link: Option<ReadingLink>,

    #[yaserde(rename = "ReadingSetListLink")]
    pub reading_set_list_link: Option<ReadingSetListLink>,

    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: ReadingTypeLink,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MeterReading {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MeterReadingList {
    #[yaserde(rename = "MeterReading")]
    pub meter_reading: Vec<MeterReading>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MeterReadingList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingBase {
    // Indicates the consumption block related to the reading. REQUIRED if
    // ReadingType numberOfConsumptionBlocks is non-zero. If not specified, is
    // assumed to be "0 - N/A".
    #[yaserde(rename = "consumptionBlock")]
    pub consumption_block: Option<ConsumptionBlockType>,

    // List of codes indicating the quality of the reading, using specification:
    // Bit 0 - valid: data that has gone through all required validation checks
    // and either passed them all or has been verified
    // Bit 1 - manually edited: Replaced or approved by a human
    // Bit 2 - estimated using reference day: data value was replaced by a
    // machine computed value based on analysis of historical data using the
    // same type of measurement.
    // Bit 3 - estimated using linear interpolation: data value was computed
    // using linear interpolation based on the readings before and after it
    // Bit 4 - questionable: data that has failed one or more checks
    // Bit 5 - derived: data that has been calculated (using logic or
    // mathematical operations), not necessarily measured directly
    // Bit 6 - projected (forecast): data that has been calculated as a
    // projection or forecast of future readings
    #[yaserde(rename = "qualityFlags")]
    pub quality_flags: Option<HexBinary16>,

    // The time interval associated with the reading. If not specified, then
    // defaults to the intervalLength specified in the associated ReadingType.
    #[yaserde(rename = "timePeriod")]
    pub time_period: Option<DateTimeInterval>,

    // Indicates the time of use tier related to the reading. REQUIRED if
    // ReadingType numberOfTouTiers is non-zero. If not specified, is assumed to
    // be "0 - N/A".
    #[yaserde(rename = "touTier")]
    pub tou_tier: Option<Toutype>,

    // Value in units specified by ReadingType
    #[yaserde(rename = "value")]
    pub value: Option<Int48>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ReadingBase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Reading {
    // The local identifier for this reading within the reading set. localIDs
    // are assigned in order of creation time. For interval data, this value
    // SHALL increase with each interval time, and for block/tier readings,
    // localID SHALL not be specified.
    #[yaserde(rename = "localID")]
    pub local_id: Option<HexBinary16>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // Indicates the consumption block related to the reading. REQUIRED if
    // ReadingType numberOfConsumptionBlocks is non-zero. If not specified, is
    // assumed to be "0 - N/A".
    #[yaserde(rename = "consumptionBlock")]
    pub consumption_block: Option<ConsumptionBlockType>,

    // List of codes indicating the quality of the reading, using specification:
    // Bit 0 - valid: data that has gone through all required validation checks
    // and either passed them all or has been verified
    // Bit 1 - manually edited: Replaced or approved by a human
    // Bit 2 - estimated using reference day: data value was replaced by a
    // machine computed value based on analysis of historical data using the
    // same type of measurement.
    // Bit 3 - estimated using linear interpolation: data value was computed
    // using linear interpolation based on the readings before and after it
    // Bit 4 - questionable: data that has failed one or more checks
    // Bit 5 - derived: data that has been calculated (using logic or
    // mathematical operations), not necessarily measured directly
    // Bit 6 - projected (forecast): data that has been calculated as a
    // projection or forecast of future readings
    #[yaserde(rename = "qualityFlags")]
    pub quality_flags: Option<HexBinary16>,

    // The time interval associated with the reading. If not specified, then
    // defaults to the intervalLength specified in the associated ReadingType.
    #[yaserde(rename = "timePeriod")]
    pub time_period: Option<DateTimeInterval>,

    // Indicates the time of use tier related to the reading. REQUIRED if
    // ReadingType numberOfTouTiers is non-zero. If not specified, is assumed to
    // be "0 - N/A".
    #[yaserde(rename = "touTier")]
    pub tou_tier: Option<Toutype>,

    // Value in units specified by ReadingType
    #[yaserde(rename = "value")]
    pub value: Option<Int48>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Reading {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingList {
    #[yaserde(rename = "Reading")]
    pub reading: Vec<Reading>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ReadingList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingSetBase {
    // Specifies the time range during which the contained readings were taken.
    #[yaserde(rename = "timePeriod")]
    pub time_period: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ReadingSetBase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingSet {
    #[yaserde(rename = "ReadingListLink")]
    pub reading_list_link: Option<ReadingListLink>,

    // Specifies the time range during which the contained readings were taken.
    #[yaserde(rename = "timePeriod")]
    pub time_period: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ReadingSet {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingSetList {
    #[yaserde(rename = "ReadingSet")]
    pub reading_set: Vec<ReadingSet>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ReadingSetList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingType {
    // The “accumulation behaviour” indicates how the value is represented
    // to accumulate over time.
    #[yaserde(rename = "accumulationBehaviour")]
    pub accumulation_behaviour: Option<AccumulationBehaviourType>,

    // The amount of heat generated when a given mass of fuel is completely
    // burned. The CalorificValue is used to convert the measured volume or mass
    // of gas into kWh. The CalorificValue attribute represents the current
    // active value.
    #[yaserde(rename = "calorificValue")]
    pub calorific_value: Option<UnitValueType>,

    // Indicates the commodity applicable to this ReadingType.
    #[yaserde(rename = "commodity")]
    pub commodity: Option<CommodityType>,

    // Accounts for changes in the volume of gas based on temperature and
    // pressure. The ConversionFactor attribute represents the current active
    // value. The ConversionFactor is dimensionless. The default value for the
    // ConversionFactor is 1, which means no conversion is applied. A price
    // server can advertise a new/different value at any time.
    #[yaserde(rename = "conversionFactor")]
    pub conversion_factor: Option<UnitValueType>,

    // The data type can be used to describe a salient attribute of the data.
    // Possible values are average, absolute, and etc.
    #[yaserde(rename = "dataQualifier")]
    pub data_qualifier: Option<DataQualifierType>,

    // Anything involving current might have a flow direction. Possible values
    // include forward and reverse.
    #[yaserde(rename = "flowDirection")]
    pub flow_direction: Option<FlowDirectionType>,

    // Default interval length specified in seconds.
    #[yaserde(rename = "intervalLength")]
    pub interval_length: Option<Uint32>,

    // Compound class that contains kindCategory and kindIndex
    #[yaserde(rename = "kind")]
    pub kind: Option<KindType>,

    // To be populated for mirrors of interval data to set the expected number
    // of intervals per ReadingSet. Servers may discard intervals received that
    // exceed this number.
    #[yaserde(rename = "maxNumberOfIntervals")]
    pub max_number_of_intervals: Option<Uint8>,

    // Number of consumption blocks. 0 means not applicable, and is the default
    // if not specified. The value needs to be at least 1 if any actual prices
    // are provided.
    #[yaserde(rename = "numberOfConsumptionBlocks")]
    pub number_of_consumption_blocks: Option<Uint8>,

    // The number of TOU tiers that can be used by any resource configured by
    // this ReadingType. Servers SHALL populate this value with the largest
    // touTier value that will <i>ever</i> be used while this ReadingType is in
    // effect. Servers SHALL set numberOfTouTiers equal to the number of
    // standard TOU tiers plus the number of CPP tiers that may be used while
    // this ReadingType is in effect. Servers SHALL specify a value between 0
    // and 255 (inclusive) for numberOfTouTiers (servers providing flat rate
    // pricing SHOULD set numberOfTouTiers to 0, as in practice there is no
    // difference between having no tiers and having one tier).
    #[yaserde(rename = "numberOfTouTiers")]
    pub number_of_tou_tiers: Option<Uint8>,

    // Contains phase information associated with the type.
    #[yaserde(rename = "phase")]
    pub phase: Option<PhaseCode>,

    // Indicates the power of ten multiplier applicable to the unit of measure
    // of this ReadingType.
    #[yaserde(rename = "powerOfTenMultiplier")]
    pub power_of_ten_multiplier: Option<PowerOfTenMultiplierType>,

    // Default sub-interval length specified in seconds for Readings of
    // ReadingType. Some demand calculations are done over a number of smaller
    // intervals. For example, in a rolling demand calculation, the demand value
    // is defined as the rolling sum of smaller intervals over the
    // intervalLength. The subintervalLength is the length of the smaller
    // interval in this calculation. It SHALL be an integral division of the
    // intervalLength. The number of sub-intervals can be calculated by dividing
    // the intervalLength by the subintervalLength.
    #[yaserde(rename = "subIntervalLength")]
    pub sub_interval_length: Option<Uint32>,

    // Reflects the supply limit set in the meter. This value can be compared to
    // the Reading value to understand if limits are being approached or
    // exceeded. Units follow the same definition as in this ReadingType.
    #[yaserde(rename = "supplyLimit")]
    pub supply_limit: Option<Uint48>,

    // Specifies whether or not the consumption blocks are differentiated by
    // TOUTier or not. Default is false, if not specified.
    // true = consumption accumulated over individual tiers
    // false = consumption accumulated over all tiers
    #[yaserde(rename = "tieredConsumptionBlocks")]
    pub tiered_consumption_blocks: Option<bool>,

    // Indicates the measurement type for the units of measure for the readings
    // of this type.
    #[yaserde(rename = "uom")]
    pub uom: Option<UomType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ReadingType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UsagePointBase {
    // Specifies the roles that apply to the usage point.
    #[yaserde(rename = "roleFlags")]
    pub role_flags: RoleFlagsType,

    // The kind of service provided by this usage point.
    #[yaserde(rename = "serviceCategoryKind")]
    pub service_category_kind: ServiceKind,

    // Specifies the current status of the service at this usage point.
    // 0 = off
    // 1 = on
    #[yaserde(rename = "status")]
    pub status: Uint8,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for UsagePointBase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UsagePoint {
    // The LFDI of the source device. This attribute SHALL be present when
    // mirroring.
    #[yaserde(rename = "deviceLFDI")]
    pub device_lfdi: Option<HexBinary160>,

    #[yaserde(rename = "MeterReadingListLink")]
    pub meter_reading_list_link: Option<MeterReadingListLink>,

    // Specifies the roles that apply to the usage point.
    #[yaserde(rename = "roleFlags")]
    pub role_flags: RoleFlagsType,

    // The kind of service provided by this usage point.
    #[yaserde(rename = "serviceCategoryKind")]
    pub service_category_kind: ServiceKind,

    // Specifies the current status of the service at this usage point.
    // 0 = off
    // 1 = on
    #[yaserde(rename = "status")]
    pub status: Uint8,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for UsagePoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UsagePointList {
    #[yaserde(rename = "UsagePoint")]
    pub usage_point: Vec<UsagePoint>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for UsagePointList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ConsumptionTariffInterval {
    // Indicates the consumption block related to the reading. If not specified,
    // is assumed to be "0 - N/A".
    #[yaserde(rename = "consumptionBlock")]
    pub consumption_block: ConsumptionBlockType,

    #[yaserde(rename = "EnvironmentalCost")]
    pub environmental_cost: Vec<EnvironmentalCost>,

    // The charge for this rate component, per unit of measure defined by the
    // associated ReadingType, in currency specified in TariffProfile.
    // The Pricing service provider determines the appropriate price attribute
    // value based on its applicable regulatory rules. For example, price could
    // be net or inclusive of applicable taxes, fees, or levies.
    // The Billing function set provides the ability to represent billing
    // information in a more detailed manner.
    #[yaserde(rename = "price")]
    pub price: Option<Int32>,

    // The lowest level of consumption that defines the starting point of this
    // consumption step or block. Thresholds start at zero for each billing
    // period.
    // If specified, the first ConsumptionTariffInterval.startValue for a
    // TimeTariffInteral instance SHALL begin at "0." Subsequent
    // ConsumptionTariffInterval.startValue elements SHALL be greater than the
    // previous one.
    #[yaserde(rename = "startValue")]
    pub start_value: Uint48,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ConsumptionTariffInterval {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ConsumptionTariffIntervalList {
    #[yaserde(rename = "ConsumptionTariffInterval")]
    pub consumption_tariff_interval: Vec<ConsumptionTariffInterval>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ConsumptionTariffIntervalList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum CostKindType {
    #[default]
    // Carbon Dioxide emissions, in grams per unit
    CarbonDioxide = 0,
    // Sulfur Dioxide emissions, in grams per unit
    SulfurDioxide = 1,
    // Nitrogen Oxides emissions, in grams per unit
    NitrogenOxide = 2,
    // Renewable generation, as a percentage of overall generation
    RenewablePercentage = 3,
    // 4-255 RESERVED
}

impl Validate for CostKindType {}

// Provides alternative or secondary price information for the relevant
// RateComponent. Supports jurisdictions that seek to convey the environmental
// price per unit of the specified commodity not expressed in currency.
// Implementers and consumers can use this attribute to prioritize operations of
// their HAN devices (e.g., PEV charging during times of high availability of
// renewable electricity resources).
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EnvironmentalCost {
    // The estimated or actual environmental or other cost, per commodity unit
    // defined by the ReadingType, for this RateComponent (e.g., grams of carbon
    // dioxide emissions each per kWh).
    #[yaserde(rename = "amount")]
    pub amount: Uint32,

    // The kind of cost referred to in the amount.
    #[yaserde(rename = "costKind")]
    pub cost_kind: CostKindType,

    // The relative level of the amount attribute. In conjunction with
    // numCostLevels, this attribute informs a device of the relative scarcity
    // of the amount attribute (e.g., a high or low availability of renewable
    // generation).
    // numCostLevels and costLevel values SHALL ascend in order of scarcity,
    // where "0" signals the lowest relative cost and higher values signal
    // increasing cost. For example, if numCostLevels is equal to “3,” then
    // if the lowest relative costLevel were equal to “0,” devices would
    // assume this is the lowest relative period to operate. Likewise, if the
    // costLevel in the next TimeTariffInterval instance is equal to “1,”
    // then the device would assume it is relatively more expensive, in
    // environmental terms, to operate during this TimeTariffInterval instance
    // than the previous one.
    // There is no limit to the number of relative price levels other than that
    // indicated in the attribute type, but for practicality, service providers
    // should strive for simplicity and recognize the diminishing returns
    // derived from increasing the numCostLevel value greater than four.
    #[yaserde(rename = "costLevel")]
    pub cost_level: Uint8,

    // The number of all relative cost levels.
    // In conjunction with costLevel, numCostLevels signals the relative
    // scarcity of the commodity for the duration of the TimeTariffInterval
    // instance (e.g., a relative indication of cost). This is useful in
    // providing context for nominal cost signals to consumers or devices that
    // might see a range of amount values from different service providres or
    // from the same service provider.
    #[yaserde(rename = "numCostLevels")]
    pub num_cost_levels: Uint8,
}

impl Validate for EnvironmentalCost {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RateComponent {
    #[yaserde(rename = "ActiveTimeTariffIntervalListLink")]
    pub active_time_tariff_interval_list_link: Option<ActiveTimeTariffIntervalListLink>,

    // Specifies the maximum flow rate (e.g. kW for electricity) for which this
    // RateComponent applies, for the usage point and given rate / tariff.
    // In combination with flowRateStartLimit, allows a service provider to
    // define the demand or output characteristics for the particular tariff
    // design. If a server includes the flowRateEndLimit attribute, then it
    // SHALL also include flowRateStartLimit attribute.
    // For example, a service provider’s tariff limits customers to 20 kWs of
    // demand for the given rate structure. Above this threshold (from 20-50
    // kWs), there are different demand charges per unit of consumption. The
    // service provider can use flowRateStartLimit and flowRateEndLimit to
    // describe the demand characteristics of the different rates. Similarly,
    // these attributes can be used to describe limits on premises DERs that
    // might be producing a commodity and sending it back into the distribution
    // network.
    // Note: At the time of writing, service provider tariffs with demand-based
    // components were not originally identified as being in scope, and service
    // provider tariffs vary widely in their use of demand components and the
    // method for computing charges. It is expected that industry groups (e.g.,
    // OpenSG) will document requirements in the future that the IEEE 2030.5
    // community can then use as source material for the next version of IEEE
    // 2030.5.
    #[yaserde(rename = "flowRateEndLimit")]
    pub flow_rate_end_limit: Option<UnitValueType>,

    // Specifies the minimum flow rate (e.g., kW for electricity) for which this
    // RateComponent applies, for the usage point and given rate / tariff.
    // In combination with flowRateEndLimit, allows a service provider to define
    // the demand or output characteristics for the particular tariff design. If
    // a server includes the flowRateStartLimit attribute, then it SHALL also
    // include flowRateEndLimit attribute.
    #[yaserde(rename = "flowRateStartLimit")]
    pub flow_rate_start_limit: Option<UnitValueType>,

    // Provides indication of the ReadingType with which this price is
    // associated.
    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: ReadingTypeLink,

    // Specifies the roles that this usage point has been assigned.
    #[yaserde(rename = "roleFlags")]
    pub role_flags: RoleFlagsType,

    #[yaserde(rename = "TimeTariffIntervalListLink")]
    pub time_tariff_interval_list_link: TimeTariffIntervalListLink,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RateComponent {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RateComponentList {
    #[yaserde(rename = "RateComponent")]
    pub rate_component: Vec<RateComponent>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for RateComponentList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TariffProfile {
    // The currency code indicating the currency for this TariffProfile.
    #[yaserde(rename = "currency")]
    pub currency: Option<CurrencyCode>,

    // Indicates the power of ten multiplier for the price attribute.
    #[yaserde(rename = "pricePowerOfTenMultiplier")]
    pub price_power_of_ten_multiplier: Option<PowerOfTenMultiplierType>,

    // Indicates the relative primacy of the provider of this program.
    #[yaserde(rename = "primacy")]
    pub primacy: PrimacyType,

    // The rate code for this tariff profile. Provided by the Pricing service
    // provider per its internal business needs and practices and provides a
    // method to identify the specific rate code for the TariffProfile instance.
    // This would typically not be communicated to the user except to facilitate
    // troubleshooting due to its service provider-specific technical nature.
    #[yaserde(rename = "rateCode")]
    pub rate_code: Option<String20>,

    #[yaserde(rename = "RateComponentListLink")]
    pub rate_component_list_link: Option<RateComponentListLink>,

    // The kind of service provided by this usage point.
    #[yaserde(rename = "serviceCategoryKind")]
    pub service_category_kind: ServiceKind,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for TariffProfile {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TariffProfileList {
    #[yaserde(rename = "TariffProfile")]
    pub tariff_profile: Vec<TariffProfile>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for TariffProfileList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TimeTariffInterval {
    #[yaserde(rename = "ConsumptionTariffIntervalListLink")]
    pub consumption_tariff_interval_list_link: Option<ConsumptionTariffIntervalListLink>,

    // Indicates the time of use tier related to the reading. If not specified,
    // is assumed to be "0 - N/A".
    #[yaserde(rename = "touTier")]
    pub tou_tier: Toutype,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval duration, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeDuration")]
    pub randomize_duration: Option<OneHourRangeType>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval start time, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeStart")]
    pub randomize_start: Option<OneHourRangeType>,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for TimeTariffInterval {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TimeTariffIntervalList {
    #[yaserde(rename = "TimeTariffInterval")]
    pub time_tariff_interval: Vec<TimeTariffInterval>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for TimeTariffIntervalList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MessagingProgram {
    #[yaserde(rename = "ActiveTextMessageListLink")]
    pub active_text_message_list_link: Option<ActiveTextMessageListLink>,

    // Indicates the language and region of the messages in this collection.
    #[yaserde(rename = "locale")]
    pub locale: LocaleType,

    // Indicates the relative primacy of the provider of this program.
    #[yaserde(rename = "primacy")]
    pub primacy: PrimacyType,

    #[yaserde(rename = "TextMessageListLink")]
    pub text_message_list_link: Option<TextMessageListLink>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MessagingProgram {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MessagingProgramList {
    #[yaserde(rename = "MessagingProgram")]
    pub messaging_program: Vec<MessagingProgram>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MessagingProgramList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum PriorityType {
    #[default]
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
    // 4-255 RESERVED
}

impl Validate for PriorityType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TextMessage {
    // Indicates the human-readable name of the publisher of the message
    #[yaserde(rename = "originator")]
    pub originator: Option<String20>,

    // The priority is used to inform the client of the priority of the
    // particular message. Devices with constrained or limited resources for
    // displaying Messages should use this attribute to determine how to handle
    // displaying currently active Messages (e.g. if a device uses a scrolling
    // method with a single Message viewable at a time it MAY want to push a low
    // priority Message to the background and bring a newly received higher
    // priority Message to the foreground).
    #[yaserde(rename = "priority")]
    pub priority: PriorityType,

    // The textMessage attribute contains the actual UTF-8 encoded text to be
    // displayed in conjunction with the messageLength attribute which contains
    // the overall length of the textMessage attribute. Clients and servers
    // SHALL support a reception of a Message of 100 bytes in length. Messages
    // that exceed the clients display size will be left to the client to choose
    // what method to handle the message (truncation, scrolling, etc.).
    #[yaserde(rename = "textMessage")]
    pub text_message: String,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for TextMessage {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TextMessageList {
    #[yaserde(rename = "TextMessage")]
    pub text_message: Vec<TextMessage>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for TextMessageList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingPeriod {
    // The amount of the bill for the previous billing period.
    #[yaserde(rename = "billLastPeriod")]
    pub bill_last_period: Option<Int48>,

    // The bill amount related to the billing period as of the statusTimeStamp.
    #[yaserde(rename = "billToDate")]
    pub bill_to_date: Option<Int48>,

    // The time interval for this billing period.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The date / time of the last update of this resource.
    #[yaserde(rename = "statusTimeStamp")]
    pub status_time_stamp: Option<TimeType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for BillingPeriod {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingPeriodList {
    #[yaserde(rename = "BillingPeriod")]
    pub billing_period: Vec<BillingPeriod>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for BillingPeriodList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingMeterReadingBase {
    #[yaserde(rename = "BillingReadingSetListLink")]
    pub billing_reading_set_list_link: Option<BillingReadingSetListLink>,

    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: Option<ReadingTypeLink>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for BillingMeterReadingBase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReading {
    #[yaserde(rename = "Charge")]
    pub charge: Vec<Charge>,

    // Indicates the consumption block related to the reading. REQUIRED if
    // ReadingType numberOfConsumptionBlocks is non-zero. If not specified, is
    // assumed to be "0 - N/A".
    #[yaserde(rename = "consumptionBlock")]
    pub consumption_block: Option<ConsumptionBlockType>,

    // List of codes indicating the quality of the reading, using specification:
    // Bit 0 - valid: data that has gone through all required validation checks
    // and either passed them all or has been verified
    // Bit 1 - manually edited: Replaced or approved by a human
    // Bit 2 - estimated using reference day: data value was replaced by a
    // machine computed value based on analysis of historical data using the
    // same type of measurement.
    // Bit 3 - estimated using linear interpolation: data value was computed
    // using linear interpolation based on the readings before and after it
    // Bit 4 - questionable: data that has failed one or more checks
    // Bit 5 - derived: data that has been calculated (using logic or
    // mathematical operations), not necessarily measured directly
    // Bit 6 - projected (forecast): data that has been calculated as a
    // projection or forecast of future readings
    #[yaserde(rename = "qualityFlags")]
    pub quality_flags: Option<HexBinary16>,

    // The time interval associated with the reading. If not specified, then
    // defaults to the intervalLength specified in the associated ReadingType.
    #[yaserde(rename = "timePeriod")]
    pub time_period: Option<DateTimeInterval>,

    // Indicates the time of use tier related to the reading. REQUIRED if
    // ReadingType numberOfTouTiers is non-zero. If not specified, is assumed to
    // be "0 - N/A".
    #[yaserde(rename = "touTier")]
    pub tou_tier: Option<Toutype>,

    // Value in units specified by ReadingType
    #[yaserde(rename = "value")]
    pub value: Option<Int48>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for BillingReading {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReadingList {
    #[yaserde(rename = "BillingReading")]
    pub billing_reading: Vec<BillingReading>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for BillingReadingList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReadingSet {
    #[yaserde(rename = "BillingReadingListLink")]
    pub billing_reading_list_link: Option<BillingReadingListLink>,

    // Specifies the time range during which the contained readings were taken.
    #[yaserde(rename = "timePeriod")]
    pub time_period: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for BillingReadingSet {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReadingSetList {
    #[yaserde(rename = "BillingReadingSet")]
    pub billing_reading_set: Vec<BillingReadingSet>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for BillingReadingSetList {}

// Charges contain charges on a customer bill. These could be items like taxes,
// levies, surcharges, rebates, or others. This is meant to allow the HAN device
// to retrieve enough information to be able to reconstruct an estimate of what
// the total bill would look like.
// Providers can provide line item billing, including multiple charge kinds
// (e.g. taxes, surcharges) at whatever granularity desired, using as many
// Charges as desired during a billing period. There can also be any number of
// Charges associated with different ReadingTypes to distinguish between TOU
// tiers, consumption blocks, or demand charges.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Charge {
    // A description of the charge.
    #[yaserde(rename = "description")]
    pub description: Option<String20>,

    // The type (kind) of charge.
    #[yaserde(rename = "kind")]
    pub kind: Option<ChargeKind>,

    // A monetary charge.
    #[yaserde(rename = "value")]
    pub value: Int32,
}

impl Validate for Charge {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum ChargeKind {
    #[default]
    ConsumptionCharge = 0,
    Rebate = 1,
    AuxiliaryCharge = 2,
    DemandCharge = 3,
    TaxCharge = 4,
    // 5-255 NOT RESERVED
}

impl Validate for ChargeKind {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAccount {
    // The ISO 4217 code indicating the currency applicable to the bill amounts
    // in the summary. See list at
    // http://www.unece.org/cefact/recommendations/rec09/rec09_ecetrd203.pdf
    #[yaserde(rename = "currency")]
    pub currency: Uint16,

    // The account number for the customer (if applicable).
    #[yaserde(rename = "customerAccount")]
    pub customer_account: Option<String42>,

    #[yaserde(rename = "CustomerAgreementListLink")]
    pub customer_agreement_list_link: Option<CustomerAgreementListLink>,

    // The name of the customer.
    #[yaserde(rename = "customerName")]
    pub customer_name: Option<String42>,

    // Indicates the power of ten multiplier for the prices in this function
    // set.
    #[yaserde(rename = "pricePowerOfTenMultiplier")]
    pub price_power_of_ten_multiplier: PowerOfTenMultiplierType,

    #[yaserde(rename = "ServiceSupplierLink")]
    pub service_supplier_link: Option<ServiceSupplierLink>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for CustomerAccount {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAccountList {
    #[yaserde(rename = "CustomerAccount")]
    pub customer_account: Vec<CustomerAccount>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for CustomerAccountList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAgreement {
    #[yaserde(rename = "ActiveBillingPeriodListLink")]
    pub active_billing_period_list_link: Option<ActiveBillingPeriodListLink>,

    #[yaserde(rename = "ActiveProjectionReadingListLink")]
    pub active_projection_reading_list_link: Option<ActiveProjectionReadingListLink>,

    #[yaserde(rename = "ActiveTargetReadingListLink")]
    pub active_target_reading_list_link: Option<ActiveTargetReadingListLink>,

    #[yaserde(rename = "BillingPeriodListLink")]
    pub billing_period_list_link: Option<BillingPeriodListLink>,

    #[yaserde(rename = "HistoricalReadingListLink")]
    pub historical_reading_list_link: Option<HistoricalReadingListLink>,

    #[yaserde(rename = "PrepaymentLink")]
    pub prepayment_link: Option<PrepaymentLink>,

    #[yaserde(rename = "ProjectionReadingListLink")]
    pub projection_reading_list_link: Option<ProjectionReadingListLink>,

    // The account number of the service account (if applicable).
    #[yaserde(rename = "serviceAccount")]
    pub service_account: Option<String42>,

    // The address or textual description of the service location.
    #[yaserde(rename = "serviceLocation")]
    pub service_location: Option<String42>,

    #[yaserde(rename = "TargetReadingListLink")]
    pub target_reading_list_link: Option<TargetReadingListLink>,

    #[yaserde(rename = "TariffProfileLink")]
    pub tariff_profile_link: Option<TariffProfileLink>,

    #[yaserde(rename = "UsagePointLink")]
    pub usage_point_link: Option<UsagePointLink>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for CustomerAgreement {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAgreementList {
    #[yaserde(rename = "CustomerAgreement")]
    pub customer_agreement: Vec<CustomerAgreement>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for CustomerAgreementList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct HistoricalReading {
    #[yaserde(rename = "BillingReadingSetListLink")]
    pub billing_reading_set_list_link: Option<BillingReadingSetListLink>,

    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: Option<ReadingTypeLink>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for HistoricalReading {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct HistoricalReadingList {
    #[yaserde(rename = "HistoricalReading")]
    pub historical_reading: Vec<HistoricalReading>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for HistoricalReadingList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ProjectionReading {
    #[yaserde(rename = "BillingReadingSetListLink")]
    pub billing_reading_set_list_link: Option<BillingReadingSetListLink>,

    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: Option<ReadingTypeLink>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ProjectionReading {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ProjectionReadingList {
    #[yaserde(rename = "ProjectionReading")]
    pub projection_reading: Vec<ProjectionReading>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ProjectionReadingList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TargetReading {
    #[yaserde(rename = "BillingReadingSetListLink")]
    pub billing_reading_set_list_link: Option<BillingReadingSetListLink>,

    #[yaserde(rename = "ReadingTypeLink")]
    pub reading_type_link: Option<ReadingTypeLink>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for TargetReading {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TargetReadingList {
    #[yaserde(rename = "TargetReading")]
    pub target_reading: Vec<TargetReading>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for TargetReadingList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ServiceSupplier {
    // E-mail address for this service supplier.
    #[yaserde(rename = "email")]
    pub email: Option<String32>,

    // Human-readable phone number for this service supplier.
    #[yaserde(rename = "phone")]
    pub phone: Option<String20>,

    // Contains the IANA PEN for the commodity provider.
    #[yaserde(rename = "providerID")]
    pub provider_id: Option<Uint32>,

    // Website URI address for this service supplier.
    #[yaserde(rename = "web")]
    pub web: Option<String42>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ServiceSupplier {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ServiceSupplierList {
    #[yaserde(rename = "ServiceSupplier")]
    pub service_supplier: Vec<ServiceSupplier>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for ServiceSupplierList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AccountBalance {
    // AvailableCredit shows the balance of the sum of credits minus the sum of
    // charges. In a Central Wallet mode this value may be passed down to the
    // Prepayment server via an out-of-band mechanism. In Local or ESI modes,
    // this value may be calculated based upon summation of CreditRegister
    // transactions minus consumption charges calculated using Metering (and
    // possibly Pricing) function set data. This value may be negative; for
    // instance, if disconnection is prevented due to a Supply Interruption
    // Override.
    #[yaserde(rename = "availableCredit")]
    pub available_credit: AccountingUnit,

    // CreditStatus identifies whether the present value of availableCredit is
    // considered OK, low, exhausted, or negative.
    #[yaserde(rename = "creditStatus")]
    pub credit_status: Option<CreditStatusType>,

    // EmergencyCredit is the amount of credit still available for the given
    // service or commodity prepayment instance. If both availableCredit and
    // emergyCredit are exhausted, then service will typically be disconnected.
    #[yaserde(rename = "emergencyCredit")]
    pub emergency_credit: Option<AccountingUnit>,

    // EmergencyCreditStatus identifies whether the present value of
    // emergencyCredit is considered OK, low, exhausted, or negative.
    #[yaserde(rename = "emergencyCreditStatus")]
    pub emergency_credit_status: Option<CreditStatusType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for AccountBalance {}

// Unit for accounting; use either 'energyUnit' or 'currencyUnit' to specify the
// unit for 'value'.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AccountingUnit {
    // Unit of service.
    #[yaserde(rename = "energyUnit")]
    pub energy_unit: Option<RealEnergy>,

    // Unit of currency.
    #[yaserde(rename = "monetaryUnit")]
    pub monetary_unit: CurrencyCode,

    // Multiplier for the 'energyUnit' or 'monetaryUnit'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value of the monetary aspect
    #[yaserde(rename = "value")]
    pub value: Int32,
}

impl Validate for AccountingUnit {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CreditRegister {
    // CreditAmount is the amount of credit being added by a particular
    // CreditRegister transaction. Negative values indicate that credit is being
    // subtracted.
    #[yaserde(rename = "creditAmount")]
    pub credit_amount: AccountingUnit,

    // CreditType indicates whether the credit transaction applies to regular or
    // emergency credit.
    #[yaserde(rename = "creditType")]
    pub credit_type: Option<CreditTypeType>,

    // EffectiveTime identifies the time at which the credit transaction goes
    // into effect. For credit addition transactions, this is typically the
    // moment at which the transaction takes place. For credit subtraction
    // transactions, (e.g., non-fuel debt recovery transactions initiated from a
    // back-haul or ESI) this may be a future time at which credit is deducted.
    #[yaserde(rename = "effectiveTime")]
    pub effective_time: TimeType,

    // Token is security data that authenticates the legitimacy of the
    // transaction. The details of this token are not defined by IEEE 2030.5.
    // How a Prepayment server handles this field is left as vendor specific
    // implementation or will be defined by one or more other standards.
    #[yaserde(rename = "token")]
    pub token: String32,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for CreditRegister {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CreditRegisterList {
    #[yaserde(rename = "CreditRegister")]
    pub credit_register: Vec<CreditRegister>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for CreditRegisterList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Prepayment {
    #[yaserde(rename = "AccountBalanceLink")]
    pub account_balance_link: AccountBalanceLink,

    #[yaserde(rename = "ActiveCreditRegisterListLink")]
    pub active_credit_register_list_link: Option<ActiveCreditRegisterListLink>,

    #[yaserde(rename = "ActiveSupplyInterruptionOverrideListLink")]
    pub active_supply_interruption_override_list_link:
        Option<ActiveSupplyInterruptionOverrideListLink>,

    // CreditExpiryLevel is the set point for availableCredit at which the
    // service level may be changed. The typical value for this attribute is 0,
    // regardless of whether the account balance is measured in a monetary or
    // commodity basis. The units for this attribute SHALL match the units used
    // for availableCredit.
    #[yaserde(rename = "creditExpiryLevel")]
    pub credit_expiry_level: Option<AccountingUnit>,

    #[yaserde(rename = "CreditRegisterListLink")]
    pub credit_register_list_link: CreditRegisterListLink,

    // LowCreditWarningLevel is the set point for availableCredit at which the
    // creditStatus attribute in the AccountBalance resource SHALL indicate that
    // available credit is low. The units for this attribute SHALL match the
    // units used for availableCredit. Typically, this value is set by the
    // service provider.
    #[yaserde(rename = "lowCreditWarningLevel")]
    pub low_credit_warning_level: Option<AccountingUnit>,

    // LowEmergencyCreditWarningLevel is the set point for emergencyCredit at
    // which the creditStatus attribute in the AccountBalance resource SHALL
    // indicate that emergencycredit is low. The units for this attribute SHALL
    // match the units used for availableCredit. Typically, this value is set by
    // the service provider.
    #[yaserde(rename = "lowEmergencyCreditWarningLevel")]
    pub low_emergency_credit_warning_level: Option<AccountingUnit>,

    // PrepayMode specifies whether the given Prepayment instance is operating
    // in Credit, Central Wallet, ESI, or Local prepayment mode. The Credit mode
    // indicates that prepayment is not presently in effect. The other modes are
    // described in the Overview Section above.
    #[yaserde(rename = "prepayMode")]
    pub prepay_mode: PrepayModeType,

    #[yaserde(rename = "PrepayOperationStatusLink")]
    pub prepay_operation_status_link: PrepayOperationStatusLink,

    #[yaserde(rename = "SupplyInterruptionOverrideListLink")]
    pub supply_interruption_override_list_link: SupplyInterruptionOverrideListLink,

    #[yaserde(rename = "UsagePoint")]
    pub usage_point: Vec<UsagePoint>,

    #[yaserde(rename = "UsagePointLink")]
    pub usage_point_link: Option<UsagePointLink>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Prepayment {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PrepaymentList {
    #[yaserde(rename = "Prepayment")]
    pub prepayment: Vec<Prepayment>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for PrepaymentList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum PrepayModeType {
    #[default]
    CentralWallet = 0,
    ESI = 1,
    Local = 2,
    Credit = 3,
    // 4-255 RESERVED
}

impl Validate for PrepayModeType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PrepayOperationStatus {
    // CreditTypeChange is used to define a pending change of creditTypeInUse,
    // which will activate at a specified time.
    #[yaserde(rename = "creditTypeChange")]
    pub credit_type_change: Option<CreditTypeChange>,

    // CreditTypeInUse identifies whether the present mode of operation is
    // consuming regular credit or emergency credit.
    #[yaserde(rename = "creditTypeInUse")]
    pub credit_type_in_use: Option<CreditTypeType>,

    // ServiceChange is used to define a pending change of serviceStatus, which
    // will activate at a specified time.
    #[yaserde(rename = "serviceChange")]
    pub service_change: Option<ServiceChange>,

    // ServiceStatus identifies whether the service is connected or
    // disconnected, or armed for connection or disconnection.
    #[yaserde(rename = "serviceStatus")]
    pub service_status: ServiceStatusType,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for PrepayOperationStatus {}

// Specifies a change to the service status.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ServiceChange {
    // The new service status, to take effect at the time specified by startTime
    #[yaserde(rename = "newStatus")]
    pub new_status: ServiceStatusType,

    // The date/time when the change is to take effect.
    #[yaserde(rename = "startTime")]
    pub start_time: TimeType,
}

impl Validate for ServiceChange {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupplyInterruptionOverride {
    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Interval defines the period of time during which supply should not be
    // interrupted.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for SupplyInterruptionOverride {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupplyInterruptionOverrideList {
    #[yaserde(rename = "SupplyInterruptionOverride")]
    pub supply_interruption_override: Vec<SupplyInterruptionOverride>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for SupplyInterruptionOverrideList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum CreditStatusType {
    #[default]
    Ok = 0,
    Low = 1,
    Exhausted = 2,
    Negative = 3,
    // 4-255 RESERVED
}

impl Validate for CreditStatusType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum CreditTypeType {
    #[default]
    Regular = 0,
    Emergency = 1,
    RegularThenEmergency = 2,
    EmergencyThenRegular = 3,
    // 4-255 RESERVED
}

impl Validate for CreditTypeType {}

// Specifies a change to the credit type.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CreditTypeChange {
    // The new credit type, to take effect at the time specified by startTime
    #[yaserde(rename = "newType")]
    pub new_type: CreditTypeType,

    // The date/time when the change is to take effect.
    #[yaserde(rename = "startTime")]
    pub start_time: TimeType,
}

impl Validate for CreditTypeChange {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum ServiceStatusType {
    #[default]
    Connected = 0,
    Disconnected = 1,
    ArmedForConnect = 2,
    ArmedForDisconnect = 3,
    NoContactor = 4,
    LoadLimited = 5,
    // 6-255 RESERVED
}

impl Validate for ServiceStatusType {}

// The RequestStatus object is used to indicate the current status of a Flow
// Reservation Request.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RequestStatus {
    // The dateTime attribute will provide a timestamp of when the request
    // status was set. dateTime MUST be set to the time at which the status
    // change occurred, not a time in the future or past.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    // Field representing the request status type.
    // 0 = Requested
    // 1 = Cancelled
    // All other values reserved.
    #[yaserde(rename = "requestStatus")]
    pub request_status: Uint8,
}

impl Validate for RequestStatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationRequest {
    // The time at which the request was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    // A value that is calculated by the storage device that defines the minimum
    // duration, in seconds, that it will take to complete the actual flow
    // transaction, including any ramp times and conditioning times, if
    // applicable.
    #[yaserde(rename = "durationRequested")]
    pub duration_requested: Option<Uint16>,

    // Indicates the total amount of energy, in Watt-Hours, requested to be
    // transferred between the storage device and the electric power system.
    // Positive values indicate charging and negative values indicate
    // discharging. This sign convention is different than for the DER function
    // where discharging is positive. Note that the energyRequestNow attribute
    // in the PowerStatus Object must always represent a charging solution and
    // it is not allowed to have a negative value.
    #[yaserde(rename = "energyRequested")]
    pub energy_requested: SignedRealEnergy,

    // The time window during which the flow reservation is needed. For example,
    // if an electric vehicle is set with a 7:00 AM time charge is needed, and
    // price drops to the lowest tier at 11:00 PM, then this window would likely
    // be from 11:00 PM until 7:00 AM.
    #[yaserde(rename = "intervalRequested")]
    pub interval_requested: DateTimeInterval,

    // Indicates the sustained level of power, in Watts, that is requested. For
    // charging this is calculated by the storage device and it represents the
    // charging system capability (which for an electric vehicle must also
    // account for any power limitations due to the EVSE control pilot). For
    // discharging, a lower value than the inverter capability can be used as a
    // target.
    #[yaserde(rename = "powerRequested")]
    pub power_requested: ActivePower,

    #[yaserde(rename = "RequestStatus")]
    pub request_status: RequestStatus,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FlowReservationRequest {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationRequestList {
    #[yaserde(rename = "FlowReservationRequest")]
    pub flow_reservation_request: Vec<FlowReservationRequest>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FlowReservationRequestList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationResponse {
    // Indicates the amount of energy available.
    #[yaserde(rename = "energyAvailable")]
    pub energy_available: SignedRealEnergy,

    // Indicates the amount of power available.
    #[yaserde(rename = "powerAvailable")]
    pub power_available: ActivePower,

    // The subject field provides a method to match the response with the
    // originating event. It is populated with the mRID of the corresponding
    // FlowReservationRequest object.
    #[yaserde(rename = "subject")]
    pub subject: Mridtype,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FlowReservationResponse {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationResponseList {
    #[yaserde(rename = "FlowReservationResponse")]
    pub flow_reservation_response: Vec<FlowReservationResponse>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for FlowReservationResponseList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DefaultDERControl {
    #[yaserde(rename = "DERControlBase")]
    pub der_control_base: DercontrolBase,

    // Enter service delay, in hundredths of a second. When present, this value
    // SHALL update the value of the corresponding setting
    // (DERSettings::setESDelay).
    #[yaserde(rename = "setESDelay")]
    pub set_es_delay: Option<Uint32>,

    // Enter service frequency high. Specified in hundredths of Hz. When
    // present, this value SHALL update the value of the corresponding setting
    // (DERSettings::setESHighFreq).
    #[yaserde(rename = "setESHighFreq")]
    pub set_es_high_freq: Option<Uint16>,

    // Enter service voltage high. Specified as an effective percent voltage,
    // defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in
    // hundredths of a percent. When present, this value SHALL update the value
    // of the corresponding setting (DERSettings::setESHighVolt).
    #[yaserde(rename = "setESHighVolt")]
    pub set_es_high_volt: Option<Int16>,

    // Enter service frequency low. Specified in hundredths of Hz. When present,
    // this value SHALL update the value of the corresponding setting
    // (DERSettings::setESLowFreq).
    #[yaserde(rename = "setESLowFreq")]
    pub set_es_low_freq: Option<Uint16>,

    // Enter service voltage low. Specified as an effective percent voltage,
    // defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in
    // hundredths of a percent. When present, this value SHALL update the value
    // of the corresponding setting (DERSettings::setESLowVolt).
    #[yaserde(rename = "setESLowVolt")]
    pub set_es_low_volt: Option<Int16>,

    // Enter service ramp time, in hundredths of a second. When present, this
    // value SHALL update the value of the corresponding setting
    // (DERSettings::setESRampTms).
    #[yaserde(rename = "setESRampTms")]
    pub set_es_ramp_tms: Option<Uint32>,

    // Enter service randomized delay, in hundredths of a second. When present,
    // this value SHALL update the value of the corresponding setting
    // (DERSettings::setESRandomDelay).
    #[yaserde(rename = "setESRandomDelay")]
    pub set_es_random_delay: Option<Uint32>,

    // Set default rate of change (ramp rate) of active power output due to
    // command or internal action, defined in %setWMax / second. Resolution is
    // in hundredths of a percent/second. A value of 0 means there is no limit.
    // Interpreted as a percentage change in output capability limit per second
    // when used as a default ramp rate. When present, this value SHALL update
    // the value of the corresponding setting (DERSettings::setGradW).
    #[yaserde(rename = "setGradW")]
    pub set_grad_w: Option<Uint16>,

    // Set soft-start rate of change (soft-start ramp rate) of active power
    // output due to command or internal action, defined in %setWMax / second.
    // Resolution is in hundredths of a percent/second. A value of 0 means there
    // is no limit. Interpreted as a percentage change in output capability
    // limit per second when used as a ramp rate. When present, this value SHALL
    // update the value of the corresponding setting
    // (DERSettings::setSoftGradW).
    #[yaserde(rename = "setSoftGradW")]
    pub set_soft_grad_w: Option<Uint16>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DefaultDERControl {}

// Type for Frequency-Droop (Frequency-Watt) operation.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FreqDroopType {
    // Frequency droop dead band for over-frequency conditions. In thousandths
    // of Hz.
    #[yaserde(rename = "dBOF")]
    pub d_bof: Uint32,

    // Frequency droop dead band for under-frequency conditions. In thousandths
    // of Hz.
    #[yaserde(rename = "dBUF")]
    pub d_buf: Uint32,

    // Frequency droop per-unit frequency change for over-frequency conditions
    // corresponding to 1 per-unit power output change. In thousandths,
    // unitless.
    #[yaserde(rename = "kOF")]
    pub k_of: Uint16,

    // Frequency droop per-unit frequency change for under-frequency conditions
    // corresponding to 1 per-unit power output change. In thousandths,
    // unitless.
    #[yaserde(rename = "kUF")]
    pub k_uf: Uint16,

    // Open loop response time, the duration from a step change in control
    // signal input until the output changes by 90% of its final change before
    // any overshoot, in hundredths of a second. Resolution is 1/100 sec. A
    // value of 0 is used to mean no limit.
    #[yaserde(rename = "openLoopTms")]
    pub open_loop_tms: Uint16,
}

impl Validate for FreqDroopType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Der {
    #[yaserde(rename = "AssociatedDERProgramListLink")]
    pub associated_der_program_list_link: Option<AssociatedDERProgramListLink>,

    #[yaserde(rename = "AssociatedUsagePointLink")]
    pub associated_usage_point_link: Option<AssociatedUsagePointLink>,

    #[yaserde(rename = "CurrentDERProgramLink")]
    pub current_der_program_link: Option<CurrentDERProgramLink>,

    #[yaserde(rename = "DERAvailabilityLink")]
    pub der_availability_link: Option<DeravailabilityLink>,

    #[yaserde(rename = "DERCapabilityLink")]
    pub der_capability_link: Option<DercapabilityLink>,

    #[yaserde(rename = "DERSettingsLink")]
    pub der_settings_link: Option<DersettingsLink>,

    #[yaserde(rename = "DERStatusLink")]
    pub der_status_link: Option<DerstatusLink>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Der {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Derlist {
    #[yaserde(rename = "DER")]
    pub der: Vec<Der>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Derlist {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Dersettings {
    // Bitmap indicating the DER Controls enabled on the device. See
    // DERControlType for values. If a control is supported (see
    // DERCapability::modesSupported), but not enabled, the control will not be
    // executed if encountered.
    #[yaserde(rename = "modesEnabled")]
    pub modes_enabled: Option<DercontrolType>,

    // Enter service delay, in hundredths of a second.
    #[yaserde(rename = "setESDelay")]
    pub set_es_delay: Option<Uint32>,

    // Enter service frequency high. Specified in hundredths of Hz.
    #[yaserde(rename = "setESHighFreq")]
    pub set_es_high_freq: Option<Uint16>,

    // Enter service voltage high. Specified as an effective percent voltage,
    // defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in
    // hundredths of a percent.
    #[yaserde(rename = "setESHighVolt")]
    pub set_es_high_volt: Option<Int16>,

    // Enter service frequency low. Specified in hundredths of Hz.
    #[yaserde(rename = "setESLowFreq")]
    pub set_es_low_freq: Option<Uint16>,

    // Enter service voltage low. Specified as an effective percent voltage,
    // defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in
    // hundredths of a percent.
    #[yaserde(rename = "setESLowVolt")]
    pub set_es_low_volt: Option<Int16>,

    // Enter service ramp time, in hundredths of a second.
    #[yaserde(rename = "setESRampTms")]
    pub set_es_ramp_tms: Option<Uint32>,

    // Enter service randomized delay, in hundredths of a second.
    #[yaserde(rename = "setESRandomDelay")]
    pub set_es_random_delay: Option<Uint32>,

    // Set default rate of change (ramp rate) of active power output due to
    // command or internal action, defined in %setWMax / second. Resolution is
    // in hundredths of a percent/second. A value of 0 means there is no limit.
    // Interpreted as a percentage change in output capability limit per second
    // when used as a default ramp rate.
    #[yaserde(rename = "setGradW")]
    pub set_grad_w: Uint16,

    // AC current maximum. Maximum AC current in RMS Amperes.
    #[yaserde(rename = "setMaxA")]
    pub set_max_a: Option<CurrentRMS>,

    // Maximum usable energy storage capacity of the DER, in AmpHours. Note:
    // this may be different from physical capability.
    #[yaserde(rename = "setMaxAh")]
    pub set_max_ah: Option<AmpereHour>,

    // Apparent power charge maximum. Maximum apparent power the DER can absorb
    // from the grid in Volt-Amperes. May differ from the apparent power maximum
    // (setMaxVA).
    #[yaserde(rename = "setMaxChargeRateVA")]
    pub set_max_charge_rate_va: Option<ApparentPower>,

    // Maximum rate of energy transfer received by the storage device, in Watts.
    // Defaults to rtgMaxChargeRateW.
    #[yaserde(rename = "setMaxChargeRateW")]
    pub set_max_charge_rate_w: Option<ActivePower>,

    // Apparent power discharge maximum. Maximum apparent power the DER can
    // deliver to the grid in Volt-Amperes. May differ from the apparent power
    // maximum (setMaxVA).
    #[yaserde(rename = "setMaxDischargeRateVA")]
    pub set_max_discharge_rate_va: Option<ApparentPower>,

    // Maximum rate of energy transfer delivered by the storage device, in
    // Watts. Defaults to rtgMaxDischargeRateW.
    #[yaserde(rename = "setMaxDischargeRateW")]
    pub set_max_discharge_rate_w: Option<ActivePower>,

    // AC voltage maximum setting.
    #[yaserde(rename = "setMaxV")]
    pub set_max_v: Option<VoltageRMS>,

    // Set limit for maximum apparent power capability of the DER (in VA).
    // Defaults to rtgMaxVA.
    #[yaserde(rename = "setMaxVA")]
    pub set_max_va: Option<ApparentPower>,

    // Set limit for maximum reactive power delivered by the DER (in var). SHALL
    // be a positive value &lt;= rtgMaxVar (default).
    #[yaserde(rename = "setMaxVar")]
    pub set_max_var: Option<ReactivePower>,

    // Set limit for maximum reactive power received by the DER (in var). If
    // present, SHALL be a negative value &gt;= rtgMaxVarNeg (default). If
    // absent, defaults to negative setMaxVar.
    #[yaserde(rename = "setMaxVarNeg")]
    pub set_max_var_neg: Option<ReactivePower>,

    // Set limit for maximum active power capability of the DER (in W). Defaults
    // to rtgMaxW.
    #[yaserde(rename = "setMaxW")]
    pub set_max_w: ActivePower,

    // Maximum energy storage capacity of the DER, in WattHours. Note: this may
    // be different from physical capability.
    #[yaserde(rename = "setMaxWh")]
    pub set_max_wh: Option<WattHour>,

    // Set minimum Power Factor displacement limit of the DER when injecting
    // reactive power (over-excited); SHALL be a positive value between 0.0
    // (typically &gt; 0.7) and 1.0. SHALL be &gt;= rtgMinPFOverExcited
    // (default).
    #[yaserde(rename = "setMinPFOverExcited")]
    pub set_min_pf_over_excited: Option<PowerFactor>,

    // Set minimum Power Factor displacement limit of the DER when absorbing
    // reactive power (under-excited); SHALL be a positive value between 0.0
    // (typically &gt; 0.7) and 0.9999. If present, SHALL be &gt;=
    // rtgMinPFUnderExcited (default). If absent, defaults to
    // setMinPFOverExcited.
    #[yaserde(rename = "setMinPFUnderExcited")]
    pub set_min_pf_under_excited: Option<PowerFactor>,

    // AC voltage minimum setting.
    #[yaserde(rename = "setMinV")]
    pub set_min_v: Option<VoltageRMS>,

    // Set soft-start rate of change (soft-start ramp rate) of active power
    // output due to command or internal action, defined in %setWMax / second.
    // Resolution is in hundredths of a percent/second. A value of 0 means there
    // is no limit. Interpreted as a percentage change in output capability
    // limit per second when used as a ramp rate.
    #[yaserde(rename = "setSoftGradW")]
    pub set_soft_grad_w: Option<Uint16>,

    // AC voltage nominal setting.
    #[yaserde(rename = "setVNom")]
    pub set_v_nom: Option<VoltageRMS>,

    // The nominal AC voltage (RMS) at the utility's point of common coupling.
    #[yaserde(rename = "setVRef")]
    pub set_v_ref: Option<VoltageRMS>,

    // The nominal AC voltage (RMS) offset between the DER's electrical
    // connection point and the utility's point of common coupling.
    #[yaserde(rename = "setVRefOfs")]
    pub set_v_ref_ofs: Option<VoltageRMS>,

    // Specifies the time at which the DER information was last updated.
    #[yaserde(rename = "updatedTime")]
    pub updated_time: TimeType,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Dersettings {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum Dertype {
    // Not applicable / Unknown
    #[default]
    Unknown = 0,
    VirtualOrMixedDER = 1,
    ReciprocatingEngine = 2,
    FuelCell = 3,
    PhotovoltaicSystem = 4,
    HeatAndPower = 5,
    // Other generation system
    OtherGeneration = 6,
    // Other storage system
    OtherStorage = 80,
    ElectricVehicle = 81,
    EVSE = 82,
    CombinedPVAndStorage = 83,
    // ALL OTHERS RESERVED
}

impl Validate for Dertype {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Deravailability {
    // Indicates number of seconds the DER will be able to deliver active power
    // at the reservePercent level.
    #[yaserde(rename = "availabilityDuration")]
    pub availability_duration: Option<Uint32>,

    // Indicates number of seconds the DER will be able to receive active power
    // at the reserveChargePercent level.
    #[yaserde(rename = "maxChargeDuration")]
    pub max_charge_duration: Option<Uint32>,

    // The timestamp when the DER availability was last updated.
    #[yaserde(rename = "readingTime")]
    pub reading_time: TimeType,

    // Percent of continuous received active power (%setMaxChargeRateW) that is
    // estimated to be available in reserve.
    #[yaserde(rename = "reserveChargePercent")]
    pub reserve_charge_percent: Option<PerCent>,

    // Percent of continuous delivered active power (%setMaxW) that is estimated
    // to be available in reserve.
    #[yaserde(rename = "reservePercent")]
    pub reserve_percent: Option<PerCent>,

    // Estimated reserve reactive power, in var. Represents the lesser of
    // received or delivered reactive power.
    #[yaserde(rename = "statVarAvail")]
    pub stat_var_avail: Option<ReactivePower>,

    // Estimated reserve active power, in watts.
    #[yaserde(rename = "statWAvail")]
    pub stat_w_avail: Option<ActivePower>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Deravailability {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Dercapability {
    // Bitmap indicating the DER Controls implemented by the device. See
    // DERControlType for values.
    #[yaserde(rename = "modesSupported")]
    pub modes_supported: DercontrolType,

    // Abnormal operating performance category as defined by IEEE 1547-2018. One
    // of:
    // 0 - not specified
    // 1 - Category I
    // 2 - Category II
    // 3 - Category III
    // All other values reserved.
    #[yaserde(rename = "rtgAbnormalCategory")]
    pub rtg_abnormal_category: Option<Uint8>,

    // Maximum continuous AC current capability of the DER, in Amperes (RMS).
    #[yaserde(rename = "rtgMaxA")]
    pub rtg_max_a: Option<CurrentRMS>,

    // Usable energy storage capacity of the DER, in AmpHours.
    #[yaserde(rename = "rtgMaxAh")]
    pub rtg_max_ah: Option<AmpereHour>,

    // Maximum apparent power charge rating in Volt-Amperes. May differ from the
    // maximum apparent power rating.
    #[yaserde(rename = "rtgMaxChargeRateVA")]
    pub rtg_max_charge_rate_va: Option<ApparentPower>,

    // Maximum rate of energy transfer received by the storage DER, in Watts.
    #[yaserde(rename = "rtgMaxChargeRateW")]
    pub rtg_max_charge_rate_w: Option<ActivePower>,

    // Maximum apparent power discharge rating in Volt-Amperes. May differ from
    // the maximum apparent power rating.
    #[yaserde(rename = "rtgMaxDischargeRateVA")]
    pub rtg_max_discharge_rate_va: Option<ApparentPower>,

    // Maximum rate of energy transfer delivered by the storage DER, in Watts.
    // Required for combined generation/storage DERs (e.g. DERType == 83).
    #[yaserde(rename = "rtgMaxDischargeRateW")]
    pub rtg_max_discharge_rate_w: Option<ActivePower>,

    // AC voltage maximum rating.
    #[yaserde(rename = "rtgMaxV")]
    pub rtg_max_v: Option<VoltageRMS>,

    // Maximum continuous apparent power output capability of the DER, in VA.
    #[yaserde(rename = "rtgMaxVA")]
    pub rtg_max_va: Option<ApparentPower>,

    // Maximum continuous reactive power delivered by the DER, in var.
    #[yaserde(rename = "rtgMaxVar")]
    pub rtg_max_var: Option<ReactivePower>,

    // Maximum continuous reactive power received by the DER, in var. If absent,
    // defaults to negative rtgMaxVar.
    #[yaserde(rename = "rtgMaxVarNeg")]
    pub rtg_max_var_neg: Option<ReactivePower>,

    // Maximum continuous active power output capability of the DER, in watts.
    // Represents combined generation plus storage output if DERType == 83.
    #[yaserde(rename = "rtgMaxW")]
    pub rtg_max_w: ActivePower,

    // Maximum energy storage capacity of the DER, in WattHours.
    #[yaserde(rename = "rtgMaxWh")]
    pub rtg_max_wh: Option<WattHour>,

    // Minimum Power Factor displacement capability of the DER when injecting
    // reactive power (over-excited); SHALL be a positive value between 0.0
    // (typically &gt; 0.7) and 1.0. If absent, defaults to unity.
    #[yaserde(rename = "rtgMinPFOverExcited")]
    pub rtg_min_pf_over_excited: Option<PowerFactor>,

    // Minimum Power Factor displacement capability of the DER when absorbing
    // reactive power (under-excited); SHALL be a positive value between 0.0
    // (typically &gt; 0.7) and 0.9999. If absent, defaults to
    // rtgMinPFOverExcited.
    #[yaserde(rename = "rtgMinPFUnderExcited")]
    pub rtg_min_pf_under_excited: Option<PowerFactor>,

    // AC voltage minimum rating.
    #[yaserde(rename = "rtgMinV")]
    pub rtg_min_v: Option<VoltageRMS>,

    // Normal operating performance category as defined by IEEE 1547-2018. One
    // of:
    // 0 - not specified
    // 1 - Category A
    // 2 - Category B
    // All other values reserved.
    #[yaserde(rename = "rtgNormalCategory")]
    pub rtg_normal_category: Option<Uint8>,

    // Specified over-excited power factor.
    #[yaserde(rename = "rtgOverExcitedPF")]
    pub rtg_over_excited_pf: Option<PowerFactor>,

    // Active power rating in Watts at specified over-excited power factor
    // (rtgOverExcitedPF). If present, rtgOverExcitedPF SHALL be present.
    #[yaserde(rename = "rtgOverExcitedW")]
    pub rtg_over_excited_w: Option<ActivePower>,

    // Reactive susceptance that remains connected to the Area EPS in the cease
    // to energize and trip state.
    #[yaserde(rename = "rtgReactiveSusceptance")]
    pub rtg_reactive_susceptance: Option<ReactiveSusceptance>,

    // Specified under-excited power factor.
    #[yaserde(rename = "rtgUnderExcitedPF")]
    pub rtg_under_excited_pf: Option<PowerFactor>,

    // Active power rating in Watts at specified under-excited power factor
    // (rtgUnderExcitedPF). If present, rtgUnderExcitedPF SHALL be present.
    #[yaserde(rename = "rtgUnderExcitedW")]
    pub rtg_under_excited_w: Option<ActivePower>,

    // AC voltage nominal rating.
    #[yaserde(rename = "rtgVNom")]
    pub rtg_v_nom: Option<VoltageRMS>,

    // Type of DER; see DERType object
    #[yaserde(rename = "type")]
    pub _type: Dertype,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Dercapability {}

// Distributed Energy Resource (DER) control values.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DercontrolBase {
    // Set DER as connected (true) or disconnected (false). Used in conjunction
    // with ramp rate when re-connecting. Implies galvanic isolation.
    #[yaserde(rename = "opModConnect")]
    pub op_mod_connect: Option<bool>,

    // Set DER as energized (true) or de-energized (false). Used in conjunction
    // with ramp rate when re-energizing.
    #[yaserde(rename = "opModEnergize")]
    pub op_mod_energize: Option<bool>,

    // The opModFixedPFAbsorbW function specifies a requested fixed Power Factor
    // (PF) setting for when active power is being absorbed. The actual
    // displacement SHALL be within the limits established by
    // setMinPFOverExcited and setMinPFUnderExcited. If issued simultaneously
    // with other reactive power controls (e.g. opModFixedVar) the control
    // resulting in least var magnitude SHOULD take precedence.
    #[yaserde(rename = "opModFixedPFAbsorbW")]
    pub op_mod_fixed_pf_absorb_w: Option<PowerFactorWithExcitation>,

    // The opModFixedPFInjectW function specifies a requested fixed Power Factor
    // (PF) setting for when active power is being injected. The actual
    // displacement SHALL be within the limits established by
    // setMinPFOverExcited and setMinPFUnderExcited. If issued simultaneously
    // with other reactive power controls (e.g. opModFixedVar) the control
    // resulting in least var magnitude SHOULD take precedence.
    #[yaserde(rename = "opModFixedPFInjectW")]
    pub op_mod_fixed_pf_inject_w: Option<PowerFactorWithExcitation>,

    // The opModFixedVar function specifies the delivered or received reactive
    // power setpoint. The context for the setpoint value is determined by
    // refType and SHALL be one of %setMaxW, %setMaxVar, or %statVarAvail. If
    // issued simultaneously with other reactive power controls (e.g.
    // opModFixedPFInjectW) the control resulting in least var magnitude SHOULD
    // take precedence.
    #[yaserde(rename = "opModFixedVar")]
    pub op_mod_fixed_var: Option<FixedVar>,

    // The opModFixedW function specifies a requested charge or discharge mode
    // setpoint, in %setMaxChargeRateW if negative value or %setMaxW or
    // %setMaxDischargeRateW if positive value (in hundredths).
    #[yaserde(rename = "opModFixedW")]
    pub op_mod_fixed_w: Option<SignedPerCent>,

    // Specifies a frequency-watt operation. This operation limits active power
    // generation or consumption when the line frequency deviates from nominal
    // by a specified amount.
    #[yaserde(rename = "opModFreqDroop")]
    pub op_mod_freq_droop: Option<FreqDroopType>,

    // Specify DERCurveLink for curveType == 0. The Frequency-Watt function
    // limits active power generation or consumption when the line frequency
    // deviates from nominal by a specified amount. The Frequency-Watt curve is
    // specified as an array of Frequency-Watt pairs that are interpolated into
    // a piecewise linear function with hysteresis. The x value of each pair
    // specifies a frequency in Hz. The y value specifies a corresponding active
    // power output in %setMaxW.
    #[yaserde(rename = "opModFreqWatt")]
    pub op_mod_freq_watt: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 1. The High Frequency Ride-Through
    // (HFRT) function is specified by one or two duration-frequency curves that
    // define the operating region under high frequency conditions. Each HFRT
    // curve is specified by an array of duration-frequency pairs that will be
    // interpolated into a piecewise linear function that defines an operating
    // region. The x value of each pair specifies a duration (time at a given
    // frequency in seconds). The y value of each pair specifies a frequency, in
    // Hz. This control specifies the "may trip" region.
    #[yaserde(rename = "opModHFRTMayTrip")]
    pub op_mod_hfrt_may_trip: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 2. The High Frequency Ride-Through
    // (HFRT) function is specified by a duration-frequency curve that defines
    // the operating region under high frequency conditions. Each HFRT curve is
    // specified by an array of duration-frequency pairs that will be
    // interpolated into a piecewise linear function that defines an operating
    // region. The x value of each pair specifies a duration (time at a given
    // frequency in seconds). The y value of each pair specifies a frequency, in
    // Hz. This control specifies the "must trip" region.
    #[yaserde(rename = "opModHFRTMustTrip")]
    pub op_mod_hfrt_must_trip: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 3. The High Voltage Ride-Through
    // (HVRT) function is specified by one, two, or three duration-volt curves
    // that define the operating region under high voltage conditions. Each HVRT
    // curve is specified by an array of duration-volt pairs that will be
    // interpolated into a piecewise linear function that defines an operating
    // region. The x value of each pair specifies a duration (time at a given
    // voltage in seconds). The y value of each pair specifies an effective
    // percentage voltage, defined as ((locally measured voltage - setVRefOfs /
    // setVRef). This control specifies the "may trip" region.
    #[yaserde(rename = "opModHVRTMayTrip")]
    pub op_mod_hvrt_may_trip: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 4. The High Voltage Ride-Through
    // (HVRT) function is specified by duration-volt curves that define the
    // operating region under high voltage conditions. Each HVRT curve is
    // specified by an array of duration-volt pairs that will be interpolated
    // into a piecewise linear function that defines an operating region. The x
    // value of each pair specifies a duration (time at a given voltage in
    // seconds). The y value of each pair specifies an effective percent
    // voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef).
    // This control specifies the "momentary cessation" region.
    #[yaserde(rename = "opModHVRTMomentaryCessation")]
    pub op_mod_hvrt_momentary_cessation: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 5. The High Voltage Ride-Through
    // (HVRT) function is specified by duration-volt curves that define the
    // operating region under high voltage conditions. Each HVRT curve is
    // specified by an array of duration-volt pairs that will be interpolated
    // into a piecewise linear function that defines an operating region. The x
    // value of each pair specifies a duration (time at a given voltage in
    // seconds). The y value of each pair specifies an effective percent
    // voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef).
    // This control specifies the "must trip" region.
    #[yaserde(rename = "opModHVRTMustTrip")]
    pub op_mod_hvrt_must_trip: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 6. The Low Frequency Ride-Through
    // (LFRT) function is specified by one or two duration-frequency curves that
    // define the operating region under low frequency conditions. Each LFRT
    // curve is specified by an array of duration-frequency pairs that will be
    // interpolated into a piecewise linear function that defines an operating
    // region. The x value of each pair specifies a duration (time at a given
    // frequency in seconds). The y value of each pair specifies a frequency, in
    // Hz. This control specifies the "may trip" region.
    #[yaserde(rename = "opModLFRTMayTrip")]
    pub op_mod_lfrt_may_trip: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 7. The Low Frequency Ride-Through
    // (LFRT) function is specified by a duration-frequency curve that defines
    // the operating region under low frequency conditions. Each LFRT curve is
    // specified by an array of duration-frequency pairs that will be
    // interpolated into a piecewise linear function that defines an operating
    // region. The x value of each pair specifies a duration (time at a given
    // frequency in seconds). The y value of each pair specifies a frequency, in
    // Hz. This control specifies the "must trip" region.
    #[yaserde(rename = "opModLFRTMustTrip")]
    pub op_mod_lfrt_must_trip: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 8. The Low Voltage Ride-Through
    // (LVRT) function is specified by one, two, or three duration-volt curves
    // that define the operating region under low voltage conditions. Each LVRT
    // curve is specified by an array of duration-volt pairs that will be
    // interpolated into a piecewise linear function that defines an operating
    // region. The x value of each pair specifies a duration (time at a given
    // voltage in seconds). The y value of each pair specifies an effective
    // percent voltage, defined as ((locally measured voltage - setVRefOfs) /
    // setVRef). This control specifies the "may trip" region.
    #[yaserde(rename = "opModLVRTMayTrip")]
    pub op_mod_lvrt_may_trip: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 9. The Low Voltage Ride-Through
    // (LVRT) function is specified by duration-volt curves that define the
    // operating region under low voltage conditions. Each LVRT curve is
    // specified by an array of duration-volt pairs that will be interpolated
    // into a piecewise linear function that defines an operating region. The x
    // value of each pair specifies a duration (time at a given voltage in
    // seconds). The y value of each pair specifies an effective percent
    // voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef).
    // This control specifies the "momentary cessation" region.
    #[yaserde(rename = "opModLVRTMomentaryCessation")]
    pub op_mod_lvrt_momentary_cessation: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 10. The Low Voltage Ride-Through
    // (LVRT) function is specified by duration-volt curves that define the
    // operating region under low voltage conditions. Each LVRT curve is
    // specified by an array of duration-volt pairs that will be interpolated
    // into a piecewise linear function that defines an operating region. The x
    // value of each pair specifies a duration (time at a given voltage in
    // seconds). The y value of each pair specifies an effective percent
    // voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef).
    // This control specifies the "must trip" region.
    #[yaserde(rename = "opModLVRTMustTrip")]
    pub op_mod_lvrt_must_trip: Option<DercurveLink>,

    // The opModMaxLimW function sets the maximum active power generation level
    // at the electrical coupling point as a percentage of set capacity
    // (%setMaxW, in hundredths). This limitation may be met e.g. by reducing PV
    // output or by using excess PV output to charge associated storage.
    #[yaserde(rename = "opModMaxLimW")]
    pub op_mod_max_lim_w: Option<PerCent>,

    // Target reactive power, in var. This control is likely to be more useful
    // for aggregators, as individual DERs may not be able to maintain a target
    // setting.
    #[yaserde(rename = "opModTargetVar")]
    pub op_mod_target_var: Option<ReactivePower>,

    // Target output power, in Watts. This control is likely to be more useful
    // for aggregators, as individual DERs may not be able to maintain a target
    // setting.
    #[yaserde(rename = "opModTargetW")]
    pub op_mod_target_w: Option<ActivePower>,

    // Specify DERCurveLink for curveType == 11. The static volt-var function
    // provides over- or under-excited var compensation as a function of
    // measured voltage. The volt-var curve is specified as an array of volt-var
    // pairs that are interpolated into a piecewise linear function with
    // hysteresis. The x value of each pair specifies an effective percent
    // voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef)
    // and SHOULD support a domain of at least 0 - 135. If VRef is present in
    // DERCurve, then the x value of each pair is additionally multiplied by
    // (VRef / 10000). The y value specifies a target var output interpreted as
    // a signed percentage (-100 to 100). The meaning of the y value is
    // determined by yRefType and must be one of %setMaxW, %setMaxVar, or
    // %statVarAvail.
    #[yaserde(rename = "opModVoltVar")]
    pub op_mod_volt_var: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 12. The Volt-Watt reduces active
    // power output as a function of measured voltage. The Volt-Watt curve is
    // specified as an array of Volt-Watt pairs that are interpolated into a
    // piecewise linear function with hysteresis. The x value of each pair
    // specifies an effective percent voltage, defined as ((locally measured
    // voltage - setVRefOfs) / setVRef) and SHOULD support a domain of at least
    // 0 - 135. The y value specifies an active power output interpreted as a
    // percentage (0 - 100). The meaning of the y value is determined by
    // yRefType and must be one of %setMaxW or %statWAvail.
    #[yaserde(rename = "opModVoltWatt")]
    pub op_mod_volt_watt: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 13. The Watt-PF function varies
    // Power Factor (PF) as a function of delivered active power. The Watt-PF
    // curve is specified as an array of Watt-PF coordinates that are
    // interpolated into a piecewise linear function with hysteresis. The x
    // value of each pair specifies a watt setting in %setMaxW, (0 - 100). The
    // PF output setting is a signed displacement in y value (PF sign SHALL be
    // interpreted according to the EEI convention, where unity PF is considered
    // unsigned). These settings are not expected to be updated very often
    // during the life of the installation, therefore only a single curve is
    // required. If issued simultaneously with other reactive power controls
    // (e.g. opModFixedPFInjectW) the control resulting in least var magnitude
    // SHOULD take precedence.
    #[yaserde(rename = "opModWattPF")]
    pub op_mod_watt_pf: Option<DercurveLink>,

    // Specify DERCurveLink for curveType == 14. The Watt-Var function varies
    // vars as a function of delivered active power. The Watt-Var curve is
    // specified as an array of Watt-Var pairs that are interpolated into a
    // piecewise linear function with hysteresis. The x value of each pair
    // specifies a watt setting in %setMaxW, (0-100). The y value specifies a
    // target var output interpreted as a signed percentage (-100 to 100). The
    // meaning of the y value is determined by yRefType and must be one of
    // %setMaxW, %setMaxVar, or %statVarAvail.
    #[yaserde(rename = "opModWattVar")]
    pub op_mod_watt_var: Option<DercurveLink>,

    // Requested ramp time, in hundredths of a second, for the device to
    // transition from the current DERControl mode setting(s) to the new mode
    // setting(s). If absent, use default ramp rate (setGradW). Resolution is
    // 1/100 sec.
    #[yaserde(rename = "rampTms")]
    pub ramp_tms: Option<Uint16>,
}

impl Validate for DercontrolBase {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Dercontrol {
    #[yaserde(rename = "DERControlBase")]
    pub der_control_base: DercontrolBase,

    // Specifies the bitmap indicating the categories of devices that SHOULD
    // respond. Devices SHOULD ignore events that do not indicate their device
    // category. If not present, all devices SHOULD respond.
    #[yaserde(rename = "deviceCategory")]
    pub device_category: Option<DeviceCategoryType>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval duration, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeDuration")]
    pub randomize_duration: Option<OneHourRangeType>,

    // Number of seconds boundary inside which a random value must be selected
    // to be applied to the associated interval start time, to avoid sudden
    // synchronized demand changes. If related to price level changes, sign may
    // be ignored. Valid range is -3600 to 3600. If not specified, 0 is the
    // default.
    #[yaserde(rename = "randomizeStart")]
    pub randomize_start: Option<OneHourRangeType>,

    // The time at which the Event was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "EventStatus")]
    pub event_status: EventStatus,

    // The period during which the Event applies.
    #[yaserde(rename = "interval")]
    pub interval: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the response resource address (URI). Required on a
    // response to a GET if responseRequired is "true".
    #[yaserde(attribute, rename = "replyTo")]
    pub reply_to: Option<String>,

    // Indicates whether or not a response is required upon receipt, creation or
    // update of this resource. Responses shall be posted to the collection
    // specified in "replyTo".
    // If the resource has a deviceCategory field, devices that match one or
    // more of the device types indicated in deviceCategory SHALL respond
    // according to the rules listed below. If the category does not match, the
    // device SHALL NOT respond. If the resource does not have a deviceCategory
    // field, a device receiving the resource SHALL respond according to the
    // rules listed below.
    // Value encoded as hex according to the following bit assignments, any
    // combination is possible.
    // See Table 27 for the list of appropriate Response status codes to be sent
    // for these purposes.
    // 0 - End device shall indicate that message was received
    // 1 - End device shall indicate specific response.
    // 2 - End user / customer response is required.
    // All other values reserved.
    #[yaserde(attribute, rename = "responseRequired")]
    pub response_required: Option<HexBinary8>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Dercontrol {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DercontrolList {
    #[yaserde(rename = "DERControl")]
    pub der_control: Vec<Dercontrol>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DercontrolList {}

type DercontrolType = Uint32;

#[repr(u32)]
pub enum DercontrolTypeFlags {
    ChargeMode = 1,
    DischargeMode = 2,
    OpModConnect = 4,
    OpModEnergize = 8,
    OpModFixedPFAbsorbW = 16,
    OpModFixedPFInjectW = 32,
    OpModFixedVar = 64,
    OpModFixedW = 128,
    OpModFreqDroop = 256,
    OpModFreqWatt = 512,
    OpModHFRTMayTrip = 1024,
    OpModHFRTMustTrip = 2048,
    OpModHVRTMayTrip = 4096,
    OpModHVRTMomentaryCessation = 8192,
    OpModHVRTMustTrip = 16384,
    OpModLFRTMayTrip = 32768,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Dercurve {
    // If the curveType is opModVoltVar, then this field MAY be present. If the
    // curveType is not opModVoltVar, then this field SHALL NOT be present.
    // Enable/disable autonomous vRef adjustment. When enabled, the Volt-Var
    // curve characteristic SHALL be adjusted autonomously as vRef changes and
    // autonomousVRefTimeConstant SHALL be present. If a DER is able to support
    // Volt-Var mode but is unable to support autonomous vRef adjustment, then
    // the DER SHALL execute the curve without autonomous vRef adjustment. If
    // not specified, then the value is false.
    #[yaserde(rename = "autonomousVRefEnable")]
    pub autonomous_v_ref_enable: Option<bool>,

    // If the curveType is opModVoltVar, then this field MAY be present. If the
    // curveType is not opModVoltVar, then this field SHALL NOT be present.
    // Adjustment range for vRef time constant, in hundredths of a second.
    #[yaserde(rename = "autonomousVRefTimeConstant")]
    pub autonomous_v_ref_time_constant: Option<Uint32>,

    // The time at which the object was created.
    #[yaserde(rename = "creationTime")]
    pub creation_time: TimeType,

    #[yaserde(rename = "CurveData")]
    pub curve_data: Vec<CurveData>,

    // Specifies the associated curve-based control mode.
    #[yaserde(rename = "curveType")]
    pub curve_type: DercurveType,

    // Open loop response time, the time to ramp up to 90% of the new target in
    // response to the change in voltage, in hundredths of a second. Resolution
    // is 1/100 sec. A value of 0 is used to mean no limit. When not present,
    // the device SHOULD follow its default behavior.
    #[yaserde(rename = "openLoopTms")]
    pub open_loop_tms: Option<Uint16>,

    // Decreasing ramp rate, interpreted as a percentage change in output
    // capability limit per second (e.g. %setMaxW / sec). Resolution is in
    // hundredths of a percent/second. A value of 0 means there is no limit. If
    // absent, ramp rate defaults to setGradW.
    #[yaserde(rename = "rampDecTms")]
    pub ramp_dec_tms: Option<Uint16>,

    // Increasing ramp rate, interpreted as a percentage change in output
    // capability limit per second (e.g. %setMaxW / sec). Resolution is in
    // hundredths of a percent/second. A value of 0 means there is no limit. If
    // absent, ramp rate defaults to rampDecTms.
    #[yaserde(rename = "rampIncTms")]
    pub ramp_inc_tms: Option<Uint16>,

    // The configuration parameter for a low-pass filter, PT1 is a time, in
    // hundredths of a second, in which the filter will settle to 95% of a step
    // change in the input value. Resolution is 1/100 sec.
    #[yaserde(rename = "rampPT1Tms")]
    pub ramp_pt1_tms: Option<Uint16>,

    // If the curveType is opModVoltVar, then this field MAY be present. If the
    // curveType is not opModVoltVar, then this field SHALL NOT be present. The
    // nominal AC voltage (RMS) adjustment to the voltage curve points for
    // Volt-Var curves.
    #[yaserde(rename = "vRef")]
    pub v_ref: Option<PerCent>,

    // Exponent for X-axis value.
    #[yaserde(rename = "xMultiplier")]
    pub x_multiplier: PowerOfTenMultiplierType,

    // Exponent for Y-axis value.
    #[yaserde(rename = "yMultiplier")]
    pub y_multiplier: PowerOfTenMultiplierType,

    // The Y-axis units context.
    #[yaserde(rename = "yRefType")]
    pub y_ref_type: DerunitRefType,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Dercurve {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CurrentDERProgramLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for CurrentDERProgramLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DercurveList {
    #[yaserde(rename = "DERCurve")]
    pub der_curve: Vec<Dercurve>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DercurveList {}

// Data point values for defining a curve or schedule
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CurveData {
    // If yvalue is Power Factor, then this field SHALL be present. If yvalue is
    // not Power Factor, then this field SHALL NOT be present.
    // True when DER is absorbing reactive power (under-excited), false
    // when DER is injecting reactive power (over-excited).
    #[yaserde(rename = "excitation")]
    pub excitation: Option<bool>,

    // The data value of the X-axis (independent) variable, depending on the
    // curve type. See definitions in DERControlBase for further information.
    #[yaserde(rename = "xvalue")]
    pub xvalue: Int32,

    // The data value of the Y-axis (dependent) variable, depending on the curve
    // type. See definitions in DERControlBase for further information. If
    // yvalue is Power Factor, the excitation field SHALL be present and yvalue
    // SHALL be a positive value. If yvalue is not Power Factor, the excitation
    // field SHALL NOT be present.
    #[yaserde(rename = "yvalue")]
    pub yvalue: Int32,
}

impl Validate for CurveData {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum DercurveType {
    #[default]
    OpModFreqWatt = 0, // (Frequency-Watt Curve Mode)
    OpModHfrtmayTrip = 1,  // (High Frequency Ride Through, May Trip Mode)
    OpModHfrtmustTrip = 2, // High Frequency Ride Through, Must Trip Mode)
    OpModHvrtmayTrip = 3,  // (High Voltage Ride Through, May Trip Mode)
    OpModHvrtmomentaryCessation = 4, // (High Voltage Ride Through, Momentary Cessation Mode)
    OpModHvrtmustTrip = 5, // (High Voltage Ride Through, Must Trip Mode)
    OpModLfrtmayTrip = 6,  // (Low Frequency Ride Through, May Trip Mode)
    OpModLfrtmustTrip = 7, // (Low Frequency Ride Through, Must Trip Mode)
    OpModLvrtmayTrip = 8,  // (Low Voltage Ride Through, May Trip Mode)
    OpModLvrtmomentaryCessation = 9, // (Low Voltage Ride Through, Momentary Cessation Mode)
    OpModLvrtmustTrip = 10, // (Low Voltage Ride Through, Must Trip Mode)
    OpModVoltVar = 11,     // (Volt-Var Mode)
    OpModVoltWatt = 12,    // (Volt-Watt Mode)
    OpModWattPf = 13,      // (Watt-PowerFactor Mode)
    OpModWattVar = 14,     // (Watt-Var Mode)
}

impl Validate for DercurveType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Derprogram {
    #[yaserde(rename = "ActiveDERControlListLink")]
    pub active_der_control_list_link: Option<ActiveDERControlListLink>,

    #[yaserde(rename = "DefaultDERControlLink")]
    pub default_der_control_link: Option<DefaultDERControlLink>,

    #[yaserde(rename = "DERControlListLink")]
    pub der_control_list_link: Option<DercontrolListLink>,

    #[yaserde(rename = "DERCurveListLink")]
    pub der_curve_list_link: Option<DercurveListLink>,

    // Indicates the relative primacy of the provider of this Program.
    #[yaserde(rename = "primacy")]
    pub primacy: PrimacyType,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Derprogram {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DerprogramList {
    #[yaserde(rename = "DERProgram")]
    pub der_program: Vec<Derprogram>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on GET,
    // ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for DerprogramList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Derstatus {
    // Bitmap indicating the status of DER alarms (see DER LogEvents for more
    // details).
    // 0 - DER_FAULT_OVER_CURRENT
    // 1 - DER_FAULT_OVER_VOLTAGE
    // 2 - DER_FAULT_UNDER_VOLTAGE
    // 3 - DER_FAULT_OVER_FREQUENCY
    // 4 - DER_FAULT_UNDER_FREQUENCY
    // 5 - DER_FAULT_VOLTAGE_IMBALANCE
    // 6 - DER_FAULT_CURRENT_IMBALANCE
    // 7 - DER_FAULT_EMERGENCY_LOCAL
    // 8 - DER_FAULT_EMERGENCY_REMOTE
    // 9 - DER_FAULT_LOW_POWER_INPUT
    // 10 - DER_FAULT_PHASE_ROTATION
    // 11-31 - Reserved
    #[yaserde(rename = "alarmStatus")]
    pub alarm_status: Option<HexBinary32>,

    // Connect/status value for generator DER.
    // See ConnectStatusType for values.
    #[yaserde(rename = "genConnectStatus")]
    pub gen_connect_status: Option<ConnectStatusType>,

    // DER InverterStatus/value.
    // See InverterStatusType for values.
    #[yaserde(rename = "inverterStatus")]
    pub inverter_status: Option<InverterStatusType>,

    // The local control mode status.
    // See LocalControlModeStatusType for values.
    #[yaserde(rename = "localControlModeStatus")]
    pub local_control_mode_status: Option<LocalControlModeStatusType>,

    // Manufacturer status code.
    #[yaserde(rename = "manufacturerStatus")]
    pub manufacturer_status: Option<ManufacturerStatusType>,

    // Operational mode currently in use.
    // See OperationalModeStatusType for values.
    #[yaserde(rename = "operationalModeStatus")]
    pub operational_mode_status: Option<OperationalModeStatusType>,

    // The timestamp when the current status was last updated.
    #[yaserde(rename = "readingTime")]
    pub reading_time: TimeType,

    // State of charge status.
    // See StateOfChargeStatusType for values.
    #[yaserde(rename = "stateOfChargeStatus")]
    pub state_of_charge_status: Option<StateOfChargeStatusType>,

    // Storage mode status.
    // See StorageModeStatusType for values.
    #[yaserde(rename = "storageModeStatus")]
    pub storage_mode_status: Option<StorageModeStatusType>,

    // Connect/status value for storage DER.
    // See ConnectStatusType for values.
    #[yaserde(rename = "storConnectStatus")]
    pub stor_connect_status: Option<ConnectStatusType>,

    // Indicates whether or not subscriptions are supported for this resource,
    // and whether or not conditional (thresholds) are supported. If not
    // specified, is "not subscribable" (0).
    #[yaserde(attribute, rename = "subscribable")]
    pub subscribable: Option<SubscribableType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for Derstatus {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum DerunitRefType {
    // Specifies context for interpreting percent values:
    #[default]
    NotApplicable = 0,
    SetMaxW = 1,
    SetMaxVar = 2,
    StatVarAvail = 3,
    SetEffectiveV = 4,
    SetMaxChargeRateW = 5,
    SetMaxDischargeRateW = 6,
    StatWAvail = 7,
    // 8-255 Reserved
}

impl Validate for DerunitRefType {}

// Average flow of charge through a conductor.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CurrentRMS {
    // Specifies exponent of value.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value in amperes RMS (uom 5)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for CurrentRMS {}

// Abstract type for specifying a fixed-point value without a given unit of
// measure.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FixedPointType {
    // Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Dimensionless value
    #[yaserde(rename = "value")]
    pub value: Int16,
}

impl Validate for FixedPointType {}

// Abstract type for specifying an unsigned fixed-point value without a given
// unit of measure.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UnsignedFixedPointType {
    // Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Dimensionless value
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for UnsignedFixedPointType {}

// The active (real) power P (in W) is the product of root-mean-square (RMS)
// voltage, RMS current, and cos(theta) where theta is the phase angle of
// current relative to voltage. It is the primary measure of the rate of flow of
// energy.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActivePower {
    // Specifies exponent for uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value in watts (uom 38)
    #[yaserde(rename = "value")]
    pub value: Int16,
}

impl Validate for ActivePower {}

// Available electric charge
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AmpereHour {
    // Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value in ampere-hours (uom 106)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for AmpereHour {}

// The apparent power S (in VA) is the product of root mean square (RMS) voltage
// and RMS current.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ApparentPower {
    // Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value in volt-amperes (uom 61)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for ApparentPower {}

// The reactive power Q (in var) is the product of root mean square (RMS)
// voltage, RMS current, and sin(theta) where theta is the phase angle of
// current relative to voltage.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReactivePower {
    // Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value in volt-amperes reactive (var) (uom 63)
    #[yaserde(rename = "value")]
    pub value: Int16,
}

impl Validate for ReactivePower {}

// Reactive susceptance
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReactiveSusceptance {
    // Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value in siemens (uom 53)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for ReactiveSusceptance {}

// Specifies a setpoint for Displacement Power Factor, the ratio between
// apparent and active powers at the fundamental frequency (e.g. 60 Hz).
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PowerFactor {
    // Significand of an unsigned value of cos(theta) between 0 and 1.0. E.g. a
    // value of 0.95 may be specified as a displacement of 950 and a multiplier
    // of -3.
    #[yaserde(rename = "displacement")]
    pub displacement: Uint16,

    // Specifies exponent of 'displacement'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,
}

impl Validate for PowerFactor {}

// Specifies a setpoint for Displacement Power Factor, the ratio between
// apparent and active powers at the fundamental frequency (e.g. 60 Hz) and
// includes an excitation flag.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PowerFactorWithExcitation {
    // Significand of an unsigned value of cos(theta) between 0 and 1.0. E.g. a
    // value of 0.95 may be specified as a displacement of 950 and a multiplier
    // of -3.
    #[yaserde(rename = "displacement")]
    pub displacement: Uint16,

    // True when DER is absorbing reactive power (under-excited), false
    // when DER is injecting reactive power (over-excited).
    #[yaserde(rename = "excitation")]
    pub excitation: bool,

    // Specifies exponent of 'displacement'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,
}

impl Validate for PowerFactorWithExcitation {}

// Specifies a signed setpoint for reactive power.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FixedVar {
    // Indicates whether to interpret 'value' as %setMaxVar or %statVarAvail.
    #[yaserde(rename = "refType")]
    pub ref_type: DerunitRefType,

    // Specify a signed setpoint for reactive power in % (see 'refType' for
    // context).
    #[yaserde(rename = "value")]
    pub value: SignedPerCent,
}

impl Validate for FixedVar {}

// Active (real) energy
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct WattHour {
    // Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value in watt-hours (uom 72)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for WattHour {}

// Average electric potential difference between two points.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct VoltageRMS {
    // Specifies exponent of uom.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value in volts RMS (uom 29)
    #[yaserde(rename = "value")]
    pub value: Uint16,
}

impl Validate for VoltageRMS {}

// DER ConnectStatus value (bitmap):
// 0 - Connected
// 1 - Available
// 2 - Operating
// 3 - Test
// 4 - Fault / Error
// All other values reserved.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ConnectStatusType {
    // The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    // The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: HexBinary8,
}

impl Validate for ConnectStatusType {}

// DER InverterStatus value:
// 0 - N/A
// 1 - off
// 2 - sleeping (auto-shutdown) or DER is at low output power/voltage
// 3 - starting up or ON but not producing power
// 4 - tracking MPPT power point
// 5 - forced power reduction/derating
// 6 - shutting down
// 7 - one or more faults exist
// 8 - standby (service on unit) - DER may be at high output voltage/power
// 9 - test mode
// 10 - as defined in manufacturer status
// All other values reserved.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct InverterStatusType {
    // The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    // The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: Uint8,
}

impl Validate for InverterStatusType {}

// DER LocalControlModeStatus/value:
// 0 – local control
// 1 – remote control
// All other values reserved.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LocalControlModeStatusType {
    // The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    // The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: Uint8,
}

impl Validate for LocalControlModeStatusType {}

// DER ManufacturerStatus/value: String data type
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ManufacturerStatusType {
    // The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    // The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: String6,
}

impl Validate for ManufacturerStatusType {}

// DER OperationalModeStatus value:
// 0 - Not applicable / Unknown
// 1 - Off
// 2 - Operational mode
// 3 - Test mode
// All other values reserved.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct OperationalModeStatusType {
    // The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    // The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: Uint8,
}

impl Validate for OperationalModeStatusType {}

// DER StateOfChargeStatus value: Percent data type
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct StateOfChargeStatusType {
    // The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    // The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: PerCent,
}

impl Validate for StateOfChargeStatusType {}

// DER StorageModeStatus value:
// 0 – storage charging
// 1 – storage discharging
// 2 – storage holding
// All other values reserved.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct StorageModeStatusType {
    // The date and time at which the state applied.
    #[yaserde(rename = "dateTime")]
    pub date_time: TimeType,

    // The value indicating the state.
    #[yaserde(rename = "value")]
    pub value: Uint8,
}

impl Validate for StorageModeStatusType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AccountBalanceLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for AccountBalanceLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveBillingPeriodListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveBillingPeriodListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveCreditRegisterListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveCreditRegisterListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveDERControlListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveDERControlListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveEndDeviceControlListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveEndDeviceControlListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveFlowReservationListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveFlowReservationListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveProjectionReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveProjectionReadingListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveSupplyInterruptionOverrideListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveSupplyInterruptionOverrideListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveTargetReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveTargetReadingListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveTextMessageListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveTextMessageListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ActiveTimeTariffIntervalListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ActiveTimeTariffIntervalListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AssociatedDERProgramListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for AssociatedDERProgramListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct AssociatedUsagePointLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for AssociatedUsagePointLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingPeriodListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for BillingPeriodListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for BillingReadingListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct BillingReadingSetListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for BillingReadingSetListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ConfigurationLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ConfigurationLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ConsumptionTariffIntervalListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ConsumptionTariffIntervalListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CreditRegisterListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for CreditRegisterListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAccountLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for CustomerAccountLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAccountListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for CustomerAccountListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct CustomerAgreementListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for CustomerAgreementListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DemandResponseProgramLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DemandResponseProgramLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DemandResponseProgramListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DemandResponseProgramListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeravailabilityLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DeravailabilityLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DefaultDERControlLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DefaultDERControlLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DercapabilityLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DercapabilityLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DercontrolListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DercontrolListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DercurveLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DercurveLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DercurveListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DercurveListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Derlink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for Derlink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DerlistLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DerlistLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DerprogramLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DerprogramLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DerprogramListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DerprogramListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DersettingsLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DersettingsLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DerstatusLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DerstatusLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceCapabilityLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DeviceCapabilityLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceInformationLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DeviceInformationLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DeviceStatusLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for DeviceStatusLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceControlListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for EndDeviceControlListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for EndDeviceLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct EndDeviceListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for EndDeviceListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FileLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FileLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FileListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FileListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FileStatusLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FileStatusLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationRequestListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FlowReservationRequestListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FlowReservationResponseListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FlowReservationResponseListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct FunctionSetAssignmentsListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for FunctionSetAssignmentsListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct HistoricalReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for HistoricalReadingListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IpaddrListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for IpaddrListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct IpinterfaceListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for IpinterfaceListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LlinterfaceListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for LlinterfaceListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LoadShedAvailabilityListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for LoadShedAvailabilityListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LogEventListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for LogEventListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MessagingProgramListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for MessagingProgramListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MeterReadingLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for MeterReadingLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MeterReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for MeterReadingListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorUsagePointListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for MirrorUsagePointListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct NeighborListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for NeighborListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct NotificationListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for NotificationListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PowerStatusLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for PowerStatusLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PrepaymentLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for PrepaymentLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PrepaymentListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for PrepaymentListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PrepayOperationStatusLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for PrepayOperationStatusLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PriceResponseCfgListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for PriceResponseCfgListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ProjectionReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ProjectionReadingListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RateComponentLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for RateComponentLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RateComponentListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for RateComponentListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ReadingLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ReadingListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingSetListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ReadingSetListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ReadingTypeLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ReadingTypeLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RegistrationLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for RegistrationLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ResponseListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ResponseListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ResponseSetListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ResponseSetListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RplinstanceListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for RplinstanceListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RplsourceRoutesListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for RplsourceRoutesListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SelfDeviceLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for SelfDeviceLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ServiceSupplierLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for ServiceSupplierLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SubscriptionListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for SubscriptionListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupplyInterruptionOverrideListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for SupplyInterruptionOverrideListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SupportedLocaleListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for SupportedLocaleListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TargetReadingListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TargetReadingListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TariffProfileLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TariffProfileLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TariffProfileListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TariffProfileListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TextMessageListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TextMessageListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TimeLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TimeLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TimeTariffIntervalListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for TimeTariffIntervalListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UsagePointLink {
    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for UsagePointLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UsagePointListLink {
    // Indicates the total number of items in the referenced list. This
    // attribute SHALL be present if the href is a local or relative URI. This
    // attribute SHOULD NOT be present if the href is a remote or absolute URI,
    // as the server may be unaware of changes to the value.
    #[yaserde(attribute, rename = "all")]
    pub all: Option<Uint32>,

    // A URI reference.
    #[yaserde(attribute, rename = "href")]
    pub href: String,
}

impl Validate for UsagePointListLink {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum AccumulationBehaviourType {
    #[default]
    // (default, if not specified)
    NotApplicable = 0,
    // The sum of the previous billing period values. Note: “Cumulative” is commonly used in conjunction with “demand.” Each demand reset causes the maximum demand value for the present billing period (since the last demand reset) to accumulate as an accumulative total of all maximum demands. So instead of “zeroing” the demand register, a demand reset has the affect of adding the present maximum demand to this accumulating total.
    Cumulative = 3,
    // The difference between the value at the end of the prescribed interval and the beginning of the interval. This is used for incremental interval data.
    // Note: One common application would be for load profile data, another use might be to report the number of events within an interval (such as the number of equipment energizations within the specified period of time.)
    DeltaData = 4,
    // As if a needle is swung out on the meter face to a value to indicate the current value. (Note: An “indicating” value is typically measured over hundreds of milliseconds or greater, or may imply a “pusher” mechanism to capture a value. Compare this to “instantaneous” which is measured over a shorter period of time.)
    Indicating = 6,
    // A form of accumulation which is selective with respect to time.
    // Note : “Summation” could be considered a specialization of “Bulk Quantity” according to the rules of inheritance where “Summation” selectively accumulates pulses over a timing pattern, and “BulkQuantity” accumulates pulses all of the time.
    Summation = 9,
    // Typically measured over the fastest period of time allowed by the definition of the metric (usually milliseconds or tens of milliseconds.) (Note: “Instantaneous” was moved to attribute #3 in 61968-9Ed2 from attribute #1 in 61968-9Ed1.)
    Instantaneous = 12,
    // ALL OTHERS RESERVED
}

impl Validate for AccumulationBehaviourType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum ApplianceLoadReductionType {
    #[default]
    // Parameter requesting the appliance to respond by providing a moderate load reduction for the duration of a delay period.  Typically referring to a “non-emergency” event in which appliances can continue operating if already in a load consuming period.
    DelayApplianceLoad = 0,
    TemporaryApplianceLoadReduction = 1,
    // 2-255 reserved
    // * Full definition of how appliances react when receiving each parameter is document in the EPA document - ENERGY STAR® Program Requirements, Product Specification for Residential Refrigerators and Freezers, Eligibility Criteria 5, Draft 2 Version 5.0.
}

impl Validate for ApplianceLoadReductionType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub enum CommodityType {
    #[default]
    NotApplicable = 0, // (default, if not specified)
    ElectricitySecondaryMetered = 1, // metered value (a premises meter is typically on the low voltage, or secondary, side of a service transformer)
    ElectricityPrimaryMetered = 2, // metered value (measured on the high voltage, or primary, side of the service transformer)
    Air = 4,
    NaturalGas = 7,
    Propane = 8,
    PotableWater = 9,
    Steam = 10,
    WasteWater = 11,
    HeatingFluid = 12,
    CoolingFluid = 13,
    // All other values reserved
}

impl Validate for CommodityType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum ConsumptionBlockType {
    #[default]
    NotApplicable = 0,
    Block1 = 1,
    Block2 = 2,
    Block3 = 3,
    Block4 = 4,
    Block5 = 5,
    Block6 = 6,
    Block7 = 7,
    Block8 = 8,
    Block9 = 9,
    Block10 = 10,
    Block11 = 11,
    Block12 = 12,
    Block13 = 13,
    Block14 = 14,
    Block15 = 15,
    Block16 = 16,
    // 17-255 RESERVED
}

impl Validate for ConsumptionBlockType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u16)]
pub enum CurrencyCode {
    // Follows codes defind in ISO 4217
    #[default]
    NotApplicable = 0,
    AUD = 36,
    CAD = 124,
    USD = 840,
    EUR = 978,
}

impl Validate for CurrencyCode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum DataQualifierType {
    #[default]
    NotApplicable = 0,
    Average = 2,
    Maximum = 8,
    Minimum = 9,
    Normal = 12,
    StandardDeviationOfPopulation = 29,
    StandardDeviationOfSample = 30,
}

impl Validate for DataQualifierType {}

// Interval of date and time.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DateTimeInterval {
    // Duration of the interval, in seconds.
    #[yaserde(rename = "duration")]
    pub duration: Uint32,

    // Date and time of the start of the interval.
    #[yaserde(rename = "start")]
    pub start: TimeType,
}

impl Validate for DateTimeInterval {}

type DeviceCategoryType = HexBinary32;
pub enum DeviceCategoryTypeFlags {
    ProgrammableCommunicatingThermostat = 1,
    StripHeaters = 2,
    BaseboardHeaters = 4,
    WaterHeater = 8,
    PoolPump = 16,
    Sauna = 32,
    HotTub = 64,
    SmartAppliance = 128,
    IrrigationPump = 256,
    ManagedCommercialAndIndustrialLoads = 512,
    SimpleMiscLoads = 1024,
    ExteriorLighting = 2048,
    InteriorLighting = 4096,
    LoadControlSwitch = 8192,
    EnergyManagementSystem = 16384,
    SmartEnergyModule = 65536,
    ElectricVehicle = 262144,
    VirutalOrMixedDer = 524288,
    ReciprocatingEngine = 2097152,
    PhotovoltaicSystem = 8388608,
    CombinedPvAndStorage = 16777216,
    OtherGenerationSystem = 33554432,
    OtherStorageSystem = 67108864,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct DstRuleType {}

impl Validate for DstRuleType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum FlowDirectionType {
    #[default]
    NotApplicable = 0,
    Forward = 1,  // delivered to customer
    Reverse = 19, // received from customer
}

impl Validate for FlowDirectionType {}

// Specifies a GPS location, expressed in WGS 84 coordinates.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct GpslocationType {
    // Specifies the latitude from equator. -90 (south) to +90 (north) in
    // decimal degrees.
    #[yaserde(rename = "lat")]
    pub lat: String32,

    // Specifies the longitude from Greenwich Meridian. -180 (west) to +180
    // (east) in decimal degrees.
    #[yaserde(rename = "lon")]
    pub lon: String32,
}

impl Validate for GpslocationType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum KindType {
    #[default]
    NotApplicable = 0,
    Currency = 3,
    Demand = 8,
    Energy = 12,
    Power = 37,
}

impl Validate for KindType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct LocaleType {}

impl Validate for LocaleType {}

pub type Mridtype = HexBinary128;

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct OneHourRangeType(pub Int16);

impl Validate for OneHourRangeType {
    fn validate(&self) -> Result<(), String> {
        if self.0 .0 > "3600".parse::<i16>().unwrap() {
            return Err(format!("MaxInclusive validation error: invalid value of 0! \nExpected: 0 <= 3600.\nActual: 0 == {}", self.0));
        }
        if self.0 .0 < "-3600".parse::<i16>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= -140737488355328.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Pentype {}

impl Validate for Pentype {}

type PerCent = Uint16;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum PhaseCode {
    #[default]
    NotApplicable = 0,
    PhaseC = 32,  // and S2
    PhaseCN = 33, // and S2N
    PhaseCA = 40,
    PhaseB = 64,
    PhaseBN = 65,
    PhaseBC = 66,
    PhaseA = 128,  // and S1
    PhaseAN = 129, // and S1N
    PhaseAB = 132,
    PhaseABC = 224,
    // ALL OTHERS RESERVED
}

impl Validate for PhaseCode {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Pintype {}

impl Validate for Pintype {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PowerOfTenMultiplierType {}

impl Validate for PowerOfTenMultiplierType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct PrimacyType {}

impl Validate for PrimacyType {}

// Real electrical energy
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RealEnergy {
    // Multiplier for 'unit'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value of the energy in Watt-hours. (uom 72)
    #[yaserde(rename = "value")]
    pub value: Uint48,
}

impl Validate for RealEnergy {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct RoleFlagsType {}

impl Validate for RoleFlagsType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct ServiceKind {}

impl Validate for ServiceKind {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct Sfditype {}

impl Validate for Sfditype {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SignedPerCent {}

impl Validate for SignedPerCent {}

// Real electrical energy, signed.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct SignedRealEnergy {
    // Multiplier for 'unit'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Value of the energy in Watt-hours. (uom 72)
    #[yaserde(rename = "value")]
    pub value: Int48,
}

impl Validate for SignedRealEnergy {}

// The subscribable values.
// 0 - Resource does not support subscriptions
// 1 - Resource supports non-conditional subscriptions
// 2 - Resource supports conditional subscriptions
// 3 - Resource supports both conditional and non-conditional subscriptions
// All other values reserved.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum SubscribableType {
    #[default]
    NoSubscriptionsSupported = 0,
    NonConditionalSubscriptions = 1,
    ConditionalSubscriptions = 2,
    AllSubscriptions = 3,
}

impl Validate for SubscribableType {}
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct TimeOffsetType {}

impl Validate for TimeOffsetType {}

type TimeType = Int64;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum Toutype {
    #[default]
    NotApplicable = 0,
    TouA = 1,
    TouB = 2,
    TouC = 3,
    TouD = 4,
    TouE = 5,
    TouF = 6,
    TouG = 7,
    TouH = 8,
    TouI = 9,
    TouJ = 10,
    TouK = 11,
    TouL = 12,
    TouM = 13,
    TouN = 14,
    TouO = 15,
}

impl Validate for Toutype {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum UnitType {
    #[default]
    kWh = 0,
    kW = 1,
    Watts = 2,
    CubicMeters = 3,
    CubicFeet = 4,
    USGallons = 5,
    ImperialGallons = 6,
    BTU = 7,
    Liters = 8,
    kPAGauge = 9,
    kPAAbsolute = 10,
    Megajoule = 11,
    Unitless = 12,
}

impl Validate for UnitType {}

// Type for specification of a specific value, with units and power of ten
// multiplier.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct UnitValueType {
    // Multiplier for 'unit'.
    #[yaserde(rename = "multiplier")]
    pub multiplier: PowerOfTenMultiplierType,

    // Unit in symbol
    #[yaserde(rename = "unit")]
    pub unit: UomType,

    // Value in units specified
    #[yaserde(rename = "value")]
    pub value: Int32,
}

impl Validate for UnitValueType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum UomType {
    #[default]
    NotApplicable = 0,
    Amperes = 5,
    Kelvin = 6,
    DegreesCelsius = 23,
    Voltage = 29,
    Joule = 31,
    Hz = 33,
    W = 38,
    MtrCubed = 42,
    VA = 61,
    VAr = 63,
    CosTheta = 65,
    VSquared = 67,
    ASquared = 69,
    VAh = 71,
    Wh = 72,
    VArh = 73,
    Ah = 106,
    FtCubed = 119,
    FtCubedPerHour = 122,
    MCubedPerHour = 125,
    USGallons = 128,
    UGGallonsPerHour = 129,
    ImperialGallons = 130,
    ImperialGallonsPerHour = 131,
    BTU = 132,
    BTUPerHour = 133,
    Liter = 134,
    LiterPerHour = 137,
    PAGauge = 140,
    PAAbsolute = 155,
    Therm = 169,
}

impl Validate for UomType {}

type VersionType = Uint16;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorMeterReading {
    // The date and time of the last update.
    #[yaserde(rename = "lastUpdateTime")]
    pub last_update_time: Option<TimeType>,

    #[yaserde(rename = "MirrorReadingSet")]
    pub mirror_reading_set: Vec<MirrorReadingSet>,

    // The date and time of the next planned update.
    #[yaserde(rename = "nextUpdateTime")]
    pub next_update_time: Option<TimeType>,

    #[yaserde(rename = "Reading")]
    pub reading: Option<Reading>,

    #[yaserde(rename = "ReadingType")]
    pub reading_type: Option<ReadingType>,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MirrorMeterReading {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorMeterReadingList {
    #[yaserde(rename = "MirrorMeterReading")]
    pub mirror_meter_reading: Vec<MirrorMeterReading>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MirrorMeterReadingList {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorReadingSet {
    #[yaserde(rename = "Reading")]
    pub reading: Vec<Reading>,

    // Specifies the time range during which the contained readings were taken.
    #[yaserde(rename = "timePeriod")]
    pub time_period: DateTimeInterval,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MirrorReadingSet {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorUsagePoint {
    // The LFDI of the device being mirrored.
    #[yaserde(rename = "deviceLFDI")]
    pub device_lfdi: HexBinary160,

    #[yaserde(rename = "MirrorMeterReading")]
    pub mirror_meter_reading: Vec<MirrorMeterReading>,

    // POST rate, or how often mirrored data should be POSTed, in seconds. A
    // client MAY indicate a preferred postRate when POSTing MirrorUsagePoint. A
    // server MAY add or modify postRate to indicate its preferred posting rate.
    #[yaserde(rename = "postRate")]
    pub post_rate: Option<Uint32>,

    // Specifies the roles that apply to the usage point.
    #[yaserde(rename = "roleFlags")]
    pub role_flags: RoleFlagsType,

    // The kind of service provided by this usage point.
    #[yaserde(rename = "serviceCategoryKind")]
    pub service_category_kind: ServiceKind,

    // Specifies the current status of the service at this usage point.
    // 0 = off
    // 1 = on
    #[yaserde(rename = "status")]
    pub status: Uint8,

    // The global identifier of the object.
    #[yaserde(rename = "mRID")]
    pub m_rid: Mridtype,

    // The description is a human readable text describing or naming the object.
    #[yaserde(rename = "description")]
    pub description: Option<String32>,

    // Contains the version number of the object. See the type definition for
    // details.
    #[yaserde(rename = "version")]
    pub version: Option<VersionType>,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MirrorUsagePoint {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
pub struct MirrorUsagePointList {
    #[yaserde(rename = "MirrorUsagePoint")]
    pub mirror_usage_point: Vec<MirrorUsagePoint>,

    // The default polling rate for this function set (this resource and all
    // resources below), in seconds. If not specified, a default of 900 seconds
    // (15 minutes) is used. It is RECOMMENDED a client poll the resources of
    // this function set every pollRate seconds.
    #[yaserde(attribute, rename = "pollRate")]
    pub poll_rate: Option<Uint32>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl Validate for MirrorUsagePointList {}

// pub type DeviceCapability = DeviceCapability;
// pub type AbstractDevice = AbstractDevice;
// pub type DeviceStatus = DeviceStatus;
// pub type EndDevice = EndDevice;
// pub type EndDeviceList = EndDeviceList;
// pub type Registration = Registration;
// pub type SelfDevice = SelfDevice;
// pub type Temperature = Temperature;
// pub type FunctionSetAssignmentsBase = FunctionSetAssignmentsBase;
// pub type FunctionSetAssignments = FunctionSetAssignments;
// pub type FunctionSetAssignmentsList = FunctionSetAssignmentsList;
// pub type Condition = Condition;
// pub type SubscriptionBase = SubscriptionBase;
// pub type Subscription = Subscription;
// pub type SubscriptionList = SubscriptionList;
// pub type Notification = Notification;
// pub type NotificationList = NotificationList;
// pub type DercontrolResponse = DercontrolResponse;
// pub type FlowReservationResponseResponse = FlowReservationResponseResponse;
// pub type AppliedTargetReduction = AppliedTargetReduction;
// pub type DrResponse = DrResponse;
// pub type PriceResponse = PriceResponse;
// pub type Response = Response;
// pub type ResponseList = ResponseList;
// pub type ResponseSet = ResponseSet;
// pub type ResponseSetList = ResponseSetList;
// pub type TextResponse = TextResponse;
// pub type Time = Time;
// pub type DeviceInformation = DeviceInformation;
// pub type Drlccapabilities = Drlccapabilities;
// pub type SupportedLocale = SupportedLocale;
// pub type SupportedLocaleList = SupportedLocaleList;
// pub type PowerStatus = PowerStatus;
// pub type PowerSourceType = PowerSourceType;
// pub type Pevinfo = Pevinfo;
// pub type Ieee802154 = Ieee802154;
// pub type Ipaddr = Ipaddr;
// pub type IpaddrList = IpaddrList;
// pub type Ipinterface = Ipinterface;
// pub type IpinterfaceList = IpinterfaceList;
// pub type Llinterface = Llinterface;
// pub type LlinterfaceList = LlinterfaceList;
// pub type LoWPAN = LoWPAN;
// pub type Neighbor = Neighbor;
// pub type NeighborList = NeighborList;
// pub type Rplinstance = Rplinstance;
// pub type RplinstanceList = RplinstanceList;
// pub type RplsourceRoutes = RplsourceRoutes;
// pub type RplsourceRoutesList = RplsourceRoutesList;
// pub type LogEvent = LogEvent;
// pub type LogEventList = LogEventList;
// pub type Configuration = Configuration;
// pub type PowerConfiguration = PowerConfiguration;
// pub type PriceResponseCfg = PriceResponseCfg;
// pub type PriceResponseCfgList = PriceResponseCfgList;
// pub type TimeConfiguration = TimeConfiguration;
// pub type File = File;
// pub type FileList = FileList;
// pub type FileStatus = FileStatus;
// pub type LoadShedAvailabilityList = LoadShedAvailabilityList;
// pub type ApplianceLoadReduction = ApplianceLoadReduction;
// pub type DemandResponseProgram = DemandResponseProgram;
// pub type DemandResponseProgramList = DemandResponseProgramList;
// pub type DutyCycle = DutyCycle;
// pub type EndDeviceControl = EndDeviceControl;
// pub type EndDeviceControlList = EndDeviceControlList;
// pub type LoadShedAvailability = LoadShedAvailability;
// pub type Offset = Offset;
// pub type SetPoint = SetPoint;
// pub type TargetReduction = TargetReduction;
// pub type MeterReading = MeterReading;
// pub type MeterReadingList = MeterReadingList;
// pub type Reading = Reading;
// pub type ReadingList = ReadingList;
// pub type ReadingSet = ReadingSet;
// pub type ReadingSetList = ReadingSetList;
// pub type ReadingType = ReadingType;
// pub type UsagePoint = UsagePoint;
// pub type UsagePointList = UsagePointList;
// pub type ConsumptionTariffInterval = ConsumptionTariffInterval;
// pub type ConsumptionTariffIntervalList = ConsumptionTariffIntervalList;
// pub type CostKindType = CostKindType;
// pub type EnvironmentalCost = EnvironmentalCost;
// pub type RateComponent = RateComponent;
// pub type RateComponentList = RateComponentList;
// pub type TariffProfile = TariffProfile;
// pub type TariffProfileList = TariffProfileList;
// pub type TimeTariffInterval = TimeTariffInterval;
// pub type TimeTariffIntervalList = TimeTariffIntervalList;
// pub type MessagingProgram = MessagingProgram;
// pub type MessagingProgramList = MessagingProgramList;
// pub type PriorityType = PriorityType;
// pub type TextMessage = TextMessage;
// pub type TextMessageList = TextMessageList;
// pub type BillingPeriod = BillingPeriod;
// pub type BillingPeriodList = BillingPeriodList;
// pub type BillingMeterReadingBase = BillingMeterReadingBase;
// pub type BillingReading = BillingReading;
// pub type BillingReadingList = BillingReadingList;
// pub type BillingReadingSet = BillingReadingSet;
// pub type BillingReadingSetList = BillingReadingSetList;
// pub type Charge = Charge;
// pub type ChargeKind = ChargeKind;
// pub type CustomerAccount = CustomerAccount;
// pub type CustomerAccountList = CustomerAccountList;
// pub type CustomerAgreement = CustomerAgreement;
// pub type CustomerAgreementList = CustomerAgreementList;
// pub type HistoricalReading = HistoricalReading;
// pub type HistoricalReadingList = HistoricalReadingList;
// pub type ProjectionReading = ProjectionReading;
// pub type ProjectionReadingList = ProjectionReadingList;
// pub type TargetReading = TargetReading;
// pub type TargetReadingList = TargetReadingList;
// pub type ServiceSupplier = ServiceSupplier;
// pub type ServiceSupplierList = ServiceSupplierList;
// pub type AccountBalance = AccountBalance;
// pub type AccountingUnit = AccountingUnit;
// pub type CreditRegister = CreditRegister;
// pub type CreditRegisterList = CreditRegisterList;
// pub type Prepayment = Prepayment;
// pub type PrepaymentList = PrepaymentList;
// pub type PrepayModeType = PrepayModeType;
// pub type PrepayOperationStatus = PrepayOperationStatus;
// pub type ServiceChange = ServiceChange;
// pub type SupplyInterruptionOverride = SupplyInterruptionOverride;
// pub type SupplyInterruptionOverrideList = SupplyInterruptionOverrideList;
// pub type CreditStatusType = CreditStatusType;
// pub type CreditTypeType = CreditTypeType;
// pub type CreditTypeChange = CreditTypeChange;
// pub type ServiceStatusType = ServiceStatusType;
// pub type RequestStatus = RequestStatus;
// pub type FlowReservationRequest = FlowReservationRequest;
// pub type FlowReservationRequestList = FlowReservationRequestList;
// pub type FlowReservationResponse = FlowReservationResponse;
// pub type FlowReservationResponseList = FlowReservationResponseList;
// pub type DefaultDERControl = DefaultDERControl;
// pub type FreqDroopType = FreqDroopType;
// pub type Der = Der;
// pub type Derlist = Derlist;
// pub type Dersettings = Dersettings;
// pub type Dertype = Dertype;
// pub type Deravailability = Deravailability;
// pub type Dercapability = Dercapability;
// pub type DercontrolBase = DercontrolBase;
// pub type Dercontrol = Dercontrol;
// pub type DercontrolList = DercontrolList;
// pub type DercontrolType = DercontrolType;
// pub type Dercurve = Dercurve;
// pub type CurrentDERProgramLink = CurrentDERProgramLink;
// pub type DercurveList = DercurveList;
// pub type CurveData = CurveData;
// pub type DercurveType = DercurveType;
// pub type Derprogram = Derprogram;
// pub type DerprogramList = DerprogramList;
// pub type Derstatus = Derstatus;
// pub type DerunitRefType = DerunitRefType;
// pub type CurrentRMS = CurrentRMS;
// pub type FixedPointType = FixedPointType;
// pub type UnsignedFixedPointType = UnsignedFixedPointType;
// pub type ActivePower = ActivePower;
// pub type AmpereHour = AmpereHour;
// pub type ApparentPower = ApparentPower;
// pub type ReactivePower = ReactivePower;
// pub type ReactiveSusceptance = ReactiveSusceptance;
// pub type PowerFactor = PowerFactor;
// pub type PowerFactorWithExcitation = PowerFactorWithExcitation;
// pub type FixedVar = FixedVar;
// pub type WattHour = WattHour;
// pub type VoltageRMS = VoltageRMS;
// pub type ConnectStatusType = ConnectStatusType;
// pub type InverterStatusType = InverterStatusType;
// pub type LocalControlModeStatusType = LocalControlModeStatusType;
// pub type ManufacturerStatusType = ManufacturerStatusType;
// pub type OperationalModeStatusType = OperationalModeStatusType;
// pub type StateOfChargeStatusType = StateOfChargeStatusType;
// pub type StorageModeStatusType = StorageModeStatusType;
// pub type AccountBalanceLink = AccountBalanceLink;
// pub type ActiveBillingPeriodListLink = ActiveBillingPeriodListLink;
// pub type ActiveCreditRegisterListLink = ActiveCreditRegisterListLink;
// pub type ActiveDERControlListLink = ActiveDERControlListLink;
// pub type ActiveEndDeviceControlListLink = ActiveEndDeviceControlListLink;
// pub type ActiveFlowReservationListLink = ActiveFlowReservationListLink;
// pub type ActiveProjectionReadingListLink = ActiveProjectionReadingListLink;
// pub type ActiveSupplyInterruptionOverrideListLink = ActiveSupplyInterruptionOverrideListLink;
// pub type ActiveTargetReadingListLink = ActiveTargetReadingListLink;
// pub type ActiveTextMessageListLink = ActiveTextMessageListLink;
// pub type ActiveTimeTariffIntervalListLink = ActiveTimeTariffIntervalListLink;
// pub type AssociatedDERProgramListLink = AssociatedDERProgramListLink;
// pub type AssociatedUsagePointLink = AssociatedUsagePointLink;
// pub type BillingPeriodListLink = BillingPeriodListLink;
// pub type BillingReadingListLink = BillingReadingListLink;
// pub type BillingReadingSetListLink = BillingReadingSetListLink;
// pub type ConfigurationLink = ConfigurationLink;
// pub type ConsumptionTariffIntervalListLink = ConsumptionTariffIntervalListLink;
// pub type CreditRegisterListLink = CreditRegisterListLink;
// pub type CustomerAccountLink = CustomerAccountLink;
// pub type CustomerAccountListLink = CustomerAccountListLink;
// pub type CustomerAgreementListLink = CustomerAgreementListLink;
// pub type DemandResponseProgramLink = DemandResponseProgramLink;
// pub type DemandResponseProgramListLink = DemandResponseProgramListLink;
// pub type DeravailabilityLink = DeravailabilityLink;
// pub type DefaultDERControlLink = DefaultDERControlLink;
// pub type DercapabilityLink = DercapabilityLink;
// pub type DercontrolListLink = DercontrolListLink;
// pub type DercurveLink = DercurveLink;
// pub type DercurveListLink = DercurveListLink;
// pub type Derlink = Derlink;
// pub type DerlistLink = DerlistLink;
// pub type DerprogramLink = DerprogramLink;
// pub type DerprogramListLink = DerprogramListLink;
// pub type DersettingsLink = DersettingsLink;
// pub type DerstatusLink = DerstatusLink;
// pub type DeviceCapabilityLink = DeviceCapabilityLink;
// pub type DeviceInformationLink = DeviceInformationLink;
// pub type DeviceStatusLink = DeviceStatusLink;
// pub type EndDeviceControlListLink = EndDeviceControlListLink;
// pub type EndDeviceLink = EndDeviceLink;
// pub type EndDeviceListLink = EndDeviceListLink;
// pub type FileLink = FileLink;
// pub type FileListLink = FileListLink;
// pub type FileStatusLink = FileStatusLink;
// pub type FlowReservationRequestListLink = FlowReservationRequestListLink;
// pub type FlowReservationResponseListLink = FlowReservationResponseListLink;
// pub type FunctionSetAssignmentsListLink = FunctionSetAssignmentsListLink;
// pub type HistoricalReadingListLink = HistoricalReadingListLink;
// pub type IpaddrListLink = IpaddrListLink;
// pub type IpinterfaceListLink = IpinterfaceListLink;
// pub type LlinterfaceListLink = LlinterfaceListLink;
// pub type LoadShedAvailabilityListLink = LoadShedAvailabilityListLink;
// pub type LogEventListLink = LogEventListLink;
// pub type MessagingProgramListLink = MessagingProgramListLink;
// pub type MeterReadingLink = MeterReadingLink;
// pub type MeterReadingListLink = MeterReadingListLink;
// pub type MirrorUsagePointListLink = MirrorUsagePointListLink;
// pub type NeighborListLink = NeighborListLink;
// pub type NotificationListLink = NotificationListLink;
// pub type PowerStatusLink = PowerStatusLink;
// pub type PrepaymentLink = PrepaymentLink;
// pub type PrepaymentListLink = PrepaymentListLink;
// pub type PrepayOperationStatusLink = PrepayOperationStatusLink;
// pub type PriceResponseCfgListLink = PriceResponseCfgListLink;
// pub type ProjectionReadingListLink = ProjectionReadingListLink;
// pub type RateComponentLink = RateComponentLink;
// pub type RateComponentListLink = RateComponentListLink;
// pub type ReadingLink = ReadingLink;
// pub type ReadingListLink = ReadingListLink;
// pub type ReadingSetListLink = ReadingSetListLink;
// pub type ReadingTypeLink = ReadingTypeLink;
// pub type RegistrationLink = RegistrationLink;
// pub type ResponseListLink = ResponseListLink;
// pub type ResponseSetListLink = ResponseSetListLink;
// pub type RplinstanceListLink = RplinstanceListLink;
// pub type RplsourceRoutesListLink = RplsourceRoutesListLink;
// pub type SelfDeviceLink = SelfDeviceLink;
// pub type ServiceSupplierLink = ServiceSupplierLink;
// pub type SubscriptionListLink = SubscriptionListLink;
// pub type SupplyInterruptionOverrideListLink = SupplyInterruptionOverrideListLink;
// pub type SupportedLocaleListLink = SupportedLocaleListLink;
// pub type TargetReadingListLink = TargetReadingListLink;
// pub type TariffProfileLink = TariffProfileLink;
// pub type TariffProfileListLink = TariffProfileListLink;
// pub type TextMessageListLink = TextMessageListLink;
// pub type TimeLink = TimeLink;
// pub type TimeTariffIntervalListLink = TimeTariffIntervalListLink;
// pub type UsagePointLink = UsagePointLink;
// pub type UsagePointListLink = UsagePointListLink;
// pub type IdentifiedObject = IdentifiedObject;
// pub type Link = Link;
// pub type List = List;
// pub type ListLink = ListLink;
// pub type Resource = Resource;
// pub type RespondableIdentifiedObject = RespondableIdentifiedObject;
// pub type RespondableResource = RespondableResource;
// pub type RespondableSubscribableIdentifiedObject = RespondableSubscribableIdentifiedObject;
// pub type SubscribableIdentifiedObject = SubscribableIdentifiedObject;
// pub type SubscribableList = SubscribableList;
// pub type SubscribableResource = SubscribableResource;
// pub type Error = Error;
// pub type Event = Event;
// pub type EventStatus = EventStatus;
// pub type RandomizableEvent = RandomizableEvent;
// pub type AccumulationBehaviourType = AccumulationBehaviourType;
// pub type ApplianceLoadReductionType = ApplianceLoadReductionType;
// pub type CommodityType = CommodityType;
// pub type ConsumptionBlockType = ConsumptionBlockType;
// pub type CurrencyCode = CurrencyCode;
// pub type DataQualifierType = DataQualifierType;
// pub type DateTimeInterval = DateTimeInterval;
// pub type DeviceCategoryType = DeviceCategoryType;
// pub type DstRuleType = DstRuleType;
// pub type FlowDirectionType = FlowDirectionType;
// pub type GpslocationType = GpslocationType;
// pub type KindType = KindType;
// pub type LocaleType = LocaleType;
// pub type Mridtype = Mridtype;
// pub type OneHourRangeType = OneHourRangeType;
// pub type Pentype = Pentype;
// pub type PerCent = PerCent;
// pub type PhaseCode = PhaseCode;
// pub type Pintype = Pintype;
// pub type PowerOfTenMultiplierType = PowerOfTenMultiplierType;
// pub type PrimacyType = PrimacyType;
// pub type RealEnergy = RealEnergy;
// pub type RoleFlagsType = RoleFlagsType;
// pub type ServiceKind = ServiceKind;
// pub type Sfditype = Sfditype;
// pub type SignedPerCent = SignedPerCent;
// pub type SignedRealEnergy = SignedRealEnergy;
// pub type TimeOffsetType = TimeOffsetType;
// pub type TimeType = TimeType;
// pub type Toutype = Toutype;
// pub type UnitType = UnitType;
// pub type UnitValueType = UnitValueType;
// pub type UomType = UomType;
// pub type VersionType = VersionType;
// pub type MirrorMeterReading = MirrorMeterReading;
// pub type MirrorMeterReadingList = MirrorMeterReadingList;
// pub type MeterReadingBase = MeterReadingBase;
// pub type MirrorReadingSet = MirrorReadingSet;
// pub type MirrorUsagePoint = MirrorUsagePoint;
// pub type MirrorUsagePointList = MirrorUsagePointList;
// pub type ReadingBase = ReadingBase;
// pub type ReadingSetBase = ReadingSetBase;
// pub type UsagePointBase = UsagePointBase;
