#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::DataRequirement::DataRequirement;
use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Annotation::Annotation;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;


/// A guidance response is the formal response to a guidance request, including any
/// output parameters returned by the evaluation, as well as the description of any
/// proposed actions to be taken.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuidanceResponse {
  /// Extensions for moduleCanonical
  #[serde(rename = "_moduleCanonical")]
  _module_canonical: Option<Element>,

  /// An identifier, CodeableConcept or canonical reference to the guidance that was
  /// requested.
  #[serde(rename = "moduleCodeableConcept")]
  module_codeable_concept: Option<CodeableConcept>,

  /// Indicates the reason the request was initiated. This is typically provided as a
  /// parameter to the evaluation and echoed by the service, although for some use
  /// cases, such as subscription- or event-based scenarios, it may provide an
  /// indication of the cause for the response.
  #[serde(rename = "reasonReference")]
  reason_reference: Option<Vec<Box<Reference>>>,

  /// The output parameters of the evaluation, if any. Many modules will result in the
  /// return of specific resources such as procedure or communication requests that
  /// are returned as part of the operation result. However, modules may define
  /// specific outputs that would be returned as the result of the evaluation, and
  /// these would be returned in this element.
  #[serde(rename = "outputParameters")]
  output_parameters: Option<Box<Reference>>,

  /// An identifier, CodeableConcept or canonical reference to the guidance that was
  /// requested.
  #[serde(rename = "moduleUri")]
  module_uri: Option<String>,

  /// Indicates when the guidance response was processed.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Provides a mechanism to communicate additional information about the response.
  note: Option<Vec<Annotation>>,

  /// Describes the reason for the guidance response in coded or textual form.
  #[serde(rename = "reasonCode")]
  reason_code: Option<Vec<CodeableConcept>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

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

  /// The actions, if any, produced by the evaluation of the artifact.
  result: Option<Box<Reference>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The encounter during which this response was created or to which the creation of
  /// this record is tightly associated.
  encounter: Option<Box<Reference>>,

  /// Extensions for moduleUri
  #[serde(rename = "_moduleUri")]
  _module_uri: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Option<Element>,

  /// Messages resulting from the evaluation of the artifact or artifacts. As part of
  /// evaluating the request, the engine may produce informational or warning
  /// messages. These messages will be provided by this element.
  #[serde(rename = "evaluationMessage")]
  evaluation_message: Option<Vec<Box<Reference>>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// If the evaluation could not be completed due to lack of information, or
  /// additional information would potentially result in a more accurate response,
  /// this element will a description of the data required in order to proceed with
  /// the evaluation. A subsequent request to the service should include this data.
  #[serde(rename = "dataRequirement")]
  data_requirement: Option<Vec<DataRequirement>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Allows a service to provide  unique, business identifiers for the response.
  identifier: Option<Vec<Identifier>>,

  /// An identifier, CodeableConcept or canonical reference to the guidance that was
  /// requested.
  #[serde(rename = "moduleCanonical")]
  module_canonical: Option<String>,

  /// The status of the response. If the evaluation is completed successfully, the
  /// status will indicate success. However, in order to complete the evaluation, the
  /// engine may require more information. In this case, the status will be data-
  /// required, and the response will contain a description of the additional
  /// required information. If the evaluation completed successfully, but the engine
  /// determines that a potentially more accurate response could be provided if more
  /// data was available, the status will be data-requested, and the response will
  /// contain a description of the additional requested information.
  status: Option<GuidanceResponseStatus>,

  /// Provides a reference to the device that performed the guidance.
  performer: Option<Box<Reference>>,

  /// The patient for which the request was processed.
  subject: Option<Box<Reference>>,

  /// The identifier of the request associated with this response. If an identifier
  /// was given as part of the request, it will be reproduced here to enable the
  /// requester to more easily identify the response in a multi-request scenario.
  #[serde(rename = "requestIdentifier")]
  request_identifier: Option<Identifier>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum GuidanceResponseStatus {
  #[serde(rename = "success")]
  Success,

  #[serde(rename = "data-requested")]
  DataRequested,

  #[serde(rename = "data-required")]
  DataRequired,

  #[serde(rename = "in-progress")]
  InProgress,

  #[serde(rename = "failure")]
  Failure,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}