/// Code generation for basic types (string, int, float, bool)
use crate::codegen::references;
use crate::codegen::types::ResourceValue;
use crate::codegen::utils::sanitize_identifier;
use std::fmt::Write as _;

/// Generates the string module
pub fn generate_string_module(strings: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod string {\n");

    // Build a namespace tree from names like "a/b/c"
    use std::collections::BTreeMap;
    #[derive(Default)]
    struct Node<'a> {
        children: BTreeMap<String, Node<'a>>,
        items: Vec<(&'a str, &'a ResourceValue)>, // (leaf_name, value)
    }

    fn insert<'a>(root: &mut Node<'a>, path: &'a str, value: &'a ResourceValue) {
        let mut parts = path.split('/').filter(|s| !s.is_empty()).peekable();
        let mut node = root;
        while let Some(part) = parts.next() {
            if parts.peek().is_none() {
                node.items.push((part, value));
            } else {
                let key = sanitize_identifier(part);
                node = node.children.entry(key).or_default();
            }
        }
    }

    let mut root: Node = Default::default();
    for (name, value) in strings {
        insert(&mut root, name, value);
    }

    fn emit_node(code: &mut String, node: &Node, indent: usize) {
        let pad = " ".repeat(indent);
        for (mod_name, child) in &node.children {
            let _ = writeln!(code, "{}pub mod {} {{", pad, mod_name);
            emit_node(code, child, indent + 4);
            let _ = writeln!(code, "{}}}", pad);
        }
        for (leaf, value) in &node.items {
            let const_name = sanitize_identifier(leaf).to_uppercase();
            match *value {
                ResourceValue::String(ref s) => {
                    let _ = writeln!(
                        code,
                        "{}pub const {}: &str = \"{}\";",
                        pad,
                        const_name,
                        s.escape_debug()
                    );
                }
                ResourceValue::Reference { ref resource_type, ref key } => {
                    let target = references::resolve_reference_path(resource_type, key, true);
                    let _ = writeln!(code, "{}pub const {}: &str = {target};", pad, const_name);
                }
                _ => {}
            }
        }
    }

    emit_node(&mut code, &root, 4);

    code.push_str("}\n");
    code
}

/// Generates the int module
pub fn generate_int_module(ints: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod int {\n");
    use std::collections::BTreeMap;
    #[derive(Default)]
    struct Node<'a> { children: BTreeMap<String, Node<'a>>, items: Vec<(&'a str, i64)> }
    fn insert<'a>(root: &mut Node<'a>, path: &'a str, val: i64) {
        let mut parts = path.split('/').filter(|s| !s.is_empty()).peekable();
        let mut node = root;
        while let Some(part) = parts.next() {
            if parts.peek().is_none() { node.items.push((part, val)); } else {
                let key = sanitize_identifier(part); node = node.children.entry(key).or_default();
            }
        }
    }
    let mut root: Node = Default::default();
    for (name, value) in ints { if let ResourceValue::Int(i) = value { insert(&mut root, name, *i); } }
    fn emit_node(code: &mut String, node: &Node, indent: usize) {
        let pad = " ".repeat(indent);
        for (k, child) in &node.children { let _=writeln!(code, "{}pub mod {} {{", pad, k); emit_node(code, child, indent+4); let _=writeln!(code, "{}}}", pad); }
        for (leaf, v) in &node.items { let _=writeln!(code, "{}pub const {}: i64 = {};", pad, sanitize_identifier(leaf).to_uppercase(), v); }
    }
    emit_node(&mut code, &root, 4);
    code.push_str("}\n");
    code
}

/// Generates the float module
pub fn generate_float_module(floats: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod float {\n");
    use std::collections::BTreeMap;
    #[derive(Default)]
    struct Node<'a> { children: BTreeMap<String, Node<'a>>, items: Vec<(&'a str, f64)> }
    fn insert<'a>(root: &mut Node<'a>, path: &'a str, val: f64) {
        let mut parts = path.split('/').filter(|s| !s.is_empty()).peekable();
        let mut node = root;
        while let Some(part) = parts.next() {
            if parts.peek().is_none() { node.items.push((part, val)); } else {
                let key = sanitize_identifier(part); node = node.children.entry(key).or_default();
            }
        }
    }
    let mut root: Node = Default::default();
    for (name, value) in floats { if let ResourceValue::Float(f) = value { insert(&mut root, name, *f); } }
    fn emit_node(code: &mut String, node: &Node, indent: usize) {
        let pad = " ".repeat(indent);
        for (k, child) in &node.children { let _=writeln!(code, "{}pub mod {} {{", pad, k); emit_node(code, child, indent+4); let _=writeln!(code, "{}}}", pad); }
        for (leaf, v) in &node.items { let _=writeln!(code, "{}pub const {}: f64 = {};", pad, sanitize_identifier(leaf).to_uppercase(), v); }
    }
    emit_node(&mut code, &root, 4);
    code.push_str("}\n");
    code
}

/// Generates the bool module
pub fn generate_bool_module(bools: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod bool {\n");
    use std::collections::BTreeMap;
    #[derive(Default)]
    struct Node<'a> { children: BTreeMap<String, Node<'a>>, items: Vec<(&'a str, bool)> }
    fn insert<'a>(root: &mut Node<'a>, path: &'a str, val: bool) {
        let mut parts = path.split('/').filter(|s| !s.is_empty()).peekable();
        let mut node = root;
        while let Some(part) = parts.next() {
            if parts.peek().is_none() { node.items.push((part, val)); } else {
                let key = sanitize_identifier(part); node = node.children.entry(key).or_default();
            }
        }
    }
    let mut root: Node = Default::default();
    for (name, value) in bools { if let ResourceValue::Bool(b) = value { insert(&mut root, name, *b); } }
    fn emit_node(code: &mut String, node: &Node, indent: usize) {
        let pad = " ".repeat(indent);
        for (k, child) in &node.children { let _=writeln!(code, "{}pub mod {} {{", pad, k); emit_node(code, child, indent+4); let _=writeln!(code, "{}}}", pad); }
        for (leaf, v) in &node.items { let _=writeln!(code, "{}pub const {}: bool = {};", pad, sanitize_identifier(leaf).to_uppercase(), v); }
    }
    emit_node(&mut code, &root, 4);
    code.push_str("}\n");
    code
}

