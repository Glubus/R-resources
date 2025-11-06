use std::collections::HashMap;

use crate::codegen::types::ResourceValue;

/// Handles the end of array tags and adds the completed array to resources
pub fn handle_array_end(
    tag_name: &str,
    current_name: &str,
    array_items: &[String],
    resources: &mut HashMap<String, Vec<(String, ResourceValue)>>,
) {
    match tag_name {
        "string-array" => {
            resources
                .entry("string_array".to_string())
                .or_default()
                .push((
                    current_name.to_string(),
                    ResourceValue::StringArray(array_items.to_vec()),
                ));
        }
        "int-array" => {
            let ints: Vec<i64> = array_items
                .iter()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();
            resources
                .entry("int_array".to_string())
                .or_default()
                .push((current_name.to_string(), ResourceValue::IntArray(ints)));
        }
        "float-array" => {
            let floats: Vec<f64> = array_items
                .iter()
                .filter_map(|s| s.parse::<f64>().ok())
                .collect();
            resources
                .entry("float_array".to_string())
                .or_default()
                .push((current_name.to_string(), ResourceValue::FloatArray(floats)));
        }
        _ => {}
    }
}
