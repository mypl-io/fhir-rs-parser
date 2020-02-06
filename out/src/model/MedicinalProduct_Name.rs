#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::MedicinalProduct_CountryLanguage::MedicinalProduct_CountryLanguage;
use crate::model::Element::Element;
use crate::model::MedicinalProduct_NamePart::MedicinalProduct_NamePart;
use crate::model::Extension::Extension;


/// Detailed definition of a medicinal product, typically for uses other than direct
/// patient care (e.g. regulatory use).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProduct_Name {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

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

  /// Extensions for productName
  #[serde(rename = "_productName")]
  _product_name: Option<Element>,

  /// Country where the name applies.
  #[serde(rename = "countryLanguage")]
  country_language: Option<Vec<MedicinalProduct_CountryLanguage>>,

  /// The full product name.
  #[serde(rename = "productName")]
  product_name: Option<String>,

  /// Coding words or phrases of the name.
  #[serde(rename = "namePart")]
  name_part: Option<Vec<MedicinalProduct_NamePart>>,

}