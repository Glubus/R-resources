//! Code generators for different resource types

/// Flat access module (`r::`)
pub mod flat;

use std::collections::HashMap;

use super::types::ResourceValue;

/// Generates Rust code for all resources
///
/// # Arguments
///
/// * `resources` - `HashMap` of resource type to list of (name, value) pairs
///
/// # Returns
///
/// A String containing the generated Rust code
pub fn generate_code(resources: &HashMap<String, Vec<(String, ResourceValue)>>) -> String {
    let mut code = String::new();
    code.push_str(&flat::generate_r_module(resources));
    code.push_str(&generate_r_struct());
    code
}

pub fn generate_code_with_tests(
    resources: &HashMap<String, Vec<(String, ResourceValue)>>,
    test_resources: &HashMap<String, Vec<(String, ResourceValue)>>,
) -> String {
    let mut code = String::new();
    code.push_str(&flat::generate_r_module(resources));
    code.push_str(&flat::generate_r_tests_module(test_resources));
    code.push_str(&generate_r_struct());
    code
}

/// Generates an empty R struct (used when no resources file exists)
pub fn generate_empty_code() -> String {
    String::from(
        r"
pub mod r {}

pub struct R;

impl Default for R {
    fn default() -> Self {
        Self::new()
    }
}

impl R {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
",
    )
}

/// Generates the main R struct
fn generate_r_struct() -> String {
    String::from(
        r"
pub struct R;

impl Default for R {
    fn default() -> Self {
        Self::new()
    }
}

impl R {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
",
    )
}
