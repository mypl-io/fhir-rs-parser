#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Element::ElementGraphql;
use crate::model::Extension::Extension;
use crate::model::Extension::ExtensionGraphql;
use async_graphql::*;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Dependent<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl StructureMap_Dependent<'_> {
    pub fn new(value: &Value) -> StructureMap_Dependent {
        StructureMap_Dependent {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for variable
    pub fn _variable(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_variable") {
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

    /// Name of a rule or group to apply.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Variable to pass to the rule or group.
    pub fn variable(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("variable") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._variable() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.variable() {
            _val.into_iter().for_each(|_e| {});
        }
        return true;
    }
}

#[derive(Debug)]
pub struct StructureMap_DependentBuilder {
    pub(crate) value: Value,
}

impl StructureMap_DependentBuilder {
    pub fn build(&self) -> StructureMap_Dependent {
        StructureMap_Dependent {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: StructureMap_Dependent) -> StructureMap_DependentBuilder {
        StructureMap_DependentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> StructureMap_DependentBuilder {
        let mut __value: Value = json!({});
        return StructureMap_DependentBuilder { value: __value };
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_DependentBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _variable<'a>(&'a mut self, val: Vec<Element>) -> &'a mut StructureMap_DependentBuilder {
        self.value["_variable"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureMap_DependentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_DependentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureMap_DependentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_DependentBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn variable<'a>(&'a mut self, val: Vec<&str>) -> &'a mut StructureMap_DependentBuilder {
        self.value["variable"] = json!(val);
        return self;
    }
}

#[derive(Debug, SimpleObject, InputObject)]
pub struct StructureMap_DependentGraphql {
    _name: Option<ElementGraphql>,
    _variable: Option<Vec<ElementGraphql>>,
    extension: Option<Vec<ExtensionGraphql>>,
    id: Option<String>,
    modifier_extension: Option<Vec<ExtensionGraphql>>,
    name: Option<String>,
    variable: Option<Vec<String>>,
}
