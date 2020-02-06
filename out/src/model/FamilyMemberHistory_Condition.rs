#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Annotation::Annotation;
use crate::model::Period::Period;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Range::Range;
use crate::model::Age::Age;


/// Significant health conditions for a person related to the patient relevant in
/// the context of care for the patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMemberHistory_Condition {
  /// An area where general notes can be placed about this specific condition.
  note: Option<Vec<Annotation>>,

  /// Extensions for contributedToDeath
  #[serde(rename = "_contributedToDeath")]
  _contributed_to_death: Option<Element>,

  /// Either the age of onset, range of approximate age or descriptive string can be
  /// recorded.  For conditions with multiple occurrences, this describes the first
  /// known occurrence.
  #[serde(rename = "onsetPeriod")]
  onset_period: Option<Period>,

  /// Either the age of onset, range of approximate age or descriptive string can be
  /// recorded.  For conditions with multiple occurrences, this describes the first
  /// known occurrence.
  #[serde(rename = "onsetString")]
  onset_string: Option<String>,

  /// Either the age of onset, range of approximate age or descriptive string can be
  /// recorded.  For conditions with multiple occurrences, this describes the first
  /// known occurrence.
  #[serde(rename = "onsetRange")]
  onset_range: Option<Range>,

  /// Extensions for onsetString
  #[serde(rename = "_onsetString")]
  _onset_string: Option<Element>,

  /// Either the age of onset, range of approximate age or descriptive string can be
  /// recorded.  For conditions with multiple occurrences, this describes the first
  /// known occurrence.
  #[serde(rename = "onsetAge")]
  onset_age: Option<Age>,

  /// Indicates what happened following the condition.  If the condition resulted in
  /// death, deceased date is captured on the relation.
  outcome: Option<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

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

  /// This condition contributed to the cause of death of the related person. If
  /// contributedToDeath is not populated, then it is unknown.
  #[serde(rename = "contributedToDeath")]
  contributed_to_death: Option<bool>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The actual condition specified. Could be a coded condition (like MI or Diabetes)
  /// or a less specific string like 'cancer' depending on how much is known about the
  /// condition and the capabilities of the creating system.
  code: CodeableConcept,

}