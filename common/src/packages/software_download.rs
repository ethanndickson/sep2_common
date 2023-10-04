use crate::traits::{SEList, SEResource, Validate};
use sep2_common_derive::{SEList, SEResource};

use super::{
    links::FileLink,
    primitives::{HexBinary16, HexBinary160, String16, String32, Uint16, Uint32, Uint8},
    types::{PENType, TimeType},
};
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "File")]
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
    pub lfdi: Option<HexBinary160>,

    // This element MUST be set to the hardware version for which this file is
    // targeted.
    #[yaserde(rename = "mfHwVer")]
    pub mf_hw_ver: Option<String32>,

    // This element MUST be set to the manufacturer's Private Enterprise Number
    // (assigned by IANA).
    #[yaserde(rename = "mfID")]
    pub mf_id: PENType,

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

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary Key - mfID (ascending)
        match self.mf_id.cmp(&other.mf_id) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Secondary Key - mfModel (ascending)
        match self.mf_model.cmp(&other.mf_model) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Tertiary Key -- mfVer (descending)
        match self.mf_ver.cmp(&other.mf_ver).reverse() {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Quaternary Key - href (ascending)
        self.href.cmp(&other.href)
    }
}

impl Validate for File {}

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEList, SEResource)]
#[yaserde(rename = "FileList")]
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

#[derive(Default, PartialEq, Eq, Debug, Clone, YaSerialize, YaDeserialize, SEResource)]
#[yaserde(rename = "FileStatus")]
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
    pub status: FileStatusType,

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

#[derive(
    Default, PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, YaSerialize, YaDeserialize,
)]
#[yaserde(namespace = "urn:ieee:std:2030.5:ns")]
#[repr(u8)]
pub enum FileStatusType {
    // No load operation in progress
    #[default]
    NotLoading = 0,
    // File load in progress (first request for file content has been issued
    // by LD)
    LoadInProgress = 1,
    // File load failed
    LoadFailed = 2,
    // File loaded successfully (full content of file has been received by
    // the LD), signature verification in progress
    LoadSuccessful = 3,
    // File signature verification failed
    VerificationFailed = 4,
    // File signature verified, waiting to activate file.
    VerficaitionSuccess = 5,
    // File activation failed
    ActivationFaild = 6,
    // File activation in progress
    ActivationInProgress = 7,
    // File activated successfully (this state may not be reached/persisted
    // through an image activation)
    ActivationSuccess = 8,
    // 9-255 - Reserved for future use.
}

impl Validate for FileStatus {}
