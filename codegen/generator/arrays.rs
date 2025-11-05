/// Code generation for array types
use crate::codegen::types::ResourceValue;
use crate::codegen::utils::sanitize_identifier;
use std::fmt::Write as _;

/// Generates the `string_array` module
pub fn generate_string_array_module(string_arrays: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod string_array {\n");

    for (name, value) in string_arrays {
        if let ResourceValue::StringArray(arr) = value {
            let items: Vec<String> = arr
                .iter()
                .map(|s| format!("\"{}\"", s.escape_debug()))
                .collect();
            let _ = writeln!(
                code,
                "    pub const {}: &[&str] = &[{}];",
                sanitize_identifier(name).to_uppercase(),
                items.join(", ")
            );
        }
    }

    code.push_str("}\n");
    code
}

/// Generates the `int_array` module
pub fn generate_int_array_module(int_arrays: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod int_array {\n");

    for (name, value) in int_arrays {
        if let ResourceValue::IntArray(arr) = value {
            let items: Vec<String> = arr.iter().map(std::string::ToString::to_string).collect();
            let _ = writeln!(
                code,
                "    pub const {}: &[i64] = &[{}];",
                sanitize_identifier(name).to_uppercase(),
                items.join(", ")
            );
        }
    }

    code.push_str("}\n");
    code
}

/// Generates the `float_array` module
pub fn generate_float_array_module(float_arrays: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod float_array {\n");

    for (name, value) in float_arrays {
        if let ResourceValue::FloatArray(arr) = value {
            let items: Vec<String> = arr.iter().map(std::string::ToString::to_string).collect();
            let _ = writeln!(
                code,
                "    pub const {}: &[f64] = &[{}];",
                sanitize_identifier(name).to_uppercase(),
                items.join(", ")
            );
        }
    }

    code.push_str("}\n");
    code
}

