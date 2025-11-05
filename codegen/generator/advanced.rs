/// Code generation for advanced types (color, url, dimension)
use crate::codegen::references;
use crate::codegen::types::ResourceValue;
use crate::codegen::utils::sanitize_identifier;
use std::fmt::Write as _;

/// Generates the color module
pub fn generate_color_module(colors: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod color {\n");

    for (name, value) in colors {
        let const_name = sanitize_identifier(name).to_uppercase();
        
        match value {
            ResourceValue::Color(c) => {
                let _ = writeln!(
                    code,
                    "    pub const {}: &str = \"{}\";",
                    const_name,
                    c.escape_debug()
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

/// Generates the url module
pub fn generate_url_module(urls: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod url {\n");

    for (name, value) in urls {
        if let ResourceValue::Url(u) = value {
            let _ = writeln!(
                code,
                "    pub const {}: &str = \"{}\";",
                sanitize_identifier(name).to_uppercase(),
                u.escape_debug()
            );
        }
    }

    code.push_str("}\n");
    code
}

/// Generates the dimension module
pub fn generate_dimension_module(dimensions: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod dimension {\n");

    for (name, value) in dimensions {
        if let ResourceValue::Dimension(d) = value {
            let _ = writeln!(
                code,
                "    pub const {}: &str = \"{}\";",
                sanitize_identifier(name).to_uppercase(),
                d.escape_debug()
            );
        }
    }

    code.push_str("}\n");
    code
}

