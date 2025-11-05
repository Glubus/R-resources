/// Code generation for flat `r::` access module
use std::collections::HashMap;

use crate::codegen::types::ResourceValue;
use crate::codegen::utils::sanitize_identifier;
use std::fmt::Write as _;

/// Generates a flat module `r` with all resources accessible directly
pub fn generate_r_module(resources: &HashMap<String, Vec<(String, ResourceValue)>>) -> String {
    let mut code = String::from("\n/// Flat access to all resources via `r::RESOURCE_NAME`\npub mod r {\n");

    // Re-export all resource types
    export_resources(&mut code, resources, "string");
    export_resources(&mut code, resources, "int");
    export_resources(&mut code, resources, "float");
    export_resources(&mut code, resources, "bool");
    export_resources(&mut code, resources, "color");
    export_resources(&mut code, resources, "url");
    export_resources(&mut code, resources, "dimension");
    export_resources(&mut code, resources, "string_array");
    export_resources(&mut code, resources, "int_array");
    export_resources(&mut code, resources, "float_array");

    code.push_str("}\n");
    // typed flat module
    let mut typed = String::from("\n/// Flat access for typed resources via `r_t::RESOURCE_NAME`\npub mod r_t {\n");
    export_resources(&mut typed, resources, "color_t");
    export_resources(&mut typed, resources, "url_t");
    typed.push_str("}\n");

    code + &typed
}

/// Helper to export resources of a given type
fn export_resources(
    code: &mut String,
    resources: &HashMap<String, Vec<(String, ResourceValue)>>,
    resource_type: &str,
) {
    if let Some(items) = resources.get(resource_type) {
        for (name, _) in items {
            if name.is_empty() {
                eprintln!("Warning: skipping empty resource name in type '{resource_type}'");
                continue;
            }
            // Split name into path and leaf; build module path for the item
            let mut parts = name.split('/').filter(|s| !s.is_empty()).peekable();
            let mut module_path = String::new();
            let mut leaf = "";
            while let Some(part) = parts.next() {
                if parts.peek().is_none() {
                    leaf = part;
                } else {
                    if !module_path.is_empty() { module_path.push_str("::"); }
                    module_path.push_str(&sanitize_identifier(part));
                }
            }
            let alias = sanitize_identifier(name).to_uppercase();
            if module_path.is_empty() {
                let _ = writeln!(code, "    pub use crate::{resource_type}::{} as {alias};", sanitize_identifier(leaf).to_uppercase());
            } else {
                let _ = writeln!(code, "    pub use crate::{resource_type}::{module_path}::{} as {alias};", sanitize_identifier(leaf).to_uppercase());
            }
        }
    }
}

