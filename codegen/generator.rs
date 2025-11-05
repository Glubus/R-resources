use std::collections::HashMap;

use super::types::ResourceValue;

/// Generates Rust code for all resources
///
/// # Arguments
///
/// * `resources` - HashMap of resource type to list of (name, value) pairs
///
/// # Returns
///
/// A String containing the generated Rust code
pub fn generate_code(resources: &HashMap<String, Vec<(String, ResourceValue)>>) -> String {
    let mut code = String::new();
    
    // Generate each module
    if let Some(strings) = resources.get("string") {
        code.push_str(&generate_string_module(strings));
    }
    
    if let Some(ints) = resources.get("int") {
        code.push_str(&generate_int_module(ints));
    }
    
    if let Some(floats) = resources.get("float") {
        code.push_str(&generate_float_module(floats));
    }
    
    if let Some(string_arrays) = resources.get("string_array") {
        code.push_str(&generate_string_array_module(string_arrays));
    }
    
    if let Some(int_arrays) = resources.get("int_array") {
        code.push_str(&generate_int_array_module(int_arrays));
    }
    
    if let Some(float_arrays) = resources.get("float_array") {
        code.push_str(&generate_float_array_module(float_arrays));
    }
    
    // Generate flat r module with all resources
    code.push_str(&generate_r_module(resources));
    
    // Generate main R struct
    code.push_str(&generate_r_struct());
    
    code
}

/// Generates an empty R struct (used when no resources file exists)
pub fn generate_empty_code() -> String {
    String::from(
        r#"
pub struct R;

impl Default for R {
    fn default() -> Self {
        Self::new()
    }
}

impl R {
    pub const fn new() -> Self {
        R
    }
}
"#,
    )
}

/// Generates the string module
fn generate_string_module(strings: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod string {\n");

    for (name, value) in strings {
        if let ResourceValue::String(s) = value {
            code.push_str(&format!(
                "    pub const {}: &str = \"{}\";\n",
                sanitize_identifier(name).to_uppercase(),
                s.escape_debug()
            ));
        }
    }

    code.push_str("}\n");
    code
}

/// Generates the int module
fn generate_int_module(ints: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod int {\n");

    for (name, value) in ints {
        if let ResourceValue::Int(i) = value {
            code.push_str(&format!(
                "    pub const {}: i64 = {};\n",
                sanitize_identifier(name).to_uppercase(),
                i
            ));
        }
    }

    code.push_str("}\n");
    code
}

/// Generates the float module
fn generate_float_module(floats: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod float {\n");

    for (name, value) in floats {
        if let ResourceValue::Float(f) = value {
            code.push_str(&format!(
                "    pub const {}: f64 = {};\n",
                sanitize_identifier(name).to_uppercase(),
                f
            ));
        }
    }

    code.push_str("}\n");
    code
}

/// Generates the string_array module
fn generate_string_array_module(string_arrays: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod string_array {\n");

    for (name, value) in string_arrays {
        if let ResourceValue::StringArray(arr) = value {
            let items: Vec<String> = arr
                .iter()
                .map(|s| format!("\"{}\"", s.escape_debug()))
                .collect();
            code.push_str(&format!(
                "    pub const {}: &[&str] = &[{}];\n",
                sanitize_identifier(name).to_uppercase(),
                items.join(", ")
            ));
        }
    }

    code.push_str("}\n");
    code
}

/// Generates the int_array module
fn generate_int_array_module(int_arrays: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod int_array {\n");

    for (name, value) in int_arrays {
        if let ResourceValue::IntArray(arr) = value {
            let items: Vec<String> = arr.iter().map(|i| i.to_string()).collect();
            code.push_str(&format!(
                "    pub const {}: &[i64] = &[{}];\n",
                sanitize_identifier(name).to_uppercase(),
                items.join(", ")
            ));
        }
    }

    code.push_str("}\n");
    code
}

/// Generates the float_array module
fn generate_float_array_module(float_arrays: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod float_array {\n");

    for (name, value) in float_arrays {
        if let ResourceValue::FloatArray(arr) = value {
            let items: Vec<String> = arr.iter().map(|f| f.to_string()).collect();
            code.push_str(&format!(
                "    pub const {}: &[f64] = &[{}];\n",
                sanitize_identifier(name).to_uppercase(),
                items.join(", ")
            ));
        }
    }

    code.push_str("}\n");
    code
}

/// Generates a flat module `r` with all resources accessible directly
fn generate_r_module(resources: &HashMap<String, Vec<(String, ResourceValue)>>) -> String {
    let mut code = String::from("\n/// Flat access to all resources via r::RESOURCE_NAME\npub mod r {\n");
    
    // Re-export strings
    if let Some(strings) = resources.get("string") {
        for (name, _) in strings {
            let sanitized = sanitize_identifier(name).to_uppercase();
            code.push_str(&format!(
                "    pub use crate::string::{};\n",
                sanitized
            ));
        }
    }
    
    // Re-export ints
    if let Some(ints) = resources.get("int") {
        for (name, _) in ints {
            let sanitized = sanitize_identifier(name).to_uppercase();
            code.push_str(&format!(
                "    pub use crate::int::{};\n",
                sanitized
            ));
        }
    }
    
    // Re-export floats
    if let Some(floats) = resources.get("float") {
        for (name, _) in floats {
            let sanitized = sanitize_identifier(name).to_uppercase();
            code.push_str(&format!(
                "    pub use crate::float::{};\n",
                sanitized
            ));
        }
    }
    
    // Re-export string arrays
    if let Some(string_arrays) = resources.get("string_array") {
        for (name, _) in string_arrays {
            let sanitized = sanitize_identifier(name).to_uppercase();
            code.push_str(&format!(
                "    pub use crate::string_array::{};\n",
                sanitized
            ));
        }
    }
    
    // Re-export int arrays
    if let Some(int_arrays) = resources.get("int_array") {
        for (name, _) in int_arrays {
            let sanitized = sanitize_identifier(name).to_uppercase();
            code.push_str(&format!(
                "    pub use crate::int_array::{};\n",
                sanitized
            ));
        }
    }
    
    // Re-export float arrays
    if let Some(float_arrays) = resources.get("float_array") {
        for (name, _) in float_arrays {
            let sanitized = sanitize_identifier(name).to_uppercase();
            code.push_str(&format!(
                "    pub use crate::float_array::{};\n",
                sanitized
            ));
        }
    }
    
    code.push_str("}\n");
    code
}

/// Generates the main R struct
fn generate_r_struct() -> String {
    String::from(
        r#"
pub struct R;

impl Default for R {
    fn default() -> Self {
        Self::new()
    }
}

impl R {
    pub const fn new() -> Self {
        R
    }
}
"#,
    )
}

/// Sanitizes an identifier to be a valid Rust identifier
///
/// Replaces non-alphanumeric characters (except underscores) with underscores
fn sanitize_identifier(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
