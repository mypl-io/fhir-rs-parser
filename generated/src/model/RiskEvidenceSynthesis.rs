#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::Annotation::AnnotationGraphql;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::CodeableConcept::CodeableConceptGraphql;
use crate::model::ContactDetail::ContactDetail;
use crate::model::ContactDetail::ContactDetailGraphql;
use crate::model::Element::Element;
use crate::model::Element::ElementGraphql;
use crate::model::Extension::Extension;
use crate::model::Extension::ExtensionGraphql;
use crate::model::Identifier::Identifier;
use crate::model::Identifier::IdentifierGraphql;
use crate::model::Meta::Meta;
use crate::model::Meta::MetaGraphql;
use crate::model::Narrative::Narrative;
use crate::model::Narrative::NarrativeGraphql;
use crate::model::Period::Period;
use crate::model::Period::PeriodGraphql;
use crate::model::Reference::Reference;
use crate::model::Reference::ReferenceGraphql;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::RelatedArtifact::RelatedArtifactGraphql;
use crate::model::ResourceList::ResourceList;
use crate::model::ResourceList::ResourceListGraphql;
use crate::model::RiskEvidenceSynthesis_Certainty::RiskEvidenceSynthesis_Certainty;
use crate::model::RiskEvidenceSynthesis_Certainty::RiskEvidenceSynthesis_CertaintyGraphql;
use crate::model::RiskEvidenceSynthesis_RiskEstimate::RiskEvidenceSynthesis_RiskEstimate;
use crate::model::RiskEvidenceSynthesis_RiskEstimate::RiskEvidenceSynthesis_RiskEstimateGraphql;
use crate::model::RiskEvidenceSynthesis_SampleSize::RiskEvidenceSynthesis_SampleSize;
use crate::model::RiskEvidenceSynthesis_SampleSize::RiskEvidenceSynthesis_SampleSizeGraphql;
use crate::model::UsageContext::UsageContext;
use crate::model::UsageContext::UsageContextGraphql;
use async_graphql::*;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a
/// population plus exposure state where the risk estimate is derived from a
/// combination of research studies.

