use crate::generator::ir::types::ResourceType;
use crate::generator::ir::{
    ResourceKey, ResourceKind, ResourceNode, ResourceOrigin,
    ResourceValue,
};
use crate::generator::parsing::{ParsedResource, ScalarValue};
use crate::generator::utils::{format_doc, sanitize_identifier};

pub struct StringType;

impl ResourceType for StringType {
    fn name(&self) -> &'static str {
        "string"
    }

    fn xml_tags(&self) -> &'static [&'static str] {
        &["string"]
    }

    fn resource_kind(&self) -> crate::generator::ir::ResourceKind {
        ResourceKind::String
    }

    fn build_node(
        &self,
        parsed: &ParsedResource,
        origin: ResourceOrigin,
    ) -> Option<ResourceNode> {
        if let ScalarValue::Text(value) = &parsed.value {
            Some(ResourceNode {
                kind: ResourceKind::String,
                value: ResourceValue::String(value.clone()),
                origin,
                doc: parsed.doc.clone(),
            })
        } else {
            None
        }
    }

    fn emit_rust(
        &self,
        key: &ResourceKey,
        node: &ResourceNode,
        indent: usize,
    ) -> Option<String> {
        if let ResourceValue::String(value) = &node.value {
            let pad = " ".repeat(indent);
            let const_name =
                sanitize_identifier(&key.name).to_uppercase();
            let escaped = value.escape_debug();
            let doc_str = format_doc(&node.doc, indent);
            Some(format!("{doc_str}{pad}pub const {const_name}: &str = \"{escaped}\";\n"))
        } else {
            None
        }
    }
}
