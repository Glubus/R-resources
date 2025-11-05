/// Code generation for array types
use crate::codegen::types::ResourceValue;
use crate::codegen::utils::sanitize_identifier;
use std::fmt::Write as _;

/// Generates the `string_array` module
pub fn generate_string_array_module(string_arrays: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod string_array {\n");
    use std::collections::BTreeMap;
    #[derive(Default)]
    struct Node<'a> { children: BTreeMap<String, Node<'a>>, items: Vec<(&'a str, Vec<String>)> }
    fn insert<'a>(root: &mut Node<'a>, path: &'a str, arr: Vec<String>) {
        let mut parts = path.split('/').filter(|s| !s.is_empty()).peekable();
        let mut node = root;
        while let Some(part) = parts.next() {
            if parts.peek().is_none() { node.items.push((part, arr)); break; } else {
                let key = sanitize_identifier(part); node = node.children.entry(key).or_default();
            }
        }
    }
    let mut root: Node = Default::default();
    for (name, value) in string_arrays { if let ResourceValue::StringArray(arr) = value { insert(&mut root, name, arr.clone()); } }
    fn emit_node(code: &mut String, node: &Node, indent: usize) {
        let pad = " ".repeat(indent);
        for (k, child) in &node.children { let _=writeln!(code, "{}pub mod {} {{", pad, k); emit_node(code, child, indent+4); let _=writeln!(code, "{}}}", pad); }
        for (leaf, arr) in &node.items {
            let items: Vec<String> = arr.iter().map(|s| format!("\"{}\"", s.escape_debug())).collect();
            let _ = writeln!(code, "{}pub const {}: &[&str] = &[{}];", pad, sanitize_identifier(leaf).to_uppercase(), items.join(", "));
        }
    }
    emit_node(&mut code, &root, 4);
    code.push_str("}\n");
    code
}

/// Generates the `int_array` module
pub fn generate_int_array_module(int_arrays: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod int_array {\n");
    use std::collections::BTreeMap;
    #[derive(Default)]
    struct Node<'a> { children: BTreeMap<String, Node<'a>>, items: Vec<(&'a str, Vec<i64>)> }
    fn insert<'a>(root: &mut Node<'a>, path: &'a str, arr: Vec<i64>) {
        let mut parts = path.split('/').filter(|s| !s.is_empty()).peekable();
        let mut node = root;
        while let Some(part) = parts.next() {
            if parts.peek().is_none() { node.items.push((part, arr)); break; } else {
                let key = sanitize_identifier(part); node = node.children.entry(key).or_default();
            }
        }
    }
    let mut root: Node = Default::default();
    for (name, value) in int_arrays { if let ResourceValue::IntArray(arr) = value { insert(&mut root, name, arr.clone()); } }
    fn emit_node(code: &mut String, node: &Node, indent: usize) {
        let pad = " ".repeat(indent);
        for (k, child) in &node.children { let _=writeln!(code, "{}pub mod {} {{", pad, k); emit_node(code, child, indent+4); let _=writeln!(code, "{}}}", pad); }
        for (leaf, arr) in &node.items { let items: Vec<String> = arr.iter().map(std::string::ToString::to_string).collect(); let _ = writeln!(code, "{}pub const {}: &[i64] = &[{}];", pad, sanitize_identifier(leaf).to_uppercase(), items.join(", ")); }
    }
    emit_node(&mut code, &root, 4);
    code.push_str("}\n");
    code
}

/// Generates the `float_array` module
pub fn generate_float_array_module(float_arrays: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod float_array {\n");
    use std::collections::BTreeMap;
    #[derive(Default)]
    struct Node<'a> { children: BTreeMap<String, Node<'a>>, items: Vec<(&'a str, Vec<f64>)> }
    fn insert<'a>(root: &mut Node<'a>, path: &'a str, arr: Vec<f64>) {
        let mut parts = path.split('/').filter(|s| !s.is_empty()).peekable();
        let mut node = root;
        while let Some(part) = parts.next() {
            if parts.peek().is_none() { node.items.push((part, arr)); break; } else {
                let key = sanitize_identifier(part); node = node.children.entry(key).or_default();
            }
        }
    }
    let mut root: Node = Default::default();
    for (name, value) in float_arrays { if let ResourceValue::FloatArray(arr) = value { insert(&mut root, name, arr.clone()); } }
    fn emit_node(code: &mut String, node: &Node, indent: usize) {
        let pad = " ".repeat(indent);
        for (k, child) in &node.children { let _=writeln!(code, "{}pub mod {} {{", pad, k); emit_node(code, child, indent+4); let _=writeln!(code, "{}}}", pad); }
        for (leaf, arr) in &node.items {
            let items: Vec<String> = arr.iter().map(|f| {
                let s = f.to_string();
                if s.contains('.') || s.contains('e') || s.contains('E') { s } else { format!("{s}.0") }
            }).collect();
            let _ = writeln!(code, "{}pub const {}: &[f64] = &[{}];", pad, sanitize_identifier(leaf).to_uppercase(), items.join(", "));
        }
    }
    emit_node(&mut code, &root, 4);
    code.push_str("}\n");
    code
}

