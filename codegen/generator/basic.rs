/// Code generation for basic types (string, int, float, bool)
use crate::codegen::references;
use crate::codegen::types::ResourceValue;
use crate::codegen::utils::sanitize_identifier;
use std::fmt::Write as _;

/// Generates the string module
pub fn generate_string_module(strings: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod string {\n");

    for (name, value) in strings {
        let const_name = sanitize_identifier(name).to_uppercase();
        
        match value {
            ResourceValue::String(s) => {
                // For v0.3.0: strings with embedded references are kept as-is
                // TODO v0.4.0: Add proper interpolation support
                let _ = writeln!(
                    code,
                    "    pub const {}: &str = \"{}\";",
                    const_name,
                    s.escape_debug()
                );
            }
            ResourceValue::Reference { resource_type, key } => {
                // Generate a reference to another resource
                let target = references::resolve_reference_path(resource_type, key, true);
                let _ = writeln!(code, "    pub const {const_name}: &str = {target};");
            }
            _ => {}
        }
    }

    code.push_str("}\n");
    code
}

/// Generates the int module
pub fn generate_int_module(ints: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod int {\n");

    for (name, value) in ints {
        if let ResourceValue::Int(i) = value {
            let _ = writeln!(
                code,
                "    pub const {}: i64 = {};",
                sanitize_identifier(name).to_uppercase(),
                i
            );
        }
    }

    code.push_str("}\n");
    code
}

/// Generates the float module
pub fn generate_float_module(floats: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod float {\n");

    for (name, value) in floats {
        if let ResourceValue::Float(f) = value {
            let _ = writeln!(
                code,
                "    pub const {}: f64 = {};",
                sanitize_identifier(name).to_uppercase(),
                f
            );
        }
    }

    code.push_str("}\n");
    code
}

/// Generates the bool module
pub fn generate_bool_module(bools: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod bool {\n");

    for (name, value) in bools {
        if let ResourceValue::Bool(b) = value {
            let _ = writeln!(
                code,
                "    pub const {}: bool = {};",
                sanitize_identifier(name).to_uppercase(),
                b
            );
        }
    }

    code.push_str("}\n");
    code
}

