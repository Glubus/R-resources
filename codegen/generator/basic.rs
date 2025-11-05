/// Code generation for basic types (string, int, float, bool)
use crate::codegen::types::ResourceValue;
use crate::codegen::utils::sanitize_identifier;

/// Generates the string module
pub fn generate_string_module(strings: &[(String, ResourceValue)]) -> String {
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
pub fn generate_int_module(ints: &[(String, ResourceValue)]) -> String {
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
pub fn generate_float_module(floats: &[(String, ResourceValue)]) -> String {
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

/// Generates the bool module
pub fn generate_bool_module(bools: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod bool {\n");

    for (name, value) in bools {
        if let ResourceValue::Bool(b) = value {
            code.push_str(&format!(
                "    pub const {}: bool = {};\n",
                sanitize_identifier(name).to_uppercase(),
                b
            ));
        }
    }

    code.push_str("}\n");
    code
}

