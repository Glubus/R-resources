/// Code generation for advanced types (color, url, dimension)
use crate::codegen::types::ResourceValue;
use crate::codegen::utils::sanitize_identifier;

/// Generates the color module
pub fn generate_color_module(colors: &[(String, ResourceValue)]) -> String {
    let mut code = String::from("\npub mod color {\n");

    for (name, value) in colors {
        if let ResourceValue::Color(c) = value {
            code.push_str(&format!(
                "    pub const {}: &str = \"{}\";\n",
                sanitize_identifier(name).to_uppercase(),
                c.escape_debug()
            ));
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
            code.push_str(&format!(
                "    pub const {}: &str = \"{}\";\n",
                sanitize_identifier(name).to_uppercase(),
                u.escape_debug()
            ));
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
            code.push_str(&format!(
                "    pub const {}: &str = \"{}\";\n",
                sanitize_identifier(name).to_uppercase(),
                d.escape_debug()
            ));
        }
    }

    code.push_str("}\n");
    code
}

