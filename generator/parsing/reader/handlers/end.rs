use quick_xml::events::BytesEnd;

use crate::generator::parsing::ast::{ParsedResource, ResourceKind};

use super::super::state::ParseState;
use super::super::utils::to_string;

pub fn handle_end(
    state: &mut ParseState,
    e: &BytesEnd<'_>,
) -> Option<ParsedResource> {
    let tag = to_string(e.name().as_ref());
    let kind = ResourceKind::from_str(&tag);

    match kind {
        ResourceKind::Namespace => {
            handle_ns_end(state);
            return None;
        }
        ResourceKind::Array => {
            return handle_array_end(state);
        }
        ResourceKind::Template => {
            return handle_template_end(state);
        }
        ResourceKind::Item => {
            handle_item_end(state);
            return None;
        }
        ResourceKind::Doc => {
            handle_doc_end(state);
            return None;
        }
        _ => {}
    }

    // Handle template parameter closing tags
    if state.in_template {
        if handle_template_param_end(state, &kind) {
            return None;
        }
    }

    // Handle normal resource closing tags
    handle_normal_resource_end(state, &kind);
    None
}

fn handle_ns_end(state: &mut ParseState) {
    state.namespace_stack.pop();
}

fn handle_doc_end(state: &mut ParseState) {
    // Store the documentation for the next resource
    let doc = state.doc_text.trim().to_string();
    state.pending_doc = if doc.is_empty() { None } else { Some(doc) };
    state.in_doc = false;
    state.doc_text.clear();
}

fn handle_array_end(state: &mut ParseState) -> Option<ParsedResource> {
    let name = state.current_name.clone()?;
    
    let element_type = state.array_type.clone().unwrap_or_else(|| "string".to_string());
    let spec = state.array_spec.clone();
    let items = state.array_items.clone();
    
    // Reset array state
    state.in_array = false;
    state.array_type = None;
    state.array_spec = None;
    state.array_items.clear();
    state.current_name = None;
    
    // Take the pending doc
    let doc = state.pending_doc.take();
    
    Some(ParsedResource {
        name,
        kind: crate::generator::parsing::ResourceKind::Array,
        value: crate::generator::parsing::ScalarValue::Array {
            element_type,
            spec,
            items,
        },
        doc,
    })
}

fn handle_template_end(state: &mut ParseState) -> Option<ParsedResource> {
    let name = state.current_name.clone()?;
    
    let text = state.template_text.trim().to_string();
    let params = state.template_params.clone();
    
    // Take the pending doc
    let doc = state.pending_doc.take();
    
    // Reset template state
    state.in_template = false;
    state.template_params.clear();
    state.template_text.clear();
    state.current_name = None;
    
    Some(ParsedResource {
        name,
        kind: crate::generator::parsing::ResourceKind::Template,
        value: crate::generator::parsing::ScalarValue::Template {
            text,
            params,
        },
        doc,
    })
}

fn handle_template_param_end(
    state: &mut ParseState,
    kind: &ResourceKind,
) -> bool {
    // Only handle as template parameter if we're actually in a template
    if !state.in_template {
        return false;
    }
    
    if matches!(
        kind,
        ResourceKind::String | ResourceKind::Number | ResourceKind::Bool | ResourceKind::Color
    ) {
        // These are template parameters, not resources - just clear current_name
        state.current_name = None;
        return true;
    }
    false
}

fn handle_item_end(state: &mut ParseState) {
    // Clear current_tag when closing item, but keep array state
    state.current_tag.clear();
}

