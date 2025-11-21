use quick_xml::events::BytesText;

use crate::generator::parsing::ast::{ParsedResource, ResourceKind};

use super::super::state::ParseState;
use super::super::utils::text_to_string;

pub fn handle_mid(
    state: &mut ParseState,
    text: &BytesText<'_>,
) -> Option<ParsedResource> {
    let kind = ResourceKind::from_str(&state.current_tag);

    // Handle doc text
    if state.in_doc {
        return handle_doc_text(state, text, kind);
    }
    
    // Handle array items
    if state.in_array {
        return handle_array_item(state, text, kind);
    }
    
    // Handle template text
    if state.in_template {
        return handle_template_text(state, text, kind);
    }
    
    // Handle normal resources
    handle_normal_resource(state, text, kind)
}

fn handle_doc_text(
    state: &mut ParseState,
    text: &BytesText<'_>,
    kind: ResourceKind,
) -> Option<ParsedResource> {
    if kind == ResourceKind::Doc {
        let trimmed = text_to_string(text).trim().to_string();
        if !trimmed.is_empty() {
            if !state.doc_text.is_empty() {
                state.doc_text.push(' ');
            }
            state.doc_text.push_str(&trimmed);
        }
    }
    None
}

fn handle_array_item(
    state: &mut ParseState,
    text: &BytesText<'_>,
    kind: ResourceKind,
) -> Option<ParsedResource> {
    if kind == ResourceKind::Item {
        let trimmed = text_to_string(text).trim().to_string();
        if !trimmed.is_empty() {
            state.array_items.push(trimmed);
        }
    }
    None
}

fn handle_template_text(
    state: &mut ParseState,
    text: &BytesText<'_>,
    kind: ResourceKind,
) -> Option<ParsedResource> {
    if kind == ResourceKind::Template {
        let trimmed = text_to_string(text).trim().to_string();
        if !trimmed.is_empty() {
            state.template_text.push_str(&trimmed);
            state.template_text.push(' ');
        }
    }
    None
}

fn handle_normal_resource(
    state: &mut ParseState,
    text: &BytesText<'_>,
    kind: ResourceKind,
) -> Option<ParsedResource> {
    let Some(name) = &state.current_name else {
        return None;
    };

    let trimmed = text_to_string(text).trim().to_string();
    if trimmed.is_empty() {
        return None;
    }

    // Take the pending doc and clear it
    let doc = state.pending_doc.take();

    match kind {
        ResourceKind::String => handle_string(name, trimmed, doc),
        ResourceKind::Number => handle_number(name, trimmed, &state.current_number_type, doc),
        ResourceKind::Bool => handle_bool(name, trimmed, doc),
        ResourceKind::Color => handle_color(name, trimmed, doc),
        ResourceKind::Template => {
            state.template_text.push_str(&trimmed);
            state.template_text.push(' ');
            None
        }
        _ => None,
    }
}

fn handle_string(name: &str, value: String, doc: Option<String>) -> Option<ParsedResource> {
    let mut resource = ParsedResource::string(name, value);
    resource.doc = doc;
    Some(resource)
}

fn handle_number(
    name: &str,
    value: String,
    explicit_type: &Option<String>,
    doc: Option<String>,
) -> Option<ParsedResource> {
    let mut resource = ParsedResource::number(name, value, explicit_type.clone());
    resource.doc = doc;
    Some(resource)
}

fn handle_bool(name: &str, value: String, doc: Option<String>) -> Option<ParsedResource> {
    value.parse::<bool>()
        .ok()
        .map(|b| {
            let mut resource = ParsedResource::bool(name, b);
            resource.doc = doc;
            resource
        })
}

