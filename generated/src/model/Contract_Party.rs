#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::CodeableConcept::CodeableConceptGraphql;
use crate::model::Extension::Extension;
use crate::model::Extension::ExtensionGraphql;
use crate::model::Reference::Reference;
use crate::model::Reference::ReferenceGraphql;
use async_graphql::*;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_Party<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Contract_Party<'_> {
    pub fn new(value: &Value) -> Contract_Party {
        Contract_Party {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Participant in the offer.
    pub fn reference(&self) -> Vec<Reference> {
        self.value
            .get("reference")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Reference {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// How the party participates in the offer.
    pub fn role(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["role"]),
        }
    }

    pub fn validate(&self) -> bool {
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
        if !self
            .reference()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if !self.role().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Contract_PartyBuilder {
    pub(crate) value: Value,
}

impl Contract_PartyBuilder {
    pub fn build(&self) -> Contract_Party {
        Contract_Party {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Contract_Party) -> Contract_PartyBuilder {
        Contract_PartyBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(reference: Vec<Reference>, role: CodeableConcept) -> Contract_PartyBuilder {
        let mut __value: Value = json!({});
        __value["reference"] = json!(reference.into_iter().map(|e| e.value).collect::<Vec<_>>());
        __value["role"] = json!(role.value);
        return Contract_PartyBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Contract_PartyBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Contract_PartyBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Contract_PartyBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug, SimpleObject, InputObject)]
pub struct Contract_PartyGraphql {
    extension: Option<Vec<ExtensionGraphql>>,
    id: Option<String>,
    modifier_extension: Option<Vec<ExtensionGraphql>>,
    reference: Vec<ReferenceGraphql>,
    role: CodeableConceptGraphql,
}