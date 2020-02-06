#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Ratio::Ratio;
use crate::model::Element::Element;


/// This resource is primarily used for the identification and definition of a
/// medication for the purposes of prescribing, dispensing, and administering a
/// medication as well as for making statements about medication use.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Medication_Ingredient {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The actual ingredient - either a substance (simple ingredient) or another
  /// medication of a medication.
  #[serde(rename = "itemCodeableConcept")]
  item_codeable_concept: Option<CodeableConcept>,

  /// Specifies how many (or how much) of the items there are in this Medication.  For
  /// example, 250 mg per tablet.  This is expressed as a ratio where the numerator is
  /// 250mg and the denominator is 1 tablet.
  strength: Option<Ratio>,

  /// Extensions for isActive
  #[serde(rename = "_isActive")]
  _is_active: Option<Element>,

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

  /// The actual ingredient - either a substance (simple ingredient) or another
  /// medication of a medication.
  #[serde(rename = "itemReference")]
  item_reference: Option<Box<Reference>>,

  /// Indication of whether this ingredient affects the therapeutic action of the
  /// drug.
  #[serde(rename = "isActive")]
  is_active: Option<bool>,

}