fn handle_color(name: &str, value: String, doc: Option<String>) -> Option<ParsedResource> {
    Some(ParsedResource {
        name: name.to_string(),
        kind: crate::generator::parsing::ResourceKind::Color,
        value: crate::generator::parsing::ScalarValue::Color(value),
        doc,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::events::BytesText;

    fn create_bytes_text<'a>(text: &'a str) -> BytesText<'a> {
        BytesText::from_escaped(text)
    }

    #[test]
    fn test_handle_array_item() {
        let mut state = ParseState::default();
        state.in_array = true;
        state.current_tag = "item".to_string();
        let text = create_bytes_text("Apple");
        
        let result = handle_array_item(&mut state, &text, ResourceKind::Item);
        
        assert_eq!(result, None);
        assert_eq!(state.array_items.len(), 1);
        assert_eq!(state.array_items[0], "Apple");
    }

    #[test]
    fn test_handle_array_item_not_item_tag() {
        let mut state = ParseState::default();
        state.in_array = true;
        state.current_tag = "string".to_string();
        let text = create_bytes_text("test");
        
        let result = handle_array_item(&mut state, &text, ResourceKind::String);
        
        assert_eq!(result, None);
        assert!(state.array_items.is_empty());
    }

    #[test]
    fn test_handle_array_item_empty() {
        let mut state = ParseState::default();
        state.in_array = true;
        state.current_tag = "item".to_string();
        let text = create_bytes_text("   ");
        
        handle_array_item(&mut state, &text, ResourceKind::Item);
        
        assert!(state.array_items.is_empty());
    }

    #[test]
    fn test_handle_template_text() {
        let mut state = ParseState::default();
        state.in_template = true;
        state.current_tag = "template".to_string();
        let text = create_bytes_text("Hello {name}!");
        
        let result = handle_template_text(&mut state, &text, ResourceKind::Template);
        
        assert_eq!(result, None);
        assert_eq!(state.template_text.trim(), "Hello {name}!");
    }

    #[test]
    fn test_handle_template_text_not_template_tag() {
        let mut state = ParseState::default();
        state.in_template = true;
        state.current_tag = "string".to_string();
        let text = create_bytes_text("test");
        
        let result = handle_template_text(&mut state, &text, ResourceKind::String);
        
        assert_eq!(result, None);
        assert!(state.template_text.is_empty());
    }

    #[test]
    fn test_handle_string() {
        let mut state = ParseState::default();
        state.current_name = Some("app_name".to_string());
        state.current_tag = "string".to_string();
        let text = create_bytes_text("My App");
        
        let result = handle_normal_resource(&mut state, &text, ResourceKind::String);
        
        assert!(result.is_some());
        let resource = result.unwrap();
        assert_eq!(resource.name, "app_name");
        assert_eq!(
            resource.kind,
            crate::generator::parsing::ResourceKind::String
        );
        if let crate::generator::parsing::ScalarValue::Text(value) = resource.value {
            assert_eq!(value, "My App");
        } else {
            panic!("Expected Text value");
        }
    }

    #[test]
    fn test_handle_number() {
        let mut state = ParseState::default();
        state.current_name = Some("max_retries".to_string());
        state.current_tag = "number".to_string();
        state.current_number_type = Some("i32".to_string());
        let text = create_bytes_text("3");
        
        let result = handle_normal_resource(&mut state, &text, ResourceKind::Number);
        
        assert!(result.is_some());
        let resource = result.unwrap();
        if let crate::generator::parsing::ScalarValue::Number {
            value,
            explicit_type,
        } = resource.value
        {
            assert_eq!(value, "3");
            assert_eq!(explicit_type, Some("i32".to_string()));
        } else {
            panic!("Expected Number value");
        }
    }

    #[test]
    fn test_handle_bool_true() {
        let mut state = ParseState::default();
        state.current_name = Some("enabled".to_string());
        state.current_tag = "bool".to_string();
        let text = create_bytes_text("true");
        
        let result = handle_normal_resource(&mut state, &text, ResourceKind::Bool);
        
        assert!(result.is_some());
        let resource = result.unwrap();
        assert!(matches!(
            resource.value,
            crate::generator::parsing::ScalarValue::Bool(true)
        ));
    }

    #[test]
    fn test_handle_bool_false() {
        let mut state = ParseState::default();
        state.current_name = Some("disabled".to_string());
        state.current_tag = "bool".to_string();
        let text = create_bytes_text("false");
        
        let result = handle_normal_resource(&mut state, &text, ResourceKind::Bool);
        
        assert!(result.is_some());
        let resource = result.unwrap();
        assert!(matches!(
            resource.value,
            crate::generator::parsing::ScalarValue::Bool(false)
        ));
    }

    #[test]
    fn test_handle_bool_invalid() {
        let mut state = ParseState::default();
        state.current_name = Some("invalid".to_string());
        state.current_tag = "bool".to_string();
        let text = create_bytes_text("not_a_bool");
        
        let result = handle_normal_resource(&mut state, &text, ResourceKind::Bool);
        
        assert_eq!(result, None);
    }

    #[test]
    fn test_handle_color() {
        let mut state = ParseState::default();
        state.current_name = Some("primary".to_string());
        state.current_tag = "color".to_string();
        let text = create_bytes_text("#FF0000");
        
        let result = handle_normal_resource(&mut state, &text, ResourceKind::Color);
        
        assert!(result.is_some());
        let resource = result.unwrap();
        if let crate::generator::parsing::ScalarValue::Color(value) = resource.value {
            assert_eq!(value, "#FF0000");
        } else {
            panic!("Expected Color value");
        }
    }

    #[test]
    fn test_handle_normal_resource_no_name() {
        let mut state = ParseState::default();
        state.current_name = None;
        state.current_tag = "string".to_string();
        let text = create_bytes_text("test");
        
        let result = handle_normal_resource(&mut state, &text, ResourceKind::String);
        
        assert_eq!(result, None);
    }

    #[test]
    fn test_handle_normal_resource_empty_text() {
        let mut state = ParseState::default();
        state.current_name = Some("test".to_string());
        state.current_tag = "string".to_string();
        let text = create_bytes_text("   ");
        
        let result = handle_normal_resource(&mut state, &text, ResourceKind::String);
        
        assert_eq!(result, None);
    }

    #[test]
    fn test_handle_mid_array_item() {
        let mut state = ParseState::default();
        state.in_array = true;
        state.current_tag = "item".to_string();
        let text = create_bytes_text("Banana");
        
        let result = handle_mid(&mut state, &text);
        
        assert_eq!(result, None);
        assert_eq!(state.array_items.len(), 1);
        assert_eq!(state.array_items[0], "Banana");
    }

    #[test]
    fn test_handle_mid_template_text() {
        let mut state = ParseState::default();
        state.in_template = true;
        state.current_tag = "template".to_string();
        let text = create_bytes_text("Welcome!");
        
        let result = handle_mid(&mut state, &text);
        
        assert_eq!(result, None);
        assert!(state.template_text.contains("Welcome!"));
    }

    #[test]
    fn test_handle_mid_string() {
        let mut state = ParseState::default();
        state.current_name = Some("title".to_string());
        state.current_tag = "string".to_string();
        let text = create_bytes_text("Hello World");
        
        let result = handle_mid(&mut state, &text);
        
        assert!(result.is_some());
        let resource = result.unwrap();
        assert_eq!(resource.name, "title");
    }
}
