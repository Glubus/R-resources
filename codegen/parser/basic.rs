use std::collections::HashMap;

use crate::codegen::types::ResourceValue;

pub fn handle_string(
    text: &str,
    current_name: &str,
    resources: &mut HashMap<String, Vec<(String, ResourceValue)>>,
) {
    let value = ResourceValue::parse_string_value(text);
    resources
        .entry("string".to_string())
        .or_default()
        .push((current_name.to_string(), value));
}

pub fn handle_int(
    text: &str,
    current_name: &str,
    resources: &mut HashMap<String, Vec<(String, ResourceValue)>>,
) {
    if let Ok(val) = text.parse::<i64>() {
        resources
            .entry("int".to_string())
            .or_default()
            .push((current_name.to_string(), ResourceValue::Int(val)));
    }
}

pub fn handle_float(
    text: &str,
    current_name: &str,
    resources: &mut HashMap<String, Vec<(String, ResourceValue)>>,
) {
    if let Ok(val) = text.parse::<f64>() {
        resources
            .entry("float".to_string())
            .or_default()
            .push((current_name.to_string(), ResourceValue::Float(val)));
    }
}

pub fn handle_bool(
    text: &str,
    current_name: &str,
    resources: &mut HashMap<String, Vec<(String, ResourceValue)>>,
) {
    if let Ok(val) = text.parse::<bool>() {
        resources
            .entry("bool".to_string())
            .or_default()
            .push((current_name.to_string(), ResourceValue::Bool(val)));
    }
}
