// File auto-generated using xsd-parser-rs & IEEE 2030.5 sep-ordered-dep.xsd
use std::borrow::Cow;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::XmlEvent as XmlEventR;
use xml::writer::XmlEvent as XmlEventW;
use xml::EventReader;
use xsd_parser::generator::validator::Validate;
use yaserde::ser::Serializer;
use yaserde::YaDeserialize;
use yaserde::YaSerialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::config::INNER_CFG;
// TODO Ethan: Temporary import all
use crate::packages::primitives::*;

use super::traits::*;

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

impl SEResource for SubscriptionBase {}
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

impl SESubscriptionBase for Subscription {}
impl SEResource for Subscription {}
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

impl SEList for SubscriptionList {}
impl SEResource for SubscriptionList {}
impl Validate for SubscriptionList {}

#[derive(Default, PartialEq, Debug)]
pub struct Notification<T: SEResource> {
    // The new location of the resource, if moved. This attribute SHALL be a
    // fully-qualified absolute URI, not a relative reference.
    // #[yaserde(rename = "newResourceURI")]
    pub new_resource_uri: Option<String>,

    // #[yaserde(rename = "Resource")]
    pub resource: Option<T>,

    // 0 = Default Status
    // 1 = Subscription canceled, no additional information
    // 2 = Subscription canceled, resource moved
    // 3 = Subscription canceled, resource definition changed (e.g., a new
    // version of IEEE 2030.5)
    // 4 = Subscription canceled, resource deleted
    // All other values reserved.
    // #[yaserde(rename = "status")]
    pub status: Uint8,

    // The subscription from which this notification was triggered. This
    // attribute SHALL be a fully-qualified absolute URI, not a relative
    // reference.
    // #[yaserde(rename = "subscriptionURI")]
    pub subscription_uri: String,

    // The resource for which the subscription applies. Query string parameters
    // SHALL NOT be specified when subscribing to list resources. Should a query
    // string parameter be specified, servers SHALL ignore them.
    // #[yaserde(rename = "subscribedResource")]
    pub subscribed_resource: String,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    // #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl<T: SEResource> YaSerialize for Notification<T> {
    fn serialize<W: std::io::Write>(&self, writer: &mut Serializer<W>) -> Result<(), String> {
        let skip = writer.skip_start_end();

        if !skip {
            let child_attr = vec![];
            let child_attr_ns = Namespace::empty();

            let yaserde_label = writer
                .get_start_event_name()
                .unwrap_or_else(|| "Notification".to_string());
            let struct_start_event =
                XmlEventW::start_element(yaserde_label.as_ref()).ns("", "urn:ieee:std:2030.5:ns");
            let struct_start_event = if let Some(ref value) = self.href {
                struct_start_event.attr("href", value)
            } else {
                struct_start_event
            };
            let event: XmlEventW = struct_start_event.into();
            match event {
                XmlEventW::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    let mut attributes: Vec<OwnedAttribute> = attributes
                        .into_owned()
                        .to_vec()
                        .iter()
                        .map(|k| k.to_owned())
                        .collect();
                    attributes.extend(child_attr);
                    let all_attributes = attributes.iter().map(|ca| ca.borrow()).collect();
                    let mut all_namespaces = namespace.into_owned();
                    all_namespaces.extend(&child_attr_ns);
                    writer
                        .write(XmlEventW::StartElement {
                            name,
                            attributes: Cow::Owned(all_attributes),
                            namespace: Cow::Owned(all_namespaces),
                        })
                        .map_err(|e| e.to_string())?;
                }
                _ => unreachable!(),
            }
            if let Some(ref item) = self.new_resource_uri {
                writer
                    .write(XmlEventW::start_element("newResourceURI"))
                    .map_err(|e| e.to_string())?;
                let value = item.to_string();
                let data_event = XmlEventW::characters(&value);
                writer.write(data_event).map_err(|e| e.to_string())?;
                writer
                    .write(XmlEventW::end_element())
                    .map_err(|e| e.to_string())?;
            }
            if let Some(ref item) = &self.resource {
                genericize_resource(
                    &yaserde::ser::to_string_with_config(item, &INNER_CFG)?,
                    writer,
                )?;
            }
            let start_event = XmlEventW::start_element("subscriptionURI");
            writer.write(start_event).map_err(|e| e.to_string())?;
            let value = self.subscription_uri.to_string();
            let data_event = XmlEventW::characters(&value);
            writer.write(data_event).map_err(|e| e.to_string())?;
            writer
                .write(XmlEventW::end_element())
                .map_err(|e| e.to_string())?;

            let start_event = XmlEventW::start_element("subscribedResource");
            writer.write(start_event).map_err(|e| e.to_string())?;
            let value = self.subscribed_resource.to_string();
            let data_event = XmlEventW::characters(&value);
            writer.write(data_event).map_err(|e| e.to_string())?;
            writer
                .write(XmlEventW::end_element())
                .map_err(|e| e.to_string())?;

            writer
                .write(XmlEventW::end_element())
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn serialize_attributes(
        &self,
        mut source_attributes: Vec<OwnedAttribute>,
        mut source_namespace: Namespace,
    ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        let child_attr: Vec<OwnedAttribute> = vec![];
        let child_attr_ns = Namespace::empty();
        let struct_start_event = XmlEventW::start_element("temp").ns("", "urn:ieee:std:2030.5:ns");
        let struct_start_event = if let Some(ref value) = self.href {
            struct_start_event.attr("href", value)
        } else {
            struct_start_event
        };
        let event: XmlEventW = struct_start_event.into();

        if let XmlEventW::StartElement {
            attributes,
            namespace,
            ..
        } = event
        {
            source_namespace.extend(&namespace.into_owned());
            source_namespace.extend(&child_attr_ns);
            let a: Vec<OwnedAttribute> = attributes
                .into_owned()
                .to_vec()
                .iter()
                .map(|k| k.to_owned())
                .collect();
            source_attributes.extend(a);
            source_attributes.extend(child_attr);
            Ok((source_attributes, source_namespace))
        } else {
            unreachable!()
        }
    }
}

impl<T: SEResource> YaDeserialize for Notification<T> {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        todo!()
    }
}

