use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

use super::types::ResourceValue;

/// Parses an XML resources file and extracts all resource definitions.
///
/// # Arguments
///
/// * `xml` - The XML content as a string
///
/// # Returns
///
/// A HashMap where:
/// - Key: resource type (e.g., "string", "int", "string_array")
/// - Value: Vec of (name, ResourceValue) pairs
///
/// # Errors
///
/// Returns a String error message if the XML is malformed or parsing fails
pub fn parse_resources(xml: &str) -> Result<HashMap<String, Vec<(String, ResourceValue)>>, String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut resources: HashMap<String, Vec<(String, ResourceValue)>> = HashMap::new();
    let mut buf = Vec::new();

    let mut current_tag = String::new();
    let mut current_name = String::new();
    let mut array_items: Vec<String> = Vec::new();
    let mut in_array = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                handle_start_tag(
                    e,
                    &mut current_tag,
                    &mut current_name,
                    &mut in_array,
                    &mut array_items,
                );
            }
            Ok(Event::Text(e)) => {
                let text = String::from_utf8_lossy(&e).trim().to_string();
                if text.is_empty() {
                    continue;
                }

                handle_text_content(
                    text,
                    &current_tag,
                    &current_name,
                    in_array,
                    &mut array_items,
                    &mut resources,
                );
            }
            Ok(Event::End(e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                if tag_name.ends_with("-array") && in_array {
                    handle_array_end(&tag_name, &current_name, &array_items, &mut resources);
                    in_array = false;
                    array_items.clear();
                }

                current_tag.clear();
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(format!(
                    "XML parsing error at position {}: {}",
                    reader.buffer_position(),
                    e
                ))
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(resources)
}

/// Handles XML start tags and extracts the name attribute
fn handle_start_tag(
    e: quick_xml::events::BytesStart,
    current_tag: &mut String,
    current_name: &mut String,
    in_array: &mut bool,
    array_items: &mut Vec<String>,
) {
    let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
    *current_tag = tag_name.clone();

    // Extract the "name" attribute
    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"name" {
            *current_name = String::from_utf8_lossy(&attr.value).to_string();
        }
    }

    // Detect array types
    if tag_name.ends_with("-array") {
        *in_array = true;
        array_items.clear();
    }
}

/// Handles text content within XML tags
fn handle_text_content(
    text: String,
    current_tag: &str,
    current_name: &str,
    in_array: bool,
    array_items: &mut Vec<String>,
    resources: &mut HashMap<String, Vec<(String, ResourceValue)>>,
) {
    if in_array && current_tag == "item" {
        array_items.push(text);
    } else if !current_name.is_empty() {
        match current_tag {
            "string" => {
                resources
                    .entry("string".to_string())
                    .or_default()
                    .push((current_name.to_string(), ResourceValue::String(text)));
            }
            "int" => {
                if let Ok(val) = text.parse::<i64>() {
                    resources
                        .entry("int".to_string())
                        .or_default()
                        .push((current_name.to_string(), ResourceValue::Int(val)));
                }
            }
            "float" => {
                if let Ok(val) = text.parse::<f64>() {
                    resources
                        .entry("float".to_string())
                        .or_default()
                        .push((current_name.to_string(), ResourceValue::Float(val)));
                }
            }
            _ => {}
        }
    }
}

/// Handles the end of array tags and adds the completed array to resources
fn handle_array_end(
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
