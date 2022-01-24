#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::Attachment::AttachmentGraphql;
use crate::model::ClaimResponse_AddItem::ClaimResponse_AddItem;
use crate::model::ClaimResponse_AddItem::ClaimResponse_AddItemGraphql;
use crate::model::ClaimResponse_Adjudication::ClaimResponse_Adjudication;
use crate::model::ClaimResponse_Adjudication::ClaimResponse_AdjudicationGraphql;
use crate::model::ClaimResponse_Error::ClaimResponse_Error;
use crate::model::ClaimResponse_Error::ClaimResponse_ErrorGraphql;
use crate::model::ClaimResponse_Insurance::ClaimResponse_Insurance;
use crate::model::ClaimResponse_Insurance::ClaimResponse_InsuranceGraphql;
use crate::model::ClaimResponse_Item::ClaimResponse_Item;
use crate::model::ClaimResponse_Item::ClaimResponse_ItemGraphql;
use crate::model::ClaimResponse_Payment::ClaimResponse_Payment;
use crate::model::ClaimResponse_Payment::ClaimResponse_PaymentGraphql;
use crate::model::ClaimResponse_ProcessNote::ClaimResponse_ProcessNote;
use crate::model::ClaimResponse_ProcessNote::ClaimResponse_ProcessNoteGraphql;
use crate::model::ClaimResponse_Total::ClaimResponse_Total;
use crate::model::ClaimResponse_Total::ClaimResponse_TotalGraphql;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::CodeableConcept::CodeableConceptGraphql;
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
use crate::model::ResourceList::ResourceList;
use crate::model::ResourceList::ResourceListGraphql;
use async_graphql::*;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClaimResponse<'_> {
    pub fn new(value: &Value) -> ClaimResponse {
        ClaimResponse {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for created
    pub fn _created(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_created") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for disposition
    pub fn _disposition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_disposition") {
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

    /// Extensions for outcome
    pub fn _outcome(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_outcome") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for preAuthRef
    pub fn _pre_auth_ref(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preAuthRef") {
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

    /// Extensions for use
    pub fn _use(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_use") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The first-tier service adjudications for payor added product or service lines.
    pub fn add_item(&self) -> Option<Vec<ClaimResponse_AddItem>> {
        if let Some(Value::Array(val)) = self.value.get("addItem") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_AddItem {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The adjudication results which are presented at the header level rather than at
    /// the line-item or add-item levels.
    pub fn adjudication(&self) -> Option<Vec<ClaimResponse_Adjudication>> {
        if let Some(Value::Array(val)) = self.value.get("adjudication") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_Adjudication {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Request for additional supporting or authorizing information.
    pub fn communication_request(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("communicationRequest") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
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

    /// The date this resource was created.
    pub fn created(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("created") {
            return Some(string);
        }
        return None;
    }

    /// A human readable description of the status of the adjudication.
    pub fn disposition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("disposition") {
            return Some(string);
        }
        return None;
    }

    /// Errors encountered during the processing of the adjudication.
    pub fn error(&self) -> Option<Vec<ClaimResponse_Error>> {
        if let Some(Value::Array(val)) = self.value.get("error") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_Error {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The actual form, by reference or inclusion, for printing the content or an EOB.
    pub fn form(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("form") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code for the form to be used for printing the content.
    pub fn form_code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("formCode") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code, used only on a response to a preauthorization, to indicate whether the
    /// benefits payable have been reserved and for whom.
    pub fn funds_reserve(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fundsReserve") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// A unique identifier assigned to this claim response.
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

    /// Financial instruments for reimbursement for the health care products and
    /// services specified on the claim.
    pub fn insurance(&self) -> Option<Vec<ClaimResponse_Insurance>> {
        if let Some(Value::Array(val)) = self.value.get("insurance") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_Insurance {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The party responsible for authorization, adjudication and reimbursement.
    pub fn insurer(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["insurer"]),
        }
    }

    /// A claim line. Either a simple (a product or service) or a 'group' of details
    /// which can also be a simple items or groups of sub-details.
    pub fn item(&self) -> Option<Vec<ClaimResponse_Item>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_Item {
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

    /// The outcome of the claim, predetermination, or preauthorization processing.
    pub fn outcome(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("outcome") {
            return Some(string);
        }
        return None;
    }

    /// The party to whom the professional services and/or products have been supplied
    /// or are being considered and for whom actual for facast reimbursement is sought.
    pub fn patient(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["patient"]),
        }
    }

    /// Type of Party to be reimbursed: subscriber, provider, other.
    pub fn payee_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("payeeType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Payment details for the adjudication of the claim.
    pub fn payment(&self) -> Option<ClaimResponse_Payment> {
        if let Some(val) = self.value.get("payment") {
            return Some(ClaimResponse_Payment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The time frame during which this authorization is effective.
    pub fn pre_auth_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("preAuthPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Reference from the Insurer which is used in later communications which refers to
    /// this adjudication.
    pub fn pre_auth_ref(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("preAuthRef") {
            return Some(string);
        }
        return None;
    }

    /// A note that describes or explains adjudication results in a human readable form.
    pub fn process_note(&self) -> Option<Vec<ClaimResponse_ProcessNote>> {
        if let Some(Value::Array(val)) = self.value.get("processNote") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_ProcessNote {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Original request resource reference.
    pub fn request(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("request") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The provider which is responsible for the claim, predetermination or
    /// preauthorization.
    pub fn requestor(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requestor") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The status of the resource instance.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// A finer grained suite of claim type codes which may convey additional
    /// information such as Inpatient vs Outpatient and/or a specialty service.
    pub fn sub_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subType") {
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

    /// Categorized monetary totals for the adjudication.
    pub fn total(&self) -> Option<Vec<ClaimResponse_Total>> {
        if let Some(Value::Array(val)) = self.value.get("total") {
            return Some(
                val.into_iter()
                    .map(|e| ClaimResponse_Total {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A finer grained suite of claim type codes which may convey additional
    /// information such as Inpatient vs Outpatient and/or a specialty service.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    /// A code to indicate whether the nature of the request is: to request adjudication
    /// of products and services previously rendered; or requesting authorization and
    /// adjudication for provision in the future; or requesting the non-binding
    /// adjudication of the listed products and services which could be provided in the
    /// future.
    pub fn fhir_use(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("use") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._created() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._disposition() {
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
        if let Some(_val) = self._outcome() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._pre_auth_ref() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._use() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.add_item() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.adjudication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.communication_request() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.created() {}
        if let Some(_val) = self.disposition() {}
        if let Some(_val) = self.error() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.form() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.form_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.funds_reserve() {
            if !_val.validate() {
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
        if let Some(_val) = self.insurance() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.insurer().validate() {
            return false;
        }
        if let Some(_val) = self.item() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
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
        if let Some(_val) = self.outcome() {}
        if !self.patient().validate() {
            return false;
        }
        if let Some(_val) = self.payee_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.payment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.pre_auth_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.pre_auth_ref() {}
        if let Some(_val) = self.process_note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.request() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.requestor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.sub_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.total() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.fhir_type().validate() {
            return false;
        }
        if let Some(_val) = self.fhir_use() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ClaimResponseBuilder {
    pub(crate) value: Value,
}

impl ClaimResponseBuilder {
    pub fn build(&self) -> ClaimResponse {
        ClaimResponse {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ClaimResponse) -> ClaimResponseBuilder {
        ClaimResponseBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        insurer: Reference,
        patient: Reference,
        fhir_type: CodeableConcept,
    ) -> ClaimResponseBuilder {
        let mut __value: Value = json!({});
        __value["insurer"] = json!(insurer.value);
        __value["patient"] = json!(patient.value);
        __value["type"] = json!(fhir_type.value);
        return ClaimResponseBuilder { value: __value };
    }

    pub fn _created<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponseBuilder {
        self.value["_created"] = json!(val.value);
        return self;
    }

    pub fn _disposition<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponseBuilder {
        self.value["_disposition"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponseBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponseBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _outcome<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponseBuilder {
        self.value["_outcome"] = json!(val.value);
        return self;
    }

    pub fn _pre_auth_ref<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponseBuilder {
        self.value["_preAuthRef"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponseBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _use<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponseBuilder {
        self.value["_use"] = json!(val.value);
        return self;
    }

    pub fn add_item<'a>(
        &'a mut self,
        val: Vec<ClaimResponse_AddItem>,
    ) -> &'a mut ClaimResponseBuilder {
        self.value["addItem"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn adjudication<'a>(
        &'a mut self,
        val: Vec<ClaimResponse_Adjudication>,
    ) -> &'a mut ClaimResponseBuilder {
        self.value["adjudication"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn communication_request<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut ClaimResponseBuilder {
        self.value["communicationRequest"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut ClaimResponseBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn created<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponseBuilder {
        self.value["created"] = json!(val);
        return self;
    }

    pub fn disposition<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponseBuilder {
        self.value["disposition"] = json!(val);
        return self;
    }

    pub fn error<'a>(&'a mut self, val: Vec<ClaimResponse_Error>) -> &'a mut ClaimResponseBuilder {
        self.value["error"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ClaimResponseBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn form<'a>(&'a mut self, val: Attachment) -> &'a mut ClaimResponseBuilder {
        self.value["form"] = json!(val.value);
        return self;
    }

    pub fn form_code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ClaimResponseBuilder {
        self.value["formCode"] = json!(val.value);
        return self;
    }

    pub fn funds_reserve<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ClaimResponseBuilder {
        self.value["fundsReserve"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponseBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut ClaimResponseBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponseBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn insurance<'a>(
        &'a mut self,
        val: Vec<ClaimResponse_Insurance>,
    ) -> &'a mut ClaimResponseBuilder {
        self.value["insurance"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn item<'a>(&'a mut self, val: Vec<ClaimResponse_Item>) -> &'a mut ClaimResponseBuilder {
        self.value["item"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponseBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ClaimResponseBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClaimResponseBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn outcome<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponseBuilder {
        self.value["outcome"] = json!(val);
        return self;
    }

    pub fn payee_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ClaimResponseBuilder {
        self.value["payeeType"] = json!(val.value);
        return self;
    }

    pub fn payment<'a>(&'a mut self, val: ClaimResponse_Payment) -> &'a mut ClaimResponseBuilder {
        self.value["payment"] = json!(val.value);
        return self;
    }

    pub fn pre_auth_period<'a>(&'a mut self, val: Period) -> &'a mut ClaimResponseBuilder {
        self.value["preAuthPeriod"] = json!(val.value);
        return self;
    }

    pub fn pre_auth_ref<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponseBuilder {
        self.value["preAuthRef"] = json!(val);
        return self;
    }

    pub fn process_note<'a>(
        &'a mut self,
        val: Vec<ClaimResponse_ProcessNote>,
    ) -> &'a mut ClaimResponseBuilder {
        self.value["processNote"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn request<'a>(&'a mut self, val: Reference) -> &'a mut ClaimResponseBuilder {
        self.value["request"] = json!(val.value);
        return self;
    }

    pub fn requestor<'a>(&'a mut self, val: Reference) -> &'a mut ClaimResponseBuilder {
        self.value["requestor"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponseBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn sub_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ClaimResponseBuilder {
        self.value["subType"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ClaimResponseBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn total<'a>(&'a mut self, val: Vec<ClaimResponse_Total>) -> &'a mut ClaimResponseBuilder {
        self.value["total"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_use<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponseBuilder {
        self.value["use"] = json!(val);
        return self;
    }
}

#[derive(Debug, SimpleObject, InputObject)]
pub struct ClaimResponseGraphql {
    _created: Option<ElementGraphql>,
    _disposition: Option<ElementGraphql>,
    _implicit_rules: Option<ElementGraphql>,
    _language: Option<ElementGraphql>,
    _outcome: Option<ElementGraphql>,
    _pre_auth_ref: Option<ElementGraphql>,
    _status: Option<ElementGraphql>,
    _use: Option<ElementGraphql>,
    add_item: Option<Vec<ClaimResponse_AddItemGraphql>>,
    adjudication: Option<Vec<ClaimResponse_AdjudicationGraphql>>,
    communication_request: Option<Vec<ReferenceGraphql>>,
    contained: Option<Vec<ResourceListGraphql>>,
    created: Option<String>,
    disposition: Option<String>,
    error: Option<Vec<ClaimResponse_ErrorGraphql>>,
    extension: Option<Vec<ExtensionGraphql>>,
    form: Option<AttachmentGraphql>,
    form_code: Option<CodeableConceptGraphql>,
    funds_reserve: Option<CodeableConceptGraphql>,
    id: Option<String>,
    identifier: Option<Vec<IdentifierGraphql>>,
    implicit_rules: Option<String>,
    insurance: Option<Vec<ClaimResponse_InsuranceGraphql>>,
    insurer: ReferenceGraphql,
    item: Option<Vec<ClaimResponse_ItemGraphql>>,
    language: Option<String>,
    meta: Option<MetaGraphql>,
    modifier_extension: Option<Vec<ExtensionGraphql>>,
    outcome: Option<String>,
    patient: ReferenceGraphql,
    payee_type: Option<CodeableConceptGraphql>,
    payment: Option<ClaimResponse_PaymentGraphql>,
    pre_auth_period: Option<PeriodGraphql>,
    pre_auth_ref: Option<String>,
    process_note: Option<Vec<ClaimResponse_ProcessNoteGraphql>>,
    request: Option<ReferenceGraphql>,
    requestor: Option<ReferenceGraphql>,
    status: Option<String>,
    sub_type: Option<CodeableConceptGraphql>,
    text: Option<NarrativeGraphql>,
    total: Option<Vec<ClaimResponse_TotalGraphql>>,
    fhir_type: CodeableConceptGraphql,
    fhir_use: Option<String>,
}