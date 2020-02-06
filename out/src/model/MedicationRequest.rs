#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::MedicationRequest_DispenseRequest::MedicationRequest_DispenseRequest;
use crate::model::MedicationRequest_Substitution::MedicationRequest_Substitution;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Identifier::Identifier;
use crate::model::Annotation::Annotation;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Dosage::Dosage;


/// An order or request for both supply of the medication and the instructions for
/// administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to
/// generalize the use across inpatient and outpatient settings, including care
/// plans, etc., and to harmonize with workflow patterns.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequest {
  /// Indicates an actual or potential clinical issue with or between one or more
  /// active or proposed clinical actions for a patient; e.g. Drug-drug interaction,
  /// duplicate therapy, dosage alert etc.
  #[serde(rename = "detectedIssue")]
  detected_issue: Option<Vec<Box<Reference>>>,

  /// Extensions for instantiatesCanonical
  #[serde(rename = "_instantiatesCanonical")]
  _instantiates_canonical: Option<Vec<Element>>,

  /// The person who entered the order on behalf of another individual for example in
  /// the case of a verbal or a telephone order.
  recorder: Option<Box<Reference>>,

  /// A code specifying the current state of the order.  Generally, this will be
  /// active or completed state.
  status: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Include additional information (for example, patient height and weight) that
  /// supports the ordering of the medication.
  #[serde(rename = "supportingInformation")]
  supporting_information: Option<Vec<Box<Reference>>>,

  /// A link to a resource representing an earlier order related order or
  /// prescription.
  #[serde(rename = "priorPrescription")]
  prior_prescription: Option<Box<Reference>>,

  /// Identifies the medication being requested. This is a link to a resource that
  /// represents the medication which may be the details of the medication or simply
  /// an attribute carrying a code that identifies the medication from a known list of
  /// medications.
  #[serde(rename = "medicationCodeableConcept")]
  medication_codeable_concept: Option<CodeableConcept>,

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

  /// The individual, organization, or device that initiated the request and has
  /// responsibility for its activation.
  requester: Option<Box<Reference>>,

  /// Indicates how the medication is to be used by the patient.
  #[serde(rename = "dosageInstruction")]
  dosage_instruction: Option<Vec<Dosage>>,

  /// The date (and perhaps time) when the prescription was initially written or
  /// authored on.
  #[serde(rename = "authoredOn")]
  authored_on: Option<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Extensions for intent
  #[serde(rename = "_intent")]
  _intent: Option<Element>,

  /// Indicates if this record was captured as a secondary 'reported' record rather
  /// than as an original primary source-of-truth record.  It may also indicate the
  /// source of the report.
  #[serde(rename = "reportedReference")]
  reported_reference: Option<Box<Reference>>,

  /// Extensions for authoredOn
  #[serde(rename = "_authoredOn")]
  _authored_on: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The reason or the indication for ordering or not ordering the medication.
  #[serde(rename = "reasonCode")]
  reason_code: Option<Vec<CodeableConcept>>,

  /// The URL pointing to an externally maintained protocol, guideline, orderset or
  /// other definition that is adhered to in whole or in part by this
  /// MedicationRequest.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Option<Vec<String>>,

  /// Extra information about the prescription that could not be conveyed by the other
  /// attributes.
  note: Option<Vec<Annotation>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for reportedBoolean
  #[serde(rename = "_reportedBoolean")]
  _reported_boolean: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
  /// determinations that may be required for delivering the requested service.
  insurance: Option<Vec<Box<Reference>>>,

  /// A link to a resource representing the person or set of individuals to whom the
  /// medication will be given.
  subject: Box<Reference>,

  /// Extensions for priority
  #[serde(rename = "_priority")]
  _priority: Option<Element>,

  /// Identifiers associated with this medication request that are defined by business
  /// processes and/or used to refer to it when a direct URL reference to the resource
  /// itself is not appropriate. They are business identifiers assigned to this
  /// resource by the performer or other systems and remain constant as the resource
  /// is updated and propagates from server to server.
  identifier: Option<Vec<Identifier>>,

  /// Indicates how quickly the Medication Request should be addressed with respect to
  /// other requests.
  priority: Option<String>,

  /// The specified desired performer of the medication treatment (e.g. the performer
  /// of the medication administration).
  performer: Option<Box<Reference>>,

  /// Indicates the type of medication request (for example, where the medication is
  /// expected to be consumed or administered (i.e. inpatient or outpatient)).
  category: Option<Vec<CodeableConcept>>,

  /// A plan or request that is fulfilled in whole or in part by this medication
  /// request.
  #[serde(rename = "basedOn")]
  based_on: Option<Vec<Box<Reference>>>,

  /// Captures the reason for the current state of the MedicationRequest.
  #[serde(rename = "statusReason")]
  status_reason: Option<CodeableConcept>,

  /// Identifies the medication being requested. This is a link to a resource that
  /// represents the medication which may be the details of the medication or simply
  /// an attribute carrying a code that identifies the medication from a known list of
  /// medications.
  #[serde(rename = "medicationReference")]
  medication_reference: Option<Box<Reference>>,

  /// A shared identifier common to all requests that were authorized more or less
  /// simultaneously by a single author, representing the identifier of the
  /// requisition or prescription.
  #[serde(rename = "groupIdentifier")]
  group_identifier: Option<Identifier>,

  /// Indicates the specific details for the dispense or medication supply part of a
  /// medication request (also known as a Medication Prescription or Medication
  /// Order).  Note that this information is not always sent with the order.  There
  /// may be in some settings (e.g. hospitals) institutional or system support for
  /// completing the dispense details in the pharmacy department.
  #[serde(rename = "dispenseRequest")]
  dispense_request: Option<MedicationRequest_DispenseRequest>,

  /// Indicates the type of performer of the administration of the medication.
  #[serde(rename = "performerType")]
  performer_type: Option<CodeableConcept>,

  /// Indicates if this record was captured as a secondary 'reported' record rather
  /// than as an original primary source-of-truth record.  It may also indicate the
  /// source of the report.
  #[serde(rename = "reportedBoolean")]
  reported_boolean: Option<bool>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// If true indicates that the provider is asking for the medication request not to
  /// occur.
  #[serde(rename = "doNotPerform")]
  do_not_perform: Option<bool>,

  /// Extensions for doNotPerform
  #[serde(rename = "_doNotPerform")]
  _do_not_perform: Option<Element>,

  /// Indicates whether or not substitution can or should be part of the dispense. In
  /// some cases, substitution must happen, in other cases substitution must not
  /// happen. This block explains the prescriber's intent. If nothing is specified
  /// substitution may be done.
  substitution: Option<MedicationRequest_Substitution>,

  /// Links to Provenance records for past versions of this resource or fulfilling
  /// request or event resources that identify key state transitions or updates that
  /// are likely to be relevant to a user looking at the current version of the
  /// resource.
  #[serde(rename = "eventHistory")]
  event_history: Option<Vec<Box<Reference>>>,

  /// Condition or observation that supports why the medication was ordered.
  #[serde(rename = "reasonReference")]
  reason_reference: Option<Vec<Box<Reference>>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Whether the request is a proposal, plan, or an original order.
  intent: Option<String>,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Option<Vec<Element>>,

  /// The description of the overall patte3rn of the administration of the medication
  /// to the patient.
  #[serde(rename = "courseOfTherapyType")]
  course_of_therapy_type: Option<CodeableConcept>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The Encounter during which this [x] was created or to which the creation of this
  /// record is tightly associated.
  encounter: Option<Box<Reference>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The URL pointing to a protocol, guideline, orderset, or other definition that is
  /// adhered to in whole or in part by this MedicationRequest.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Option<Vec<String>>,

}