#[derive(Debug)]
pub struct RiskEvidenceSynthesis<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RiskEvidenceSynthesis<'_> {
    pub fn new(value: &Value) -> RiskEvidenceSynthesis {
        RiskEvidenceSynthesis {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for approvalDate
    pub fn _approval_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_approvalDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for lastReviewDate
    pub fn _last_review_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastReviewDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub fn approval_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("approvalDate") {
            return Some(string);
        }
        return None;
    }

    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the content.
    pub fn author(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("author") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A description of the certainty of the risk estimate.
    pub fn certainty(&self) -> Option<Vec<RiskEvidenceSynthesis_Certainty>> {
        if let Some(Value::Array(val)) = self.value.get("certainty") {
            return Some(
                val.into_iter()
                    .map(|e| RiskEvidenceSynthesis_Certainty {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A copyright statement relating to the risk evidence synthesis and/or its
    /// contents. Copyright statements are generally legal restrictions on the use and
    /// publishing of the risk evidence synthesis.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// The date  (and optionally time) when the risk evidence synthesis was published.
    /// The date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the risk evidence synthesis changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// A free text natural language description of the risk evidence synthesis from a
    /// consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// An individual or organization primarily responsible for internal coherence of
    /// the content.
    pub fn editor(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("editor") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The period during which the risk evidence synthesis content was or is planned to
    /// be in active use.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An individual or organization responsible for officially endorsing the content
    /// for use in some setting.
    pub fn endorser(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("endorser") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a EvidenceVariable resource that defines the exposure for the
    /// research.
    pub fn exposure(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("exposure") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A formal identifier that is used to identify this risk evidence synthesis when
    /// it is represented in other formats, or referenced in a specification, model,
    /// design or an instance.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// A legal or geographic region in which the risk evidence synthesis is intended to
    /// be used.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub fn last_review_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastReviewDate") {
            return Some(string);
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

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
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A natural language name identifying the risk evidence synthesis. This name
    /// should be usable as an identifier for the module by machine processing
    /// applications such as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// A human-readable string to clarify or explain concepts about the resource.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a EvidenceVariable resomece that defines the outcome for the
    /// research.
    pub fn outcome(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["outcome"]),
        }
    }

    /// A reference to a EvidenceVariable resource that defines the population for the
    /// research.
    pub fn population(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["population"]),
        }
    }

    /// The name of the organization or individual that published the risk evidence
    /// synthesis.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// Related artifacts such as additional documentation, justification, or
    /// bibliographic references.
    pub fn related_artifact(&self) -> Option<Vec<RelatedArtifact>> {
        if let Some(Value::Array(val)) = self.value.get("relatedArtifact") {
            return Some(
                val.into_iter()
                    .map(|e| RelatedArtifact {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An individual or organization primarily responsible for review of some aspect of
    /// the content.
    pub fn reviewer(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("reviewer") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The estimated risk of the outcome.
    pub fn risk_estimate(&self) -> Option<RiskEvidenceSynthesis_RiskEstimate> {
        if let Some(val) = self.value.get("riskEstimate") {
            return Some(RiskEvidenceSynthesis_RiskEstimate {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A description of the size of the sample involved in the synthesis.
    pub fn sample_size(&self) -> Option<RiskEvidenceSynthesis_SampleSize> {
        if let Some(val) = self.value.get("sampleSize") {
            return Some(RiskEvidenceSynthesis_SampleSize {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The status of this risk evidence synthesis. Enables tracking the life-cycle of
    /// the content.
    pub fn status(&self) -> Option<RiskEvidenceSynthesisStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(RiskEvidenceSynthesisStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Type of study eg randomized trial.
    pub fn study_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("studyType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Type of synthesis eg meta-analysis.
    pub fn synthesis_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("synthesisType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A short, descriptive, user-friendly title for the risk evidence synthesis.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Descriptive topics related to the content of the RiskEvidenceSynthesis. Topics
    /// provide a high-level categorization grouping types of EffectEvidenceSynthesiss
    /// that can be useful for filtering and searching.
    pub fn topic(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("topic") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An absolute URI that is used to identify this risk evidence synthesis when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this risk evidence
    /// synthesis is (or will be) published. This URL can be the target of a canonical
    /// reference. It SHALL remain the same when the risk evidence synthesis is stored
    /// on different servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate risk evidence
    /// synthesis instances.
    pub fn use_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("useContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The identifier that is used to identify this version of the risk evidence
    /// synthesis when it is referenced in a specification, model, design or instance.
    /// This is an arbitrary value managed by the risk evidence synthesis author and is
    /// not expected to be globally unique. For example, it might be a timestamp (e.g.
    /// yyyymmdd) if a managed version is not available. There is also no expectation
    /// that versions can be placed in a lexicographical sequence.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._approval_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._copyright() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._last_review_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._publisher() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._url() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.approval_date() {}
        if let Some(_val) = self.author() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.certainty() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.editor() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.effective_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.endorser() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.exposure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.jurisdiction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.last_review_date() {}
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.outcome().validate() {
            return false;
        }
        if !self.population().validate() {
            return false;
        }
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.related_artifact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reviewer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.risk_estimate() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sample_size() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.study_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.synthesis_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.topic() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.use_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct RiskEvidenceSynthesisBuilder {
    pub(crate) value: Value,
}

impl RiskEvidenceSynthesisBuilder {
    pub fn build(&self) -> RiskEvidenceSynthesis {
        RiskEvidenceSynthesis {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: RiskEvidenceSynthesis) -> RiskEvidenceSynthesisBuilder {
        RiskEvidenceSynthesisBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(outcome: Reference, population: Reference) -> RiskEvidenceSynthesisBuilder {
        let mut __value: Value = json!({});
        __value["outcome"] = json!(outcome.value);
        __value["population"] = json!(population.value);
        return RiskEvidenceSynthesisBuilder { value: __value };
    }

    pub fn _approval_date<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_approvalDate"] = json!(val.value);
        return self;
    }

    pub fn _copyright<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_copyright"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _last_review_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_lastReviewDate"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _publisher<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_publisher"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn approval_date<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["approvalDate"] = json!(val);
        return self;
    }

    pub fn author<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["author"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn certainty<'a>(
        &'a mut self,
        val: Vec<RiskEvidenceSynthesis_Certainty>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["certainty"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contact<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn copyright<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["copyright"] = json!(val);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn editor<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["editor"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn effective_period<'a>(&'a mut self, val: Period) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["effectivePeriod"] = json!(val.value);
        return self;
    }

    pub fn endorser<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["endorser"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn exposure<'a>(&'a mut self, val: Reference) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["exposure"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn last_review_date<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["lastReviewDate"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["publisher"] = json!(val);
        return self;
    }

    pub fn related_artifact<'a>(
        &'a mut self,
        val: Vec<RelatedArtifact>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["relatedArtifact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reviewer<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["reviewer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn risk_estimate<'a>(
        &'a mut self,
        val: RiskEvidenceSynthesis_RiskEstimate,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["riskEstimate"] = json!(val.value);
        return self;
    }

    pub fn sample_size<'a>(
        &'a mut self,
        val: RiskEvidenceSynthesis_SampleSize,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["sampleSize"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: RiskEvidenceSynthesisStatus,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn study_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["studyType"] = json!(val.value);
        return self;
    }

    pub fn synthesis_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["synthesisType"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn topic<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["topic"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut RiskEvidenceSynthesisBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}

#[derive(Debug, SimpleObject, InputObject)]
pub struct RiskEvidenceSynthesisGraphql {
    _approval_date: Option<ElementGraphql>,
    _copyright: Option<ElementGraphql>,
    _date: Option<ElementGraphql>,
    _description: Option<ElementGraphql>,
    _implicit_rules: Option<ElementGraphql>,
    _language: Option<ElementGraphql>,
    _last_review_date: Option<ElementGraphql>,
    _name: Option<ElementGraphql>,
    _publisher: Option<ElementGraphql>,
    _status: Option<ElementGraphql>,
    _title: Option<ElementGraphql>,
    _url: Option<ElementGraphql>,
    _version: Option<ElementGraphql>,
    approval_date: Option<String>,
    author: Option<Vec<ContactDetailGraphql>>,
    certainty: Option<Vec<RiskEvidenceSynthesis_CertaintyGraphql>>,
    contact: Option<Vec<ContactDetailGraphql>>,
    contained: Option<Vec<ResourceListGraphql>>,
    copyright: Option<String>,
    date: Option<String>,
    description: Option<String>,
    editor: Option<Vec<ContactDetailGraphql>>,
    effective_period: Option<PeriodGraphql>,
    endorser: Option<Vec<ContactDetailGraphql>>,
    exposure: Option<ReferenceGraphql>,
    extension: Option<Vec<ExtensionGraphql>>,
    id: Option<String>,
    identifier: Option<Vec<IdentifierGraphql>>,
    implicit_rules: Option<String>,
    jurisdiction: Option<Vec<CodeableConceptGraphql>>,
    language: Option<String>,
    last_review_date: Option<String>,
    meta: Option<MetaGraphql>,
    modifier_extension: Option<Vec<ExtensionGraphql>>,
    name: Option<String>,
    note: Option<Vec<AnnotationGraphql>>,
    outcome: ReferenceGraphql,
    population: ReferenceGraphql,
    publisher: Option<String>,
    related_artifact: Option<Vec<RelatedArtifactGraphql>>,
    reviewer: Option<Vec<ContactDetailGraphql>>,
    risk_estimate: Option<RiskEvidenceSynthesis_RiskEstimateGraphql>,
    sample_size: Option<RiskEvidenceSynthesis_SampleSizeGraphql>,
    status: Option<RiskEvidenceSynthesisStatus>,
    study_type: Option<CodeableConceptGraphql>,
    synthesis_type: Option<CodeableConceptGraphql>,
    text: Option<NarrativeGraphql>,
    title: Option<String>,
    topic: Option<Vec<CodeableConceptGraphql>>,
    url: Option<String>,
    use_context: Option<Vec<UsageContextGraphql>>,
    version: Option<String>,
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq)]
pub enum RiskEvidenceSynthesisStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl RiskEvidenceSynthesisStatus {
    pub fn from_string(string: &str) -> Option<RiskEvidenceSynthesisStatus> {
        match string {
            "draft" => Some(RiskEvidenceSynthesisStatus::Draft),
            "active" => Some(RiskEvidenceSynthesisStatus::Active),
            "retired" => Some(RiskEvidenceSynthesisStatus::Retired),
            "unknown" => Some(RiskEvidenceSynthesisStatus::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            RiskEvidenceSynthesisStatus::Draft => "draft".to_string(),
            RiskEvidenceSynthesisStatus::Active => "active".to_string(),
            RiskEvidenceSynthesisStatus::Retired => "retired".to_string(),
            RiskEvidenceSynthesisStatus::Unknown => "unknown".to_string(),
        }
    }
}
