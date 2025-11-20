/// Code generation for flat `r::` access module (Kotlin-style nested modules)
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;

use crate::codegen::types::{
    InterpolationPart, NumberValue, ResourceValue, Template, TemplateParameter,
    TemplateParameterType,
};
use crate::codegen::utils::sanitize_identifier;

struct ResourceEntry {
    resource_type: String,
    namespace_path: Vec<String>,
    leaf_name: String,
    value: ResourceValue,
}

#[derive(Default)]
struct NamespaceNode {
    children: BTreeMap<String, NamespaceNode>,
    resource_indices: Vec<usize>,
}

/// Generates a flat module `r` with nested namespace structure (Kotlin-style: r::auth::title)
pub fn generate_r_module(resources: &HashMap<String, Vec<(String, ResourceValue)>>) -> String {
    let entries = collect_entries(resources);
    let mut tree = build_namespace_tree(&entries);
    sort_namespace_tree(&mut tree, &entries);

    let needs_big_decimal = resources
        .get("number")
        .map(|items| {
            items.iter().any(|(_, value)| {
                matches!(
                    value,
                    ResourceValue::Number(NumberValue::BigDecimal(_))
                )
            })
        })
        .unwrap_or(false);

    let mut code = String::from("\npub mod r {\n");
    if needs_big_decimal {
        code.push_str("    use core::str::FromStr;\n");
        code.push_str("    use std::sync::LazyLock;\n");
        code.push_str("    use r_resources::BigDecimal;\n");
    }
    emit_namespace_tree(&mut code, &tree, &entries, resources, 4);
    code.push_str("}\n");
    code
}

pub fn generate_r_tests_module(
    test_resources: &HashMap<String, Vec<(String, ResourceValue)>>,
) -> String {
    let entries = collect_entries(test_resources);
    let mut tree = build_namespace_tree(&entries);
    sort_namespace_tree(&mut tree, &entries);

    let needs_big_decimal = test_resources
        .get("number")
        .map(|items| {
            items.iter().any(|(_, value)| {
                matches!(
                    value,
                    ResourceValue::Number(NumberValue::BigDecimal(_))
                )
            })
        })
        .unwrap_or(false);

    let mut code = String::from("\npub mod r_tests {\n");
    if needs_big_decimal {
        code.push_str("    use core::str::FromStr;\n");
        code.push_str("    use std::sync::LazyLock;\n");
        code.push_str("    use r_resources::BigDecimal;\n");
    }
    emit_namespace_tree(&mut code, &tree, &entries, test_resources, 4);
    code.push_str("}\n");
    code
}

fn collect_entries(
    resources: &HashMap<String, Vec<(String, ResourceValue)>>,
) -> Vec<ResourceEntry> {
    let mut entries = Vec::new();
    for (resource_type, items) in resources {
        for (name, value) in items {
            let (namespace_path, leaf_name) = split_path_to_namespace(name);
            if leaf_name.is_empty() {
                continue;
            }
            entries.push(ResourceEntry {
                resource_type: resource_type.to_string(),
                namespace_path,
                leaf_name,
                value: value.clone(),
            });
        }
    }
    entries
}

fn split_path_to_namespace(path: &str) -> (Vec<String>, String) {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.is_empty() {
        return (Vec::new(), String::new());
    }

    if parts.len() == 1 {
        return (Vec::new(), sanitize_identifier(parts[0]));
    }

    let leaf = sanitize_identifier(parts[parts.len() - 1]);
    let namespace_parts = parts[..parts.len() - 1]
        .iter()
        .map(|p| sanitize_identifier(p))
        .collect();

    (namespace_parts, leaf)
}

fn build_namespace_tree(entries: &[ResourceEntry]) -> NamespaceNode {
    let mut root = NamespaceNode::default();
    for (idx, entry) in entries.iter().enumerate() {
        let mut current = &mut root;
        for ns_part in &entry.namespace_path {
            current = current.children.entry(ns_part.clone()).or_default();
        }
        current.resource_indices.push(idx);
    }
    root
}

