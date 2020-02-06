#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Money::Money;


/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice_PriceComponent {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The factor that has been applied on the base price for calculating this
  /// component.
  factor: Option<f32>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The amount calculated for this component.
  amount: Option<Money>,

  /// A code that identifies the component. Codes may be used to differentiate between
  /// kinds of taxes, surcharges, discounts etc.
  code: Option<CodeableConcept>,

  /// Extensions for factor
  #[serde(rename = "_factor")]
  _factor: Option<Element>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

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

  /// This code identifies the type of the component.
  #[serde(rename = "type")]
  fhir_type: Option<Invoice_PriceComponentType>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Invoice_PriceComponentType {
  #[serde(rename = "base")]
  Base,

  #[serde(rename = "surcharge")]
  Surcharge,

  #[serde(rename = "deduction")]
  Deduction,

  #[serde(rename = "discount")]
  Discount,

  #[serde(rename = "tax")]
  Tax,

  #[serde(rename = "informational")]
  Informational,

}