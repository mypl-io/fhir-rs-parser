#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Annotation::Annotation;
use crate::model::EffectEvidenceSynthesis_CertaintySubcomponent::EffectEvidenceSynthesis_CertaintySubcomponent;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is derived
/// from a combination of research studies.

#[derive(Debug)]
pub struct EffectEvidenceSynthesis_Certainty<'a> {
  pub value: &'a Value,
}

impl EffectEvidenceSynthesis_Certainty<'_> {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A human-readable string to clarify or explain concepts about the resource.
  pub fn note(&self) -> Option<Vec<Annotation>> {
    if let Some(Value::Array(val)) = self.value.get("note") {
      return Some(val.into_iter().map(|e| Annotation { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A description of a component of the overall certainty.
  pub fn certainty_subcomponent(&self) -> Option<Vec<EffectEvidenceSynthesis_CertaintySubcomponent>> {
    if let Some(Value::Array(val)) = self.value.get("certaintySubcomponent") {
      return Some(val.into_iter().map(|e| EffectEvidenceSynthesis_CertaintySubcomponent { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A rating of the certainty of the effect estimate.
  pub fn rating(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("rating") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
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

}