#![allow(unused_imports, non_camel_case_types)]

use crate::model::Quantity::Quantity;
use crate::model::Money::Money;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::ClaimResponse_Adjudication::ClaimResponse_Adjudication;
use serde_json::value::Value;



/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse_SubDetail1<'a> {
  pub value: &'a Value,
}

impl ClaimResponse_SubDetail1<'_> {
  /// When the value is a group code then this item collects a set of related claim
  /// details, otherwise this contains the product, service, drug or other billing
  /// code for the item.
  pub fn product_or_service(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["productOrService"],
    }
  }

  /// The number of repetitions of a service or product.
  pub fn quantity(&self) -> Option<Quantity> {
    if let Some(val) = self.value.get("quantity") {
      return Some(Quantity { value: val });
    }
    return None;
  }

  /// If the item is not a group then this is the fee for the product or service,
  /// otherwise this is the total of the fees for the details of the group.
  pub fn unit_price(&self) -> Option<Money> {
    if let Some(val) = self.value.get("unitPrice") {
      return Some(Money { value: val });
    }
    return None;
  }

  /// A real number that represents a multiplier used in determining the overall value
  /// of services delivered and/or goods received. The concept of a Factor allows for
  /// a discount or surcharge multiplier to be applied to a monetary amount.
  pub fn factor(&self) -> Option<f64> {
    if let Some(val) = self.value.get("factor") {
      return Some(val.as_f64().unwrap());
    }
    return None;
  }

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
  pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The numbers associated with notes below which apply to the adjudication of this
  /// item.
  pub fn note_number(&self) -> Option<Vec<i64>> {
    if let Some(Value::Array(val)) = self.value.get("noteNumber") {
      return Some(val.into_iter().map(|e| e.as_i64().unwrap()).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for noteNumber
  pub fn _note_number(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_noteNumber") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string);
    }
    return None;
  }

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  pub fn modifier(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("modifier") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for factor
  pub fn _factor(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_factor") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  pub fn extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("extension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The quantity times the unit price for an additional service or product or
  /// charge.
  pub fn net(&self) -> Option<Money> {
    if let Some(val) = self.value.get("net") {
      return Some(Money { value: val });
    }
    return None;
  }

  /// The adjudication results.
  pub fn adjudication(&self) -> Vec<ClaimResponse_Adjudication> {
    self.value.get("adjudication").unwrap().as_array().unwrap().into_iter().map(|e| ClaimResponse_Adjudication { value: e }).collect::<Vec<_>>()
  }

  pub fn validate(&self) -> bool {
    let _ = self.product_or_service().validate();
    if let Some(_val) = self.quantity() {
      _val.validate();
    }
    if let Some(_val) = self.unit_price() {
      _val.validate();
    }
    if let Some(_val) = self.factor() {
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.note_number() {
      _val.into_iter().for_each(|_e| {});
    }
    if let Some(_val) = self._note_number() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.modifier() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._factor() {
      _val.validate();
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.net() {
      _val.validate();
    }
    let _ = self.adjudication().into_iter().for_each(|e| { e.validate(); });
    return true;
  }

}
