#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Period::Period;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;



/// Represents a defined collection of entities that may be discussed or acted upon
/// collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.

#[derive(Debug)]
pub struct Group_Member<'a> {
  pub value: &'a Value,
}

impl Group_Member<'_> {
  /// Extensions for inactive
  pub fn _inactive(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_inactive") {
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

  /// A reference to the entity that is a member of the group. Must be consistent with
  /// Group.type. If the entity is another group, then the type must be the same.
  pub fn entity(&self) -> Reference {
    Reference {
      value: &self.value["entity"],
    }
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

  /// The period that the member was in the group, if known.
  pub fn period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("period") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// A flag to indicate that the member is no longer in the group, but previously may
  /// have been a member.
  pub fn inactive(&self) -> Option<bool> {
    if let Some(val) = self.value.get("inactive") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

}