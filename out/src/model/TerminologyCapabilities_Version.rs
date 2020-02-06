#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::TerminologyCapabilities_Filter::TerminologyCapabilities_Filter;
use crate::model::Element::Element;


/// A TerminologyCapabilities resource documents a set of capabilities (behaviors)
/// of a FHIR Terminology Server that may be used as a statement of actual server
/// functionality or a statement of required or desired server implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilities_Version {
  /// If this is the default version for this code system.
  #[serde(rename = "isDefault")]
  is_default: Option<bool>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Vec<Element>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for property
  #[serde(rename = "_property")]
  _property: Option<Vec<Element>>,

  /// If the compositional grammar defined by the code system is supported.
  compositional: Option<bool>,

  /// Filter Properties supported.
  filter: Option<Vec<TerminologyCapabilities_Filter>>,

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

  /// Extensions for isDefault
  #[serde(rename = "_isDefault")]
  _is_default: Option<Element>,

  /// Properties supported for $lookup.
  property: Option<Vec<String>>,

  /// Extensions for compositional
  #[serde(rename = "_compositional")]
  _compositional: Option<Element>,

  /// For version-less code systems, there should be a single version with no
  /// identifier.
  code: Option<String>,

  /// Language Displays supported.
  language: Option<Vec<String>>,

  /// Extensions for code
  #[serde(rename = "_code")]
  _code: Option<Element>,

}