impl<T: SEResource> SESubscriptionBase for Notification<T> {}
impl<T: SEResource> SEResource for Notification<T> {}
impl<T: SEResource> Validate for Notification<T> {}

#[derive(Default, PartialEq, Debug)]
pub struct NotificationList<T: SEResource> {
    // #[yaserde(rename = "Notification")]
    pub notification: Vec<Notification<T>>,

    // The number specifying "all" of the items in the list. Required on a
    // response to a GET, ignored otherwise.
    // #[yaserde(attribute, rename = "all")]
    pub all: Uint32,

    // Indicates the number of items in this page of results.
    // #[yaserde(attribute, rename = "results")]
    pub results: Uint32,

    // A reference to the resource address (URI). Required in a response to a
    // GET, ignored otherwise.
    // #[yaserde(attribute, rename = "href")]
    pub href: Option<String>,
}

impl<T: SEResource> YaSerialize for NotificationList<T> {
    fn serialize<W: std::io::Write>(&self, writer: &mut Serializer<W>) -> Result<(), String> {
        todo!()
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        todo!()
    }
}

impl<T: SEResource> YaDeserialize for NotificationList<T> {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        todo!()
    }
}

impl<T: SEResource> SEList for NotificationList<T> {}
impl<T: SEResource> SEResource for NotificationList<T> {}
impl<T: SEResource> Validate for NotificationList<T> {}

fn genericize_resource<W: std::io::Write>(
    xml: &str,
    writer: &mut Serializer<W>,
) -> Result<(), String> {
    let parser = EventReader::new(xml.as_bytes());
    let mut depth = 0;
    for event in parser {
        match event {
            Ok(XmlEventR::StartElement {
                name, attributes, ..
            }) if depth == 0 => {
                let mut new_attr = vec![OwnedAttribute {
                    name: OwnedName::local("xsi:type"),
                    value: name.local_name,
                }];
                new_attr.extend(attributes);
                let all_attr = new_attr.iter().map(|ca| ca.borrow()).collect();
                writer
                    .write(XmlEventW::StartElement {
                        name: "Resource".into(),
                        attributes: Cow::Owned(all_attr),
                        namespace: Cow::Owned(Namespace::empty()),
                    })
                    .map_err(|e| e.to_string())?;
                depth += 1;
            }
            event @ Ok(XmlEventR::StartElement { .. }) => {
                writer
                    .write(event.unwrap().as_writer_event().unwrap())
                    .map_err(|e| e.to_string())?;
                depth += 1
            }
            event @ Ok(XmlEventR::EndElement { .. }) => {
                let event = event.unwrap();
                let mut event = event.as_writer_event().unwrap();
                if depth == 1 {
                    event = XmlEventW::end_element().name("Resource").into();
                }
                writer.write(event).map_err(|e| e.to_string())?;
                depth -= 1;
            }
            Ok(XmlEventR::StartDocument { .. } | XmlEventR::EndDocument) => (),
            Ok(event) => writer
                .write(event.as_writer_event().unwrap())
                .map_err(|e| e.to_string())?,
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(())
}
