#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use serde_json::value::Value;



/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.

#[derive(Debug)]
pub struct CodeSystem_Property<'a> {
  pub value: &'a Value,
}

impl CodeSystem_Property<'_> {
  /// Extensions for uri
  pub fn _uri(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_uri") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for code
  pub fn _code(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_code") {
      return Some(Element { value: val });
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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A code that is used to identify the property. The code is used internally (in
  /// CodeSystem.concept.property.code) and also externally, such as in property
  /// filters.
  pub fn code(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("code") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A description of the property- why it is defined, and how its value might be
  /// used.
  pub fn description(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("description") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for type
  pub fn _type(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_type") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Reference to the formal meaning of the property. One possible source of meaning
  /// is the [Concept Properties](codesystem-concept-properties.html) code system.
  pub fn uri(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("uri") {
      return Some(string.to_string());
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

  /// The type of the property value. Properties of type "code" contain a code defined
  /// by the code system (e.g. a reference to another defined concept).
  pub fn fhir_type(&self) -> Option<CodeSystem_PropertyType> {
    if let Some(Value::String(val)) = self.value.get("type") {
      return Some(CodeSystem_PropertyType::from_string(&val).unwrap());
    }
    return None;
  }

}

#[derive(Debug)]
pub enum CodeSystem_PropertyType {
  Code,
  Coding,
  String,
  Integer,
  Boolean,
  DateTime,
  Decimal,
}

impl CodeSystem_PropertyType {
    pub fn from_string(string: &str) -> Option<CodeSystem_PropertyType> {
      match string {
        "code" => Some(CodeSystem_PropertyType::Code),
        "Coding" => Some(CodeSystem_PropertyType::Coding),
        "string" => Some(CodeSystem_PropertyType::String),
        "integer" => Some(CodeSystem_PropertyType::Integer),
        "boolean" => Some(CodeSystem_PropertyType::Boolean),
        "dateTime" => Some(CodeSystem_PropertyType::DateTime),
        "decimal" => Some(CodeSystem_PropertyType::Decimal),
        _ => None,
    }
  }
}
