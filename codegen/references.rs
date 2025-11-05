/// Reference resolution for resources
use std::collections::HashMap;

use super::types::ResourceValue;
use super::utils::sanitize_identifier;

/// Resolves a reference to its target resource path
///
/// Example: Reference { `resource_type`: "string", key: "`app_name`" }
/// Returns: "`string::APP_NAME`" or "`crate::string::APP_NAME`"
pub fn resolve_reference_path(resource_type: &str, key: &str, use_crate_prefix: bool) -> String {
    let sanitized_key = sanitize_identifier(key).to_uppercase();
    if use_crate_prefix {
        format!("crate::{resource_type}::{sanitized_key}")
    } else {
        format!("{resource_type}::{sanitized_key}")
    }
}

/// Validates that all references point to existing resources
pub fn validate_references(
    resources: &HashMap<String, Vec<(String, ResourceValue)>>,
) -> Vec<String> {
    let mut errors = Vec::new();

    for (res_type, items) in resources {
        for (name, value) in items {
            if let ResourceValue::Reference { resource_type, key } = value {
                // Check if the referenced resource exists
                if let Some(target_resources) = resources.get(resource_type) {
                    let key_exists = target_resources.iter().any(|(n, _)| n == key);
                    if !key_exists {
                        errors.push(format!(
                            "Unresolved reference in {res_type}.{name}: @{resource_type}/{key} does not exist"
                        ));
                    }
                } else {
                    errors.push(format!(
                        "Invalid reference in {res_type}.{name}: resource type '{resource_type}' does not exist"
                    ));
                }
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_reference_path() {
        assert_eq!(
            resolve_reference_path("string", "app_name", false),
            "string::APP_NAME"
        );
        assert_eq!(
            resolve_reference_path("color", "primary", true),
            "crate::color::PRIMARY"
        );
    }
}

