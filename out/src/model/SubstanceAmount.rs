#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Quantity::Quantity;
use crate::model::Extension::Extension;
use crate::model::SubstanceAmount_ReferenceRange::SubstanceAmount_ReferenceRange;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Range::Range;
use crate::model::Element::Element;


/// Chemical substances are a single substance type whose primary defining element
/// is the molecular structure. Chemical substances shall be defined on the basis of
/// their complete covalent molecular structure; the presence of a salt (counter-
/// ion) and/or solvates (water, alcohols) is also captured. Purity, grade,
/// physical form or particle size are not taken into account in the definition of a
/// chemical substance or in the assignment of a Substance ID.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceAmount {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Most elements that require a quantitative value will also have a field called
  /// amount type. Amount type should always be specified because the actual value of
  /// the amount is often dependent on it. EXAMPLE: In capturing the actual relative
  /// amounts of substances or molecular fragments it is essential to indicate whether
  /// the amount refers to a mole ratio or weight ratio. For any given element an
  /// effort should be made to use same the amount type for all related definitional
  /// elements.
  #[serde(rename = "amountType")]
  amount_type: Option<CodeableConcept>,

  /// A textual comment on a numeric value.
  #[serde(rename = "amountText")]
  amount_text: Option<String>,

  /// Reference range of possible or expected values.
  #[serde(rename = "referenceRange")]
  reference_range: Option<SubstanceAmount_ReferenceRange>,

  /// Used to capture quantitative values for a variety of elements. If only limits
  /// are given, the arithmetic mean would be the average. If only a single definite
  /// value for a given element is given, it would be captured in this field.
  #[serde(rename = "amountRange")]
  amount_range: Option<Range>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for amountString
  #[serde(rename = "_amountString")]
  _amount_string: Option<Element>,

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

  /// Extensions for amountText
  #[serde(rename = "_amountText")]
  _amount_text: Option<Element>,

  /// Used to capture quantitative values for a variety of elements. If only limits
  /// are given, the arithmetic mean would be the average. If only a single definite
  /// value for a given element is given, it would be captured in this field.
  #[serde(rename = "amountString")]
  amount_string: Option<String>,

  /// Used to capture quantitative values for a variety of elements. If only limits
  /// are given, the arithmetic mean would be the average. If only a single definite
  /// value for a given element is given, it would be captured in this field.
  #[serde(rename = "amountQuantity")]
  amount_quantity: Option<Quantity>,

}