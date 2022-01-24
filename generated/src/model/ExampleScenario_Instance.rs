#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Element::ElementGraphql;
use crate::model::ExampleScenario_ContainedInstance::ExampleScenario_ContainedInstance;
use crate::model::ExampleScenario_ContainedInstance::ExampleScenario_ContainedInstanceGraphql;
use crate::model::ExampleScenario_Version::ExampleScenario_Version;
use crate::model::ExampleScenario_Version::ExampleScenario_VersionGraphql;
use crate::model::Extension::Extension;
use crate::model::Extension::ExtensionGraphql;
use async_graphql::*;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Example of workflow instance.

#[derive(Debug)]
pub struct ExampleScenario_Instance<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExampleScenario_Instance<'_> {
    pub fn new(value: &Value) -> ExampleScenario_Instance {
        ExampleScenario_Instance {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for resourceId
    pub fn _resource_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resourceId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for resourceType
    pub fn _resource_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resourceType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Resources contained in the instance (e.g. the observations contained in a
    /// bundle).
    pub fn contained_instance(&self) -> Option<Vec<ExampleScenario_ContainedInstance>> {
        if let Some(Value::Array(val)) = self.value.get("containedInstance") {
            return Some(
                val.into_iter()
                    .map(|e| ExampleScenario_ContainedInstance {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Human-friendly description of the resource instance.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
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

    /// A short name for the resource instance.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The id of the resource for referencing.
    pub fn resource_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resourceId") {
            return Some(string);
        }
        return None;
    }

    /// The type of the resource.
    pub fn resource_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resourceType") {
            return Some(string);
        }
        return None;
    }

    /// A specific version of the resource.
    pub fn version(&self) -> Option<Vec<ExampleScenario_Version>> {
        if let Some(Value::Array(val)) = self.value.get("version") {
            return Some(
                val.into_iter()
                    .map(|e| ExampleScenario_Version {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._resource_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._resource_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained_instance() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
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
        if let Some(_val) = self.resource_id() {}
        if let Some(_val) = self.resource_type() {}
        if let Some(_val) = self.version() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ExampleScenario_InstanceBuilder {
    pub(crate) value: Value,
}

impl ExampleScenario_InstanceBuilder {
    pub fn build(&self) -> ExampleScenario_Instance {
        ExampleScenario_Instance {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ExampleScenario_Instance) -> ExampleScenario_InstanceBuilder {
        ExampleScenario_InstanceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ExampleScenario_InstanceBuilder {
        let mut __value: Value = json!({});
        return ExampleScenario_InstanceBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _resource_id<'a>(&'a mut self, val: Element) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["_resourceId"] = json!(val.value);
        return self;
    }

    pub fn _resource_type<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["_resourceType"] = json!(val.value);
        return self;
    }

    pub fn contained_instance<'a>(
        &'a mut self,
        val: Vec<ExampleScenario_ContainedInstance>,
    ) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["containedInstance"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn resource_id<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["resourceId"] = json!(val);
        return self;
    }

    pub fn resource_type<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["resourceType"] = json!(val);
        return self;
    }

    pub fn version<'a>(
        &'a mut self,
        val: Vec<ExampleScenario_Version>,
    ) -> &'a mut ExampleScenario_InstanceBuilder {
        self.value["version"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug, SimpleObject, InputObject)]
pub struct ExampleScenario_InstanceGraphql {
    _description: Option<ElementGraphql>,
    _name: Option<ElementGraphql>,
    _resource_id: Option<ElementGraphql>,
    _resource_type: Option<ElementGraphql>,
    contained_instance: Option<Vec<ExampleScenario_ContainedInstanceGraphql>>,
    description: Option<String>,
    extension: Option<Vec<ExtensionGraphql>>,
    id: Option<String>,
    modifier_extension: Option<Vec<ExtensionGraphql>>,
    name: Option<String>,
    resource_id: Option<String>,
    resource_type: Option<String>,
    version: Option<Vec<ExampleScenario_VersionGraphql>>,
}