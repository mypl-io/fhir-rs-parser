#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A compartment definition that defines how resources are accessed on a server.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompartmentDefinition_Resource {
  /// The name of a resource supported by the server.
  code: Option<String>,

  /// Extensions for documentation
  #[serde(rename = "_documentation")]
  _documentation: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for param
  #[serde(rename = "_param")]
  _param: Option<Vec<Element>>,

  /// Extensions for code
  #[serde(rename = "_code")]
  _code: Option<Element>,

  /// Additional documentation about the resource and compartment.
  documentation: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Extension>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The name of a search parameter that represents the link to the compartment. More
  /// than one may be listed because a resource may be linked to a compartment in more
  /// than one way,.
  param: Option<Vec<String>>,

}