#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Measure_Component::Measure_Component;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Expression::Expression;
use serde_json::value::Value;



/// The Measure resource provides the definition of a quality measure.

#[derive(Debug)]
pub struct Measure_Stratifier<'a> {
  pub value: &'a Value,
}

impl Measure_Stratifier<'_> {
  /// An expression that specifies the criteria for the stratifier. This is typically
  /// the name of an expression defined within a referenced library, but it may also
  /// be a path to a stratifier element.
  pub fn criteria(&self) -> Option<Expression> {
    if let Some(val) = self.value.get("criteria") {
      return Some(Expression { value: val });
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

  /// The human readable description of this stratifier criteria.
  pub fn description(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("description") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for description
  pub fn _description(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_description") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A component of the stratifier criteria for the measure report, specified as
  /// either the name of a valid CQL expression defined within a referenced library or
  /// a valid FHIR Resource Path.
  pub fn component(&self) -> Option<Vec<Measure_Component>> {
    if let Some(Value::Array(val)) = self.value.get("component") {
      return Some(val.into_iter().map(|e| Measure_Component { value: e }).collect::<Vec<_>>());
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

  /// Indicates a meaning for the stratifier. This can be as simple as a unique
  /// identifier, or it can establish meaning in a broader context by drawing from a
  /// terminology, allowing stratifiers to be correlated across measures.
  pub fn code(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("code") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

}