fn sort_namespace_tree(node: &mut NamespaceNode, entries: &[ResourceEntry]) {
    node.resource_indices
        .sort_by(|a, b| entries[*a].leaf_name.cmp(&entries[*b].leaf_name));
    for child in node.children.values_mut() {
        sort_namespace_tree(child, entries);
    }
}

fn emit_namespace_tree(
    code: &mut String,
    node: &NamespaceNode,
    entries: &[ResourceEntry],
    all_resources: &HashMap<String, Vec<(String, ResourceValue)>>,
    indent: usize,
) {
    let pad = " ".repeat(indent);
    for (ns_name, child) in &node.children {
        let _ = writeln!(code, "{}pub mod {} {{", pad, ns_name);
        emit_namespace_tree(code, child, entries, all_resources, indent + 4);
        let _ = writeln!(code, "{}}}", pad);
    }

    for idx in &node.resource_indices {
        if let Some(entry) = entries.get(*idx) {
            emit_resource(code, entry, all_resources, indent);
        }
    }
}

fn emit_resource(
    code: &mut String,
    entry: &ResourceEntry,
    all_resources: &HashMap<String, Vec<(String, ResourceValue)>>,
    indent: usize,
) {
    let pad = " ".repeat(indent);
    let const_name = sanitize_identifier(&entry.leaf_name).to_uppercase();
    match entry.resource_type.as_str() {
        "string" => emit_string_resource(code, &pad, &const_name, entry, all_resources),
        "number" => emit_number_resource(code, &pad, &const_name, entry),
        "bool" => emit_bool_resource(code, &pad, &const_name, entry),
        "color" | "url" | "dimension" => emit_text_resource(code, &pad, &const_name, entry),
        "string_array" => emit_string_array_resource(code, &pad, &const_name, entry),
        "int_array" => emit_int_array_resource(code, &pad, &const_name, entry),
        "float_array" => emit_float_array_resource(code, &pad, &const_name, entry),
        _ => {}
    }
}

fn emit_string_resource(
    code: &mut String,
    pad: &str,
    const_name: &str,
    entry: &ResourceEntry,
    all_resources: &HashMap<String, Vec<(String, ResourceValue)>>,
) {
    match &entry.value {
        ResourceValue::String(value) => emit_string_constant(code, pad, const_name, value),
        ResourceValue::Reference {
            resource_type,
            key,
        } => emit_reference_constant(code, pad, const_name, resource_type, key),
        ResourceValue::InterpolatedString(parts) => {
            let mut visited = HashSet::new();
            let resolved =
                resolve_interpolation_parts(parts, all_resources, &mut visited).unwrap_or_default();
            emit_string_constant(code, pad, const_name, &resolved);
        }
        ResourceValue::Template(template) => {
            let fn_name = sanitize_identifier(&entry.leaf_name).to_lowercase();
            emit_template_function(code, pad, &fn_name, template);
        }
        _ => {}
    }
}

fn emit_number_resource(code: &mut String, pad: &str, const_name: &str, entry: &ResourceEntry) {
    match &entry.value {
        ResourceValue::Number(value) => emit_number_value(code, pad, const_name, value),
        ResourceValue::Reference {
            resource_type,
            key,
        } => emit_reference_constant(code, pad, const_name, resource_type, key),
        _ => {}
    }
}

fn emit_number_value(
    code: &mut String,
    pad: &str,
    const_name: &str,
    number: &NumberValue,
) {
    match number {
        NumberValue::Int(value) => {
            let _ = writeln!(code, "{pad}pub const {const_name}: i64 = {value};");
        }
        NumberValue::Float(value) => {
            let formatted = format_float(*value);
            let _ = writeln!(code, "{pad}pub const {const_name}: f64 = {formatted};");
        }
        NumberValue::BigDecimal(raw) => emit_big_decimal_static(code, pad, const_name, raw),
        NumberValue::Typed { literal, ty } => {
            let _ = writeln!(
                code,
                "{pad}pub const {const_name}: {} = {};",
                ty.as_str(),
                literal
            );
        }
    }
}

