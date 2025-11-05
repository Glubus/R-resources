use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

use super::types::ResourceValue;
pub mod basic;
pub mod advanced;

pub mod arrays;

/// Parses an XML resources file and extracts all resource definitions.
pub fn parse_resources(xml: &str) -> Result<HashMap<String, Vec<(String, ResourceValue)>>, String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut resources: HashMap<String, Vec<(String, ResourceValue)>> = HashMap::new();
    let mut buf = Vec::new();

    let mut current_tag = String::new();
    let mut current_name = String::new();
    let mut current_profile: Option<String> = None;
    let mut array_items: Vec<String> = Vec::new();
    let mut in_array = false;
    let mut namespace_stack: Vec<String> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => on_start(
                &e,
                &mut current_tag,
                &mut current_name,
                &mut current_profile,
                &mut in_array,
                &mut array_items,
                &mut namespace_stack,
            ),
            Ok(Event::Text(e)) => on_text(
                &e,
                &mut reader,
                &current_tag,
                &current_name,
                in_array,
                &mut array_items,
                &mut resources,
            ),
            Ok(Event::End(e)) => on_end(
                &e,
                &mut current_tag,
                &current_name,
                &mut in_array,
                &mut array_items,
                &mut resources,
                &mut namespace_stack,
            ),
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

/// Handles XML start tags and extracts the name and profile attributes
fn handle_start_tag(
    e: &quick_xml::events::BytesStart,
    current_tag: &mut String,
    current_name: &mut String,
    current_profile: &mut Option<String>,
    in_array: &mut bool,
    array_items: &mut Vec<String>,
    namespace_stack: &mut Vec<String>,
) {
    let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
    current_tag.clone_from(&tag_name);

    // Extract attributes (do not clear current_name/profile to preserve array context)
    let mut name_attr: Option<String> = None;
    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => {
                name_attr = Some(String::from_utf8_lossy(&attr.value).to_string());
            }
            b"profile" => {
                *current_profile = Some(String::from_utf8_lossy(&attr.value).to_string());
            }
            _ => {}
        }
    }

    // Namespace handling: <ns name="..."> pushes a namespace level
    if tag_name == "ns" {
        if let Some(ns_name) = name_attr {
            namespace_stack.push(ns_name);
        }
        // Do not treat <ns> as a resource-bearing tag
        current_name.clear();
    } else if let Some(local_name) = name_attr {
        // Qualify resource name with namespace path if present
        if namespace_stack.is_empty() {
            *current_name = local_name;
        } else {
            *current_name = format!("{}/{}", namespace_stack.join("/"), local_name);
        }
    }

    // Detect array types
    if tag_name.ends_with("-array") {
        *in_array = true;
        array_items.clear();
    }
}

/// Wrapper for Start event
fn on_start(
    e: &quick_xml::events::BytesStart,
    current_tag: &mut String,
    current_name: &mut String,
    current_profile: &mut Option<String>,
    in_array: &mut bool,
    array_items: &mut Vec<String>,
    namespace_stack: &mut Vec<String>,
) {
    handle_start_tag(
        e,
        current_tag,
        current_name,
        current_profile,
        in_array,
        array_items,
        namespace_stack,
    );
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
            "string" => basic::handle_string(&text, current_name, resources),
            "int" => basic::handle_int(&text, current_name, resources),
            "float" => basic::handle_float(&text, current_name, resources),
            "bool" => basic::handle_bool(&text, current_name, resources),
            "color" => advanced::handle_color(&text, current_name, resources),
            "url" => advanced::handle_url(&text, current_name, resources),
            "dimension" => advanced::handle_dimension(&text, current_name, resources),
            _ => {}
        }
    }
}

/// Wrapper for Text event
fn on_text(
    e: &quick_xml::events::BytesText,
    _reader: &mut Reader<&[u8]>,
    current_tag: &str,
    current_name: &str,
    in_array: bool,
    array_items: &mut Vec<String>,
    resources: &mut HashMap<String, Vec<(String, ResourceValue)>>,
) {
    let text = String::from_utf8_lossy(e).trim().to_string();
    if text.is_empty() {
        return;
    }

    handle_text_content(
        text,
        current_tag,
        current_name,
        in_array,
        array_items,
        resources,
    );
}

/// Wrapper for End event
fn on_end(
    e: &quick_xml::events::BytesEnd,
    current_tag: &mut String,
    current_name: &str,
    in_array: &mut bool,
    array_items: &mut Vec<String>,
    resources: &mut HashMap<String, Vec<(String, ResourceValue)>>,
    namespace_stack: &mut Vec<String>,
) {
    let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();

    if tag_name.ends_with("-array") && *in_array {
        arrays::handle_array_end(
            &tag_name,
            current_name,
            array_items,
            resources,
        );
        *in_array = false;
        array_items.clear();
    }

    // Pop namespace level on </ns>
    if tag_name == "ns" {
        namespace_stack.pop();
    }

    current_tag.clear();
}


