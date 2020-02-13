#![allow(unused_imports, non_camel_case_types)]

use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;



/// A collection of documents compiled for a purpose together with metadata that
/// applies to the collection.

#[derive(Debug)]
pub struct DocumentManifest_Related<'a> {
  pub value: &'a Value,
}

impl DocumentManifest_Related<'_> {
  /// Related identifier to this DocumentManifest.  For example, Order numbers,
  /// accession numbers, XDW workflow numbers.
  pub fn identifier(&self) -> Option<Identifier> {
    if let Some(val) = self.value.get("identifier") {
      return Some(Identifier { value: val });
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

  /// Related Resource to this DocumentManifest. For example, Order, ServiceRequest,
  /// Procedure, EligibilityRequest, etc.
  pub fn fhir_ref(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("ref") {
      return Some(Reference { value: val });
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