fn emit_big_decimal_static(code: &mut String, pad: &str, const_name: &str, raw: &str) {
    let literal = escape_literal(raw);
    let _ = writeln!(
        code,
        "{pad}pub static {const_name}: LazyLock<BigDecimal> = LazyLock::new(|| {{"
    );
    let _ = writeln!(
        code,
        "{pad}    BigDecimal::from_str(\"{literal}\").expect(\"valid decimal literal\")"
    );
    let _ = writeln!(code, "{pad}}});");
}

fn emit_bool_resource(code: &mut String, pad: &str, const_name: &str, entry: &ResourceEntry) {
    match &entry.value {
        ResourceValue::Bool(value) => {
            let _ = writeln!(code, "{pad}pub const {const_name}: bool = {value};");
        }
        ResourceValue::Reference {
            resource_type,
            key,
        } => emit_reference_constant(code, pad, const_name, resource_type, key),
        _ => {}
    }
}

fn emit_text_resource(code: &mut String, pad: &str, const_name: &str, entry: &ResourceEntry) {
    match &entry.value {
        ResourceValue::Color(value)
        | ResourceValue::Url(value)
        | ResourceValue::Dimension(value) => {
            let _ = writeln!(
                code,
                "{pad}pub const {const_name}: &str = \"{}\";",
                value.escape_debug()
            );
        }
        ResourceValue::Reference {
            resource_type,
            key,
        } => emit_reference_constant(code, pad, const_name, resource_type, key),
        _ => {}
    }
}

fn emit_string_array_resource(code: &mut String, pad: &str, const_name: &str, entry: &ResourceEntry) {
    if let ResourceValue::StringArray(items) = &entry.value {
        let body = items
            .iter()
            .map(|s| format!("\"{}\"", s.escape_debug()))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(code, "{pad}pub const {const_name}: &[&str] = &[{body}];");
    }
}

fn emit_int_array_resource(code: &mut String, pad: &str, const_name: &str, entry: &ResourceEntry) {
    if let ResourceValue::IntArray(items) = &entry.value {
        let body = items
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(code, "{pad}pub const {const_name}: &[i64] = &[{body}];");
    }
}

fn emit_float_array_resource(code: &mut String, pad: &str, const_name: &str, entry: &ResourceEntry) {
    if let ResourceValue::FloatArray(items) = &entry.value {
        let body = items
            .iter()
            .map(|value| format_float(*value))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(code, "{pad}pub const {const_name}: &[f64] = &[{body}];");
    }
}

fn emit_string_constant(code: &mut String, pad: &str, const_name: &str, value: &str) {
    let _ = writeln!(
        code,
        "{pad}pub const {const_name}: &str = \"{}\";",
        value.escape_debug()
    );
}

fn emit_reference_constant(
    code: &mut String,
    pad: &str,
    const_name: &str,
    resource_type: &str,
    key: &str,
) {
    let target = crate::codegen::references::resolve_reference_path(resource_type, key, true);
    let _ = writeln!(code, "{pad}pub use {target} as {const_name};");
}

fn emit_template_function(code: &mut String, pad: &str, fn_name: &str, template: &Template) {
    let params = template
        .parameters
        .iter()
        .map(|param| format!("{}: {}", sanitize_identifier(&param.name), rust_type(param)))
        .collect::<Vec<_>>()
        .join(", ");
    let (format_str, format_args) =
        parse_template_placeholders(&template.template, &template.parameters);
    let _ = writeln!(code, "{pad}#[must_use]");
    let _ = writeln!(code, "{pad}pub fn {fn_name}({params}) -> String {{");
    if format_args.is_empty() {
        let _ = writeln!(code, "{pad}    \"{}\".to_string()", format_str.replace('\\', ""));
    } else {
        let args = format_args.join(", ");
        let _ = writeln!(code, "{pad}    format!(\"{format_str}\", {args})");
    }
    let _ = writeln!(code, "{pad}}}");
}

