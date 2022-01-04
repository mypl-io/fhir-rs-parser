#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::CodeableConcept::CodeableConceptGraphql;
use crate::model::Element::Element;
use crate::model::Element::ElementGraphql;
use crate::model::Extension::Extension;
use crate::model::Extension::ExtensionGraphql;
use crate::model::Quantity::Quantity;
use crate::model::Quantity::QuantityGraphql;
use async_graphql::*;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_DrugCharacteristic<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_DrugCharacteristic<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_DrugCharacteristic {
        MedicationKnowledge_DrugCharacteristic {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for valueBase64Binary
    pub fn _value_base_6_4_binary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBase64Binary") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueString
    pub fn _value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// A code specifying which characteristic of the medicine is being described (for
    /// example, colour, shape, imprint).
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Description of the characteristic.
    pub fn value_base_6_4_binary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueBase64Binary") {
            return Some(string);
        }
        return None;
    }

    /// Description of the characteristic.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Description of the characteristic.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Description of the characteristic.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value_base_6_4_binary() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_string() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_base_6_4_binary() {}
        if let Some(_val) = self.value_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_string() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationKnowledge_DrugCharacteristicBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_DrugCharacteristicBuilder {
    pub fn build(&self) -> MedicationKnowledge_DrugCharacteristic {
        MedicationKnowledge_DrugCharacteristic {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationKnowledge_DrugCharacteristic,
    ) -> MedicationKnowledge_DrugCharacteristicBuilder {
        MedicationKnowledge_DrugCharacteristicBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationKnowledge_DrugCharacteristicBuilder {
        let mut __value: Value = json!({});
        return MedicationKnowledge_DrugCharacteristicBuilder { value: __value };
    }

    pub fn _value_base_6_4_binary<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicationKnowledge_DrugCharacteristicBuilder {
        self.value["_valueBase64Binary"] = json!(val.value);
        return self;
    }

    pub fn _value_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicationKnowledge_DrugCharacteristicBuilder {
        self.value["_valueString"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_DrugCharacteristicBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationKnowledge_DrugCharacteristicBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_DrugCharacteristicBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationKnowledge_DrugCharacteristicBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn value_base_6_4_binary<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationKnowledge_DrugCharacteristicBuilder {
        self.value["valueBase64Binary"] = json!(val);
        return self;
    }

    pub fn value_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationKnowledge_DrugCharacteristicBuilder {
        self.value["valueCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn value_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut MedicationKnowledge_DrugCharacteristicBuilder {
        self.value["valueQuantity"] = json!(val.value);
        return self;
    }

    pub fn value_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicationKnowledge_DrugCharacteristicBuilder {
        self.value["valueString"] = json!(val);
        return self;
    }
}

#[derive(Debug, SimpleObject, InputObject)]
pub struct MedicationKnowledge_DrugCharacteristicGraphql {
    _value_base_6_4_binary: Option<ElementGraphql>,
    _value_string: Option<ElementGraphql>,
    extension: Option<Vec<ExtensionGraphql>>,
    id: Option<String>,
    modifier_extension: Option<Vec<ExtensionGraphql>>,
    fhir_type: Option<CodeableConceptGraphql>,
    value_base_6_4_binary: Option<String>,
    value_codeable_concept: Option<CodeableConceptGraphql>,
    value_quantity: Option<QuantityGraphql>,
    value_string: Option<String>,
}
