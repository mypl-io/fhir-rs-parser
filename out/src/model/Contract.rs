#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::Attachment::Attachment;
use crate::model::Contract_Friendly::Contract_Friendly;
use crate::model::Reference::Reference;
use crate::model::Contract_Legal::Contract_Legal;
use crate::model::Identifier::Identifier;
use crate::model::Contract_Signer::Contract_Signer;
use crate::model::Period::Period;
use crate::model::Narrative::Narrative;
use crate::model::Contract_ContentDefinition::Contract_ContentDefinition;
use crate::model::Contract_Rule::Contract_Rule;
use crate::model::Contract_Term::Contract_Term;


/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
  /// Sub-category for the Contract that distinguishes the kinds of systems that would
  /// be interested in the Contract within the context of the Contract's scope.
  #[serde(rename = "subType")]
  sub_type: Option<Vec<CodeableConcept>>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// A selector of legal concerns for this Contract definition, derivative, or
  /// instance in any legal state.
  scope: Option<CodeableConcept>,

  /// Recognized governance framework or system operating with a circumscribed scope
  /// in accordance with specified principles, policies, processes or procedures for
  /// managing rights, actions, or behaviors of parties or principals relative to
  /// resources.
  domain: Option<Vec<Box<Reference>>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The "patient friendly language" versionof the Contract in whole or in parts.
  /// "Patient friendly language" means the representation of the Contract and
  /// Contract Provisions in a manner that is readily accessible and understandable by
  /// a layperson in accordance with best practices for communication styles that
  /// ensure that those agreeing to or signing the Contract understand the roles,
  /// actions, obligations, responsibilities, and implication of the agreement.
  friendly: Option<Vec<Contract_Friendly>>,

  /// Legally binding Contract: This is the signed and legally recognized
  /// representation of the Contract, which is considered the "source of truth" and
  /// which would be the basis for legal action related to enforcement of this
  /// Contract.
  #[serde(rename = "legallyBindingReference")]
  legally_binding_reference: Option<Box<Reference>>,

  /// Narrows the range of legal concerns to focus on the achievement of specific
  /// contractual objectives.
  #[serde(rename = "topicCodeableConcept")]
  topic_codeable_concept: Option<CodeableConcept>,

  /// An explanatory or alternate user-friendly title for this Contract definition,
  /// derivative, or instance in any legal state.t giving additional information about
  /// its content.
  subtitle: Option<String>,

  /// Links to Provenance records for past versions of this Contract definition,
  /// derivative, or instance, which identify key state transitions or updates that
  /// are likely to be relevant to a user looking at the current version of the
  /// Contract.  The Provence.entity indicates the target that was changed in the
  /// update. http://build.fhir.org/provenance-definitions.html#Provenance.entity.
  #[serde(rename = "relevantHistory")]
  relevant_history: Option<Vec<Box<Reference>>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The minimal content derived from the basal information source at a specific
  /// stage in its lifecycle.
  #[serde(rename = "contentDerivative")]
  content_derivative: Option<CodeableConcept>,

  /// Extensions for issued
  #[serde(rename = "_issued")]
  _issued: Option<Element>,

  /// Narrows the range of legal concerns to focus on the achievement of specific
  /// contractual objectives.
  #[serde(rename = "topicReference")]
  topic_reference: Option<Box<Reference>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Event resulting in discontinuation or termination of this Contract instance by
  /// one or more parties to the contract.
  #[serde(rename = "expirationType")]
  expiration_type: Option<CodeableConcept>,

  /// Precusory content developed with a focus and intent of supporting the formation
  /// a Contract instance, which may be associated with and transformable into a
  /// Contract.
  #[serde(rename = "contentDefinition")]
  content_definition: Option<Contract_ContentDefinition>,

  /// List of Computable Policy Rule Language Representations of this Contract.
  rule: Option<Vec<Contract_Rule>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// A short, descriptive, user-friendly title for this Contract definition,
  /// derivative, or instance in any legal state.t giving additional information about
  /// its content.
  title: Option<String>,

  /// An edition identifier used for business purposes to label business significant
  /// variants.
  version: Option<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Information that may be needed by/relevant to the performer in their execution
  /// of this term action.
  #[serde(rename = "supportingInfo")]
  supporting_info: Option<Vec<Box<Reference>>>,

  /// The URL pointing to an externally maintained definition that is adhered to in
  /// whole or in part by this Contract.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Option<String>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// The status of the resource instance.
  status: Option<String>,

  /// Sites in which the contract is complied with,  exercised, or in force.
  site: Option<Vec<Box<Reference>>>,

  /// One or more Contract Provisions, which may be related and conveyed as a group,
  /// and may contain nested groups.
  term: Option<Vec<Contract_Term>>,

  /// Legally binding Contract: This is the signed and legally recognized
  /// representation of the Contract, which is considered the "source of truth" and
  /// which would be the basis for legal action related to enforcement of this
  /// Contract.
  #[serde(rename = "legallyBindingAttachment")]
  legally_binding_attachment: Option<Attachment>,

  /// Relevant time or time-period when this Contract is applicable.
  applies: Option<Period>,

  /// Extensions for subtitle
  #[serde(rename = "_subtitle")]
  _subtitle: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// When this  Contract was issued.
  issued: Option<String>,

  /// Extensions for alias
  #[serde(rename = "_alias")]
  _alias: Option<Vec<Element>>,

  /// A formally or informally recognized grouping of people, principals,
  /// organizations, or jurisdictions formed for the purpose of achieving some form of
  /// collective action such as the promulgation, administration and enforcement of
  /// contracts and policies.
  authority: Option<Vec<Box<Reference>>>,

  /// The individual or organization that authored the Contract definition,
  /// derivative, or instance in any legal state.
  author: Option<Box<Reference>>,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Option<Element>,

  /// A high-level category for the legal instrument, whether constructed as a
  /// Contract definition, derivative, or instance in any legal state.  Provides
  /// additional information about its content within the context of the Contract's
  /// scope to distinguish the kinds of systems that would be interested in the
  /// contract.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// List of Legal expressions or representations of this Contract.
  legal: Option<Vec<Contract_Legal>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Unique identifier for this Contract or a derivative that references a Source
  /// Contract.
  identifier: Option<Vec<Identifier>>,

  /// The URL pointing to a FHIR-defined Contract Definition that is adhered to in
  /// whole or part by this Contract.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Option<Box<Reference>>,

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

  /// A natural language name identifying this Contract definition, derivative, or
  /// instance in any legal state. Provides additional information about its content.
  /// This name should be usable as an identifier for the module by machine processing
  /// applications such as code generation.
  name: Option<String>,

  /// Alternative representation of the title for this Contract definition,
  /// derivative, or instance in any legal state., e.g., a domain specific contract
  /// number related to legislation.
  alias: Option<Vec<String>>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// The target entity impacted by or of interest to parties to the agreement.
  subject: Option<Vec<Box<Reference>>>,

  /// Legal states of the formation of a legal instrument, which is a formally
  /// executed written document that can be formally attributed to its author, records
  /// and formally expresses a legally enforceable act, process, or contractual duty,
  /// obligation, or right, and therefore evidences that act, process, or agreement.
  #[serde(rename = "legalState")]
  legal_state: Option<CodeableConcept>,

  /// Parties with legal standing in the Contract, including the principal parties,
  /// the grantor(s) and grantee(s), which are any person or organization bound by the
  /// contract, and any ancillary parties, which facilitate the execution of the
  /// contract such as a notary or witness.
  signer: Option<Vec<Contract_Signer>>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// Canonical identifier for this contract, represented as a URI (globally unique).
  url: Option<String>,

}