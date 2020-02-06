#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;


/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationGuide_Page {
  /// Extensions for generation
  #[serde(rename = "_generation")]
  _generation: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The source address for the page.
  #[serde(rename = "nameUrl")]
  name_url: Option<String>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// A code that indicates how the page is generated.
  generation: Option<ImplementationGuide_PageGeneration>,

  /// A short title used to represent this page in navigational structures such as
  /// table of contents, bread crumbs, etc.
  title: Option<String>,

  /// Extensions for nameUrl
  #[serde(rename = "_nameUrl")]
  _name_url: Option<Element>,

  /// Nested Pages/Sections under this page.
  page: Option<Vec<ImplementationGuide_Page>>,

  /// The source address for the page.
  #[serde(rename = "nameReference")]
  name_reference: Option<Box<Reference>>,

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

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImplementationGuide_PageGeneration {
  #[serde(rename = "html")]
  Html,

  #[serde(rename = "markdown")]
  Markdown,

  #[serde(rename = "xml")]
  Xml,

  #[serde(rename = "generated")]
  Generated,

}