fn rust_type(param: &TemplateParameter) -> &'static str {
    match param.param_type {
        TemplateParameterType::String => "&str",
        TemplateParameterType::Int => "i64",
        TemplateParameterType::Float => "f64",
        TemplateParameterType::Bool => "bool",
    }
}

fn parse_template_placeholders(
    template_str: &str,
    parameters: &[TemplateParameter],
) -> (String, Vec<String>) {
    let mut format_parts = Vec::new();
    let mut format_args = Vec::new();
    let mut current_pos = 0;

    while let Some(start) = template_str[current_pos..].find('{') {
        let start_abs = current_pos + start;
        let text_before = &template_str[current_pos..start_abs];
        if !text_before.is_empty() {
            format_parts.push(text_before.escape_debug().to_string());
        }

        if let Some(end) = template_str[start_abs + 1..].find('}') {
            let end_abs = start_abs + 1 + end;
            let placeholder = &template_str[start_abs + 1..end_abs];
            if let Some(param) = parameters.iter().find(|p| p.name == placeholder) {
                let param_name = sanitize_identifier(&param.name);
                format_args.push(param_name.clone());
                format_parts.push("{}".to_string());
            } else {
                format_parts.push(format!("\\{{{placeholder}\\}}"));
            }
            current_pos = end_abs + 1;
        } else {
            format_parts.push("\\{".to_string());
            current_pos = start_abs + 1;
        }
    }

    let remaining = &template_str[current_pos..];
    if !remaining.is_empty() {
        format_parts.push(remaining.escape_debug().to_string());
    }

    (format_parts.join(""), format_args)
}

fn resolve_interpolation_parts(
    parts: &[InterpolationPart],
    all_resources: &HashMap<String, Vec<(String, ResourceValue)>>,
    visited: &mut HashSet<String>,
) -> Option<String> {
    let mut result = String::new();
    for part in parts {
        match part {
            InterpolationPart::Text(text) => result.push_str(text),
            InterpolationPart::Reference { resource_type, key } => {
                if let Some(resolved) =
                    resolve_string_value(resource_type, key, all_resources, visited)
                {
                    result.push_str(&resolved);
                } else {
                    result.push_str(&format!("@{}", key));
                }
            }
        }
    }
    Some(result)
}

fn resolve_string_value(
    resource_type: &str,
    key: &str,
    all_resources: &HashMap<String, Vec<(String, ResourceValue)>>,
    visited: &mut HashSet<String>,
) -> Option<String> {
    let ref_key = format!("{resource_type}:{key}");
    if !visited.insert(ref_key.clone()) {
        return None;
    }

    let result = if let Some(target_resources) = all_resources.get(resource_type) {
        if let Some((_, value)) = target_resources.iter().find(|(name, _)| name == key) {
            extract_string_from_value(value, all_resources, visited)
        } else {
            None
        }
    } else {
        None
    };

    visited.remove(&ref_key);
    result
}

fn extract_string_from_value(
    value: &ResourceValue,
    all_resources: &HashMap<String, Vec<(String, ResourceValue)>>,
    visited: &mut HashSet<String>,
) -> Option<String> {
    match value {
        ResourceValue::String(text) => Some(text.clone()),
        ResourceValue::Reference {
            resource_type,
            key,
        } => resolve_string_value(resource_type, key, all_resources, visited),
        ResourceValue::InterpolatedString(parts) => {
            resolve_interpolation_parts(parts, all_resources, visited)
        }
        _ => None,
    }
}

fn format_float(value: f64) -> String {
    let s = value.to_string();
    if s.contains('.') || s.contains('e') || s.contains('E') {
        s
    } else {
        format!("{s}.0")
    }
}

fn escape_literal(literal: &str) -> String {
    literal
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}
