#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use serde_json::value::Value;



/// Details and position information for a physical place where services are
/// provided and resources and participants may be stored, found, contained, or
/// accommodated.

#[derive(Debug)]
pub struct Location_Position<'a> {
  pub value: &'a Value,
}

impl Location_Position<'_> {
  /// Extensions for longitude
  pub fn _longitude(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_longitude") {
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

  /// Longitude. The value domain and the interpretation are the same as for the text
  /// of the longitude element in KML (see notes below).
  pub fn longitude(&self) -> Option<f64> {
    if let Some(val) = self.value.get("longitude") {
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

  /// Altitude. The value domain and the interpretation are the same as for the text
  /// of the altitude element in KML (see notes below).
  pub fn altitude(&self) -> Option<f64> {
    if let Some(val) = self.value.get("altitude") {
      return Some(val.as_f64().unwrap());
    }
    return None;
  }

  /// Latitude. The value domain and the interpretation are the same as for the text
  /// of the latitude element in KML (see notes below).
  pub fn latitude(&self) -> Option<f64> {
    if let Some(val) = self.value.get("latitude") {
      return Some(val.as_f64().unwrap());
    }
    return None;
  }

  /// Extensions for altitude
  pub fn _altitude(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_altitude") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for latitude
  pub fn _latitude(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_latitude") {
      return Some(Element { value: val });
    }
    return None;
  }

}