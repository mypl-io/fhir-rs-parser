#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::CodeableConcept::CodeableConceptGraphql;
use crate::model::Extension::Extension;
use crate::model::Extension::ExtensionGraphql;
use crate::model::SubstancePolymer_StartingMaterial::SubstancePolymer_StartingMaterial;
use crate::model::SubstancePolymer_StartingMaterial::SubstancePolymer_StartingMaterialGraphql;
use async_graphql::*;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Todo.

#[derive(Debug)]
pub struct SubstancePolymer_MonomerSet<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstancePolymer_MonomerSet<'_> {
    pub fn new(value: &Value) -> SubstancePolymer_MonomerSet {
        SubstancePolymer_MonomerSet {
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

    /// Todo.
    pub fn ratio_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("ratioType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Todo.
    pub fn starting_material(&self) -> Option<Vec<SubstancePolymer_StartingMaterial>> {
        if let Some(Value::Array(val)) = self.value.get("startingMaterial") {
            return Some(
                val.into_iter()
                    .map(|e| SubstancePolymer_StartingMaterial {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
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
        if let Some(_val) = self.ratio_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.starting_material() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstancePolymer_MonomerSetBuilder {
    pub(crate) value: Value,
}

impl SubstancePolymer_MonomerSetBuilder {
    pub fn build(&self) -> SubstancePolymer_MonomerSet {
        SubstancePolymer_MonomerSet {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstancePolymer_MonomerSet) -> SubstancePolymer_MonomerSetBuilder {
        SubstancePolymer_MonomerSetBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstancePolymer_MonomerSetBuilder {
        let mut __value: Value = json!({});
        return SubstancePolymer_MonomerSetBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstancePolymer_MonomerSetBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstancePolymer_MonomerSetBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstancePolymer_MonomerSetBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn ratio_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstancePolymer_MonomerSetBuilder {
        self.value["ratioType"] = json!(val.value);
        return self;
    }

    pub fn starting_material<'a>(
        &'a mut self,
        val: Vec<SubstancePolymer_StartingMaterial>,
    ) -> &'a mut SubstancePolymer_MonomerSetBuilder {
        self.value["startingMaterial"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug, SimpleObject, InputObject)]
pub struct SubstancePolymer_MonomerSetGraphql {
    extension: Option<Vec<ExtensionGraphql>>,
    id: Option<String>,
    modifier_extension: Option<Vec<ExtensionGraphql>>,
    ratio_type: Option<CodeableConceptGraphql>,
    starting_material: Option<Vec<SubstancePolymer_StartingMaterialGraphql>>,
}