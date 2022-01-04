#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::Address::AddressGraphql;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::CodeableConcept::CodeableConceptGraphql;
use crate::model::Element::Element;
use crate::model::Element::ElementGraphql;
use crate::model::ExplanationOfBenefit_Adjudication::ExplanationOfBenefit_Adjudication;
use crate::model::ExplanationOfBenefit_Adjudication::ExplanationOfBenefit_AdjudicationGraphql;
use crate::model::ExplanationOfBenefit_Detail1::ExplanationOfBenefit_Detail1;
use crate::model::ExplanationOfBenefit_Detail1::ExplanationOfBenefit_Detail1Graphql;
use crate::model::Extension::Extension;
use crate::model::Extension::ExtensionGraphql;
use crate::model::Money::Money;
use crate::model::Money::MoneyGraphql;
use crate::model::Period::Period;
use crate::model::Period::PeriodGraphql;
use crate::model::Quantity::Quantity;
use crate::model::Quantity::QuantityGraphql;
use crate::model::Reference::Reference;
use crate::model::Reference::ReferenceGraphql;
use async_graphql::*;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.

#[derive(Debug)]
pub struct ExplanationOfBenefit_AddItem<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExplanationOfBenefit_AddItem<'_> {
    pub fn new(value: &Value) -> ExplanationOfBenefit_AddItem {
        ExplanationOfBenefit_AddItem {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for detailSequence
    pub fn _detail_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_detailSequence") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for factor
    pub fn _factor(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_factor") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for itemSequence
    pub fn _item_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_itemSequence") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for noteNumber
    pub fn _note_number(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_noteNumber") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for servicedDate
    pub fn _serviced_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_servicedDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for subDetailSequence
    pub fn _sub_detail_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_subDetailSequence") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The adjudication results.
    pub fn adjudication(&self) -> Option<Vec<ExplanationOfBenefit_Adjudication>> {
        if let Some(Value::Array(val)) = self.value.get("adjudication") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Adjudication {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Physical service site on the patient (limb, tooth, etc.).
    pub fn body_site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The second-tier service adjudications for payor added services.
    pub fn detail(&self) -> Option<Vec<ExplanationOfBenefit_Detail1>> {
        if let Some(Value::Array(val)) = self.value.get("detail") {
            return Some(
                val.into_iter()
                    .map(|e| ExplanationOfBenefit_Detail1 {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The sequence number of the details within the claim item which this line is
    /// intended to replace.
    pub fn detail_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("detailSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
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

    /// A real number that represents a multiplier used in determining the overall value
    /// of services delivered and/or goods received. The concept of a Factor allows for
    /// a discount or surcharge multiplier to be applied to a monetary amount.
    pub fn factor(&self) -> Option<f64> {
        if let Some(val) = self.value.get("factor") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Claim items which this service line is intended to replace.
    pub fn item_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("itemSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Where the product or service was provided.
    pub fn location_address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("locationAddress") {
            return Some(Address {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Where the product or service was provided.
    pub fn location_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("locationCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Where the product or service was provided.
    pub fn location_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("locationReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Item typification or modifiers codes to convey additional context for the
    /// product or service.
    pub fn modifier(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("modifier") {
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

    /// The quantity times the unit price for an additional service or product or
    /// charge.
    pub fn net(&self) -> Option<Money> {
        if let Some(val) = self.value.get("net") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The numbers associated with notes below which apply to the adjudication of this
    /// item.
    pub fn note_number(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("noteNumber") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// When the value is a group code then this item collects a set of related claim
    /// details, otherwise this contains the product, service, drug or other billing
    /// code for the item.
    pub fn product_or_service(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["productOrService"]),
        }
    }

    /// Identifies the program under which this may be recovered.
    pub fn program_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("programCode") {
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

    /// The providers who are authorized for the services rendered to the patient.
    pub fn provider(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("provider") {
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

    /// The number of repetitions of a service or product.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub fn serviced_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("servicedDate") {
            return Some(string);
        }
        return None;
    }

    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub fn serviced_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("servicedPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The sequence number of the sub-details woithin the details within the claim item
    /// which this line is intended to replace.
    pub fn sub_detail_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("subDetailSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A region or surface of the bodySite, e.g. limb region or tooth surface(s).
    pub fn sub_site(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("subSite") {
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

    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub fn unit_price(&self) -> Option<Money> {
        if let Some(val) = self.value.get("unitPrice") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._detail_sequence() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._factor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._item_sequence() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._note_number() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._serviced_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._sub_detail_sequence() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.adjudication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.body_site() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.detail() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.detail_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.factor() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.item_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.location_address() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.location_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.location_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.net() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.note_number() {
            _val.into_iter().for_each(|_e| {});
        }
        if !self.product_or_service().validate() {
            return false;
        }
        if let Some(_val) = self.program_code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.provider() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.serviced_date() {}
        if let Some(_val) = self.serviced_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sub_detail_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.sub_site() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.unit_price() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ExplanationOfBenefit_AddItemBuilder {
    pub(crate) value: Value,
}

impl ExplanationOfBenefit_AddItemBuilder {
    pub fn build(&self) -> ExplanationOfBenefit_AddItem {
        ExplanationOfBenefit_AddItem {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ExplanationOfBenefit_AddItem) -> ExplanationOfBenefit_AddItemBuilder {
        ExplanationOfBenefit_AddItemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(product_or_service: CodeableConcept) -> ExplanationOfBenefit_AddItemBuilder {
        let mut __value: Value = json!({});
        __value["productOrService"] = json!(product_or_service.value);
        return ExplanationOfBenefit_AddItemBuilder { value: __value };
    }

    pub fn _detail_sequence<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["_detailSequence"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _factor<'a>(&'a mut self, val: Element) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["_factor"] = json!(val.value);
        return self;
    }

    pub fn _item_sequence<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["_itemSequence"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _note_number<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["_noteNumber"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _serviced_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["_servicedDate"] = json!(val.value);
        return self;
    }

    pub fn _sub_detail_sequence<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["_subDetailSequence"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn adjudication<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_Adjudication>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["adjudication"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn body_site<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["bodySite"] = json!(val.value);
        return self;
    }

    pub fn detail<'a>(
        &'a mut self,
        val: Vec<ExplanationOfBenefit_Detail1>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["detail"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn detail_sequence<'a>(
        &'a mut self,
        val: Vec<i64>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["detailSequence"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn factor<'a>(&'a mut self, val: f64) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["factor"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn item_sequence<'a>(
        &'a mut self,
        val: Vec<i64>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["itemSequence"] = json!(val);
        return self;
    }

    pub fn location_address<'a>(
        &'a mut self,
        val: Address,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["locationAddress"] = json!(val.value);
        return self;
    }

    pub fn location_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["locationCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn location_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["locationReference"] = json!(val.value);
        return self;
    }

    pub fn modifier<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["modifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn net<'a>(&'a mut self, val: Money) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["net"] = json!(val.value);
        return self;
    }

    pub fn note_number<'a>(
        &'a mut self,
        val: Vec<i64>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["noteNumber"] = json!(val);
        return self;
    }

    pub fn program_code<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["programCode"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn provider<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["provider"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn serviced_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["servicedDate"] = json!(val);
        return self;
    }

    pub fn serviced_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["servicedPeriod"] = json!(val.value);
        return self;
    }

    pub fn sub_detail_sequence<'a>(
        &'a mut self,
        val: Vec<i64>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["subDetailSequence"] = json!(val);
        return self;
    }

    pub fn sub_site<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["subSite"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn unit_price<'a>(&'a mut self, val: Money) -> &'a mut ExplanationOfBenefit_AddItemBuilder {
        self.value["unitPrice"] = json!(val.value);
        return self;
    }
}

#[derive(Debug, SimpleObject, InputObject)]
pub struct ExplanationOfBenefit_AddItemGraphql {
    _detail_sequence: Option<Vec<ElementGraphql>>,
    _factor: Option<ElementGraphql>,
    _item_sequence: Option<Vec<ElementGraphql>>,
    _note_number: Option<Vec<ElementGraphql>>,
    _serviced_date: Option<ElementGraphql>,
    _sub_detail_sequence: Option<Vec<ElementGraphql>>,
    adjudication: Option<Vec<ExplanationOfBenefit_AdjudicationGraphql>>,
    body_site: Option<CodeableConceptGraphql>,
    detail: Option<Vec<ExplanationOfBenefit_Detail1Graphql>>,
    detail_sequence: Option<Vec<i64>>,
    extension: Option<Vec<ExtensionGraphql>>,
    factor: Option<f64>,
    id: Option<String>,
    item_sequence: Option<Vec<i64>>,
    location_address: Option<AddressGraphql>,
    location_codeable_concept: Option<CodeableConceptGraphql>,
    location_reference: Option<ReferenceGraphql>,
    modifier: Option<Vec<CodeableConceptGraphql>>,
    modifier_extension: Option<Vec<ExtensionGraphql>>,
    net: Option<MoneyGraphql>,
    note_number: Option<Vec<i64>>,
    product_or_service: CodeableConceptGraphql,
    program_code: Option<Vec<CodeableConceptGraphql>>,
    provider: Option<Vec<ReferenceGraphql>>,
    quantity: Option<QuantityGraphql>,
    serviced_date: Option<String>,
    serviced_period: Option<PeriodGraphql>,
    sub_detail_sequence: Option<Vec<i64>>,
    sub_site: Option<Vec<CodeableConceptGraphql>>,
    unit_price: Option<MoneyGraphql>,
}
