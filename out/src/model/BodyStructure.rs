#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::ResourceList::ResourceList;


/// Record details about an anatomical structure.  This resource may be used when a
/// coded concept does not provide the necessary detail needed for the use case.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyStructure {
  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// A summary, characterization or explanation of the body structure.
  description: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Identifier for this instance of the anatomical structure.
  identifier: Option<Vec<Identifier>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The anatomical location or region of the specimen, lesion, or body structure.
  location: Option<CodeableConcept>,

  /// Image or images used to identify a location.
  image: Option<Vec<Attachment>>,

  /// Extensions for active
  #[serde(rename = "_active")]
  _active: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Whether this body site is in active use.
  active: Option<bool>,

  /// The kind of structure being represented by the body structure at
  /// `BodyStructure.location`.  This can define both normal and abnormal
  /// morphologies.
  morphology: Option<CodeableConcept>,

  /// Qualifier to refine the anatomical location.  These include qualifiers for
  /// laterality, relative location, directionality, number, and plane.
  #[serde(rename = "locationQualifier")]
  location_qualifier: Option<Vec<CodeableConcept>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Extension>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The person to which the body site belongs.
  patient: Box<Reference>,

}