fn handle_normal_resource_end(state: &mut ParseState, kind: &ResourceKind) {
    if matches!(
        kind,
        ResourceKind::String | ResourceKind::Number | ResourceKind::Bool | ResourceKind::Color
            | ResourceKind::Template | ResourceKind::Array
    ) {
        state.current_name = None;
    }
    
    if *kind == ResourceKind::Number {
        state.current_number_type = None;
    }
    
    state.current_tag.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::events::BytesEnd;

    fn create_bytes_end<'a>(tag: &'a str) -> BytesEnd<'a> {
        BytesEnd::new(tag)
    }

    #[test]
    fn test_handle_ns_end() {
        let mut state = ParseState::default();
        state.namespace_stack.push("auth".to_string());
        state.namespace_stack.push("errors".to_string());
        
        handle_ns_end(&mut state);
        
        assert_eq!(state.namespace_stack.len(), 1);
        assert_eq!(state.namespace_stack[0], "auth");
    }

    #[test]
    fn test_handle_array_end() {
        let mut state = ParseState::default();
        state.current_name = Some("fruits".to_string());
        state.array_type = Some("string".to_string());
        state.array_spec = None;
        state.array_items = vec!["Apple".to_string(), "Banana".to_string()];
        
        let result = handle_array_end(&mut state);
        
        assert!(result.is_some());
        let resource = result.unwrap();
        assert_eq!(resource.name, "fruits");
        assert_eq!(
            resource.kind,
            crate::generator::parsing::ResourceKind::Array
        );
        if let crate::generator::parsing::ScalarValue::Array {
            element_type,
            spec,
            items,
        } = resource.value
        {
            assert_eq!(element_type, "string");
            assert_eq!(spec, None);
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], "Apple");
            assert_eq!(items[1], "Banana");
        } else {
            panic!("Expected Array value");
        }
        
        // Verify state was reset
        assert!(!state.in_array);
        assert_eq!(state.array_type, None);
        assert_eq!(state.array_spec, None);
        assert!(state.array_items.is_empty());
        assert_eq!(state.current_name, None);
    }

    #[test]
    fn test_handle_array_end_no_name() {
        let mut state = ParseState::default();
        state.current_name = None;
        state.array_items = vec!["test".to_string()];
        
        let result = handle_array_end(&mut state);
        
        assert_eq!(result, None);
    }

    #[test]
    fn test_handle_array_end_default_type() {
        let mut state = ParseState::default();
        state.current_name = Some("items".to_string());
        state.array_type = None; // No type specified, should default to "string"
        state.array_items = vec!["item1".to_string()];
        
        let result = handle_array_end(&mut state);
        
        assert!(result.is_some());
        if let crate::generator::parsing::ScalarValue::Array {
            element_type, ..
        } = result.unwrap().value
        {
            assert_eq!(element_type, "string");
        } else {
            panic!("Expected Array value");
        }
    }

    #[test]
    fn test_handle_template_end() {
        let mut state = ParseState::default();
        state.current_name = Some("welcome".to_string());
        state.template_text = "Hello {name}!".to_string();
        state.template_params = vec![crate::generator::parsing::ast::TemplateParam {
            name: "name".to_string(),
            value: crate::generator::parsing::ScalarValue::Text(String::new()),
        }];
        
        let result = handle_template_end(&mut state);
        
        assert!(result.is_some());
        let resource = result.unwrap();
        assert_eq!(resource.name, "welcome");
        assert_eq!(
            resource.kind,
            crate::generator::parsing::ResourceKind::Template
        );
        if let crate::generator::parsing::ScalarValue::Template { text, params } = resource.value {
            assert_eq!(text, "Hello {name}!");
            assert_eq!(params.len(), 1);
        } else {
            panic!("Expected Template value");
        }
        
        // Verify state was reset
        assert!(!state.in_template);
        assert!(state.template_params.is_empty());
        assert!(state.template_text.is_empty());
        assert_eq!(state.current_name, None);
    }

    #[test]
    fn test_handle_template_end_no_name() {
        let mut state = ParseState::default();
        state.current_name = None;
        state.template_text = "test".to_string();
        
        let result = handle_template_end(&mut state);
        
        assert_eq!(result, None);
    }

    #[test]
    fn test_handle_template_param_end_string() {
        let mut state = ParseState::default();
        state.in_template = true;
        state.current_name = Some("param".to_string());
        
        let result = handle_template_param_end(&mut state, &ResourceKind::String);
        
        assert!(result);
        assert_eq!(state.current_name, None);
    }

    #[test]
    fn test_handle_template_param_end_number() {
        let mut state = ParseState::default();
        state.in_template = true;
        state.current_name = Some("count".to_string());
        
        let result = handle_template_param_end(&mut state, &ResourceKind::Number);
        
        assert!(result);
        assert_eq!(state.current_name, None);
    }

    #[test]
    fn test_handle_template_param_end_not_in_template() {
        let mut state = ParseState::default();
        state.in_template = false;
        state.current_name = Some("test".to_string());
        
        let result = handle_template_param_end(&mut state, &ResourceKind::String);
        
        // When not in template, the function should return false
        // because it only handles template parameters when in_template is true
        assert!(!result);
        assert_eq!(state.current_name, Some("test".to_string()));
    }

    #[test]
    fn test_handle_template_param_end_not_param_type() {
        let mut state = ParseState::default();
        state.in_template = true;
        state.current_name = Some("test".to_string());
        
        let result = handle_template_param_end(&mut state, &ResourceKind::Array);
        
        assert!(!result);
        assert_eq!(state.current_name, Some("test".to_string()));
    }

    #[test]
    fn test_handle_item_end() {
        let mut state = ParseState::default();
        state.in_array = true;
        state.current_tag = "item".to_string();
        state.array_items = vec!["test".to_string()];
        
        handle_item_end(&mut state);
        
        assert!(state.current_tag.is_empty());
        // Array state should be preserved
        assert!(state.in_array);
        assert_eq!(state.array_items.len(), 1);
    }

    #[test]
    fn test_handle_normal_resource_end_string() {
        let mut state = ParseState::default();
        state.current_name = Some("app_name".to_string());
        state.current_tag = "string".to_string();
        
        handle_normal_resource_end(&mut state, &ResourceKind::String);
        
        assert_eq!(state.current_name, None);
        assert!(state.current_tag.is_empty());
    }

    #[test]
    fn test_handle_normal_resource_end_number() {
        let mut state = ParseState::default();
        state.current_name = Some("max".to_string());
        state.current_number_type = Some("i32".to_string());
        state.current_tag = "number".to_string();
        
        handle_normal_resource_end(&mut state, &ResourceKind::Number);
        
        assert_eq!(state.current_name, None);
        assert_eq!(state.current_number_type, None);
        assert!(state.current_tag.is_empty());
    }

    #[test]
    fn test_handle_normal_resource_end_not_resource() {
        let mut state = ParseState::default();
        state.current_name = Some("test".to_string());
        state.current_tag = "unknown".to_string();
        
        handle_normal_resource_end(&mut state, &ResourceKind::Namespace);
        
        // Should not clear current_name for non-resource tags
        assert_eq!(state.current_name, Some("test".to_string()));
        assert!(state.current_tag.is_empty());
    }

    #[test]
    fn test_handle_end_namespace() {
        let mut state = ParseState::default();
        state.namespace_stack.push("auth".to_string());
        let e = create_bytes_end("ns");
        
        let result = handle_end(&mut state, &e);
        
        assert_eq!(result, None);
        assert!(state.namespace_stack.is_empty());
    }

    #[test]
    fn test_handle_end_array() {
        let mut state = ParseState::default();
        state.current_name = Some("numbers".to_string());
        state.array_type = Some("number".to_string());
        state.array_items = vec!["1".to_string()];
        let e = create_bytes_end("array");
        
        let result = handle_end(&mut state, &e);
        
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "numbers");
    }

    #[test]
    fn test_handle_end_template() {
        let mut state = ParseState::default();
        state.current_name = Some("welcome".to_string());
        state.template_text = "Hello!".to_string();
        let e = create_bytes_end("template");
        
        let result = handle_end(&mut state, &e);
        
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "welcome");
    }

    #[test]
    fn test_handle_end_item() {
        let mut state = ParseState::default();
        state.in_array = true;
        state.current_tag = "item".to_string();
        let e = create_bytes_end("item");
        
        let result = handle_end(&mut state, &e);
        
        assert_eq!(result, None);
        assert!(state.current_tag.is_empty());
    }

    #[test]
    fn test_handle_end_string() {
        let mut state = ParseState::default();
        state.current_name = Some("title".to_string());
        state.current_tag = "string".to_string();
        let e = create_bytes_end("string");
        
        let result = handle_end(&mut state, &e);
        
        assert_eq!(result, None);
        assert_eq!(state.current_name, None);
    }
}
