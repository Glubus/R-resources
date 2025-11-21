use quick_xml::events::BytesStart;

use crate::generator::parsing::ast::{ResourceKind, TemplateParam};

use super::super::state::ParseState;
use super::super::utils::{attr_value, to_string};

pub fn handle_start(
    state: &mut ParseState,
    e: &BytesStart<'_>,
) {
    let tag = to_string(e.name().as_ref());
    state.current_tag = tag.clone();
    let kind = ResourceKind::from_str(&tag);

    match kind {
        ResourceKind::Namespace => {
            handle_ns(state, e);
            return;
        }
        ResourceKind::Template => {
            handle_template(state);
        }
        ResourceKind::Array => {
            handle_array(state, e);
            return;
        }
        ResourceKind::Item => {
            // Item tags don't have names, just track the tag
            return;
        }
        ResourceKind::Doc => {
            handle_doc(state);
            return;
        }
        _ => {}
    }

    let number_type = if kind == ResourceKind::Number {
        attr_value(e, b"type")
    } else {
        None
    };

    let param_name = build_resource_name(state, e);

    // Handle template parameters
    if state.in_template && kind != ResourceKind::Template {
        if handle_template_param(state, e, kind, &param_name, &number_type) {
            return;
        }
    }

    // Set state for normal resource processing
    state.current_number_type = number_type;
    state.current_name = param_name;
}

fn handle_ns(state: &mut ParseState, e: &BytesStart<'_>) {
    if let Some(ns_name) = attr_value(e, b"name") {
        state.namespace_stack.push(ns_name);
    }
    state.current_name = None;
}

fn handle_template(state: &mut ParseState) {
    state.in_template = true;
    state.template_params.clear();
    state.template_text.clear();
}

fn handle_array(state: &mut ParseState, e: &BytesStart<'_>) {
    state.in_array = true;
    state.array_type = attr_value(e, b"type");
    state.array_spec = attr_value(e, b"spec");
    state.array_items.clear();
    
    let param_name = build_resource_name(state, e);
    state.current_name = param_name;
}

fn handle_template_param(
    state: &mut ParseState,
    _e: &BytesStart<'_>,
    kind: ResourceKind,
    param_name: &Option<String>,
    number_type: &Option<String>,
) -> bool {
    let Some(param_name_str) = param_name else {
        return false;
    };

    let param_value = match kind {
        ResourceKind::String => {
            Some(crate::generator::parsing::ScalarValue::Text(String::new()))
        }
        ResourceKind::Number => {
            Some(crate::generator::parsing::ScalarValue::Number {
                value: String::new(),
                explicit_type: number_type.clone(),
            })
        }
        ResourceKind::Bool => {
            Some(crate::generator::parsing::ScalarValue::Bool(false))
        }
        ResourceKind::Color => {
            Some(crate::generator::parsing::ScalarValue::Color(String::new()))
        }
        _ => None,
    };

    if let Some(value) = param_value {
        state.template_params.push(TemplateParam {
            name: param_name_str.clone(),
            value,
        });
        state.current_tag = "template".to_string();
        return true;
    }

    false
}

fn handle_doc(state: &mut ParseState) {
    state.in_doc = true;
    state.doc_text.clear();
}

fn build_resource_name(
    state: &ParseState,
    e: &BytesStart<'_>,
) -> Option<String> {
    let name_attr = attr_value(e, b"name")?;
    
    if state.namespace_stack.is_empty() {
        Some(name_attr)
    } else {
        let mut path = state.namespace_stack.join("/");
        path.push('/');
        path.push_str(&name_attr);
        Some(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::events::BytesStart;

    fn create_bytes_start<'a>(tag: &'a str, attrs: &'a [(&'a [u8], &'a [u8])]) -> BytesStart<'a> {
        let mut bytes_start = BytesStart::new(tag);
        for (name, value) in attrs {
            bytes_start.push_attribute((*name, *value));
        }
        bytes_start
    }

    #[test]
    fn test_handle_ns() {
        let mut state = ParseState::default();
        let e = create_bytes_start("ns", &[(b"name", b"auth")]);
        
        handle_ns(&mut state, &e);
        
        assert_eq!(state.namespace_stack.len(), 1);
        assert_eq!(state.namespace_stack[0], "auth");
        assert_eq!(state.current_name, None);
    }

    #[test]
    fn test_handle_template() {
        let mut state = ParseState::default();
        
        handle_template(&mut state);
        
        assert!(state.in_template);
        assert!(state.template_params.is_empty());
        assert!(state.template_text.is_empty());
    }

    #[test]
    fn test_handle_array() {
        let mut state = ParseState::default();
        let e = create_bytes_start("array", &[
            (b"name", b"numbers"),
            (b"type", b"number"),
            (b"spec", b"bigdecimal"),
        ]);
        
        handle_array(&mut state, &e);
        
        assert!(state.in_array);
        assert_eq!(state.array_type, Some("number".to_string()));
        assert_eq!(state.array_spec, Some("bigdecimal".to_string()));
        assert!(state.array_items.is_empty());
        assert_eq!(state.current_name, Some("numbers".to_string()));
    }

    #[test]
    fn test_handle_array_in_namespace() {
        let mut state = ParseState::default();
        state.namespace_stack.push("ui".to_string());
        let e = create_bytes_start("array", &[(b"name", b"colors")]);
        
        handle_array(&mut state, &e);
        
        assert_eq!(state.current_name, Some("ui/colors".to_string()));
    }

    #[test]
    fn test_build_resource_name() {
        let state = ParseState::default();
        let e = create_bytes_start("string", &[(b"name", b"app_name")]);
        
        let name = build_resource_name(&state, &e);
        
        assert_eq!(name, Some("app_name".to_string()));
    }

    #[test]
    fn test_build_resource_name_with_namespace() {
        let mut state = ParseState::default();
        state.namespace_stack.push("auth".to_string());
        state.namespace_stack.push("errors".to_string());
        let e = create_bytes_start("string", &[(b"name", b"invalid")]);
        
        let name = build_resource_name(&state, &e);
        
        assert_eq!(name, Some("auth/errors/invalid".to_string()));
    }

    #[test]
    fn test_build_resource_name_no_name_attr() {
        let state = ParseState::default();
        let e = create_bytes_start("item", &[]);
        
        let name = build_resource_name(&state, &e);
        
        assert_eq!(name, None);
    }

    #[test]
    fn test_handle_template_param_string() {
        let mut state = ParseState::default();
        state.in_template = true;
        let e = create_bytes_start("string", &[(b"name", b"user_name")]);
        
        let result = handle_template_param(
            &mut state,
            &e,
            ResourceKind::String,
            &Some("user_name".to_string()),
            &None,
        );
        
        assert!(result);
        assert_eq!(state.template_params.len(), 1);
        assert_eq!(state.template_params[0].name, "user_name");
        assert!(matches!(
            state.template_params[0].value,
            crate::generator::parsing::ScalarValue::Text(_)
        ));
    }

    #[test]
    fn test_handle_template_param_number() {
        let mut state = ParseState::default();
        state.in_template = true;
        let e = create_bytes_start("number", &[(b"name", b"count")]);
        
        let result = handle_template_param(
            &mut state,
            &e,
            ResourceKind::Number,
            &Some("count".to_string()),
            &Some("i32".to_string()),
        );
        
        assert!(result);
        assert_eq!(state.template_params.len(), 1);
        if let crate::generator::parsing::ScalarValue::Number { explicit_type, .. } = &state.template_params[0].value {
            assert_eq!(explicit_type, &Some("i32".to_string()));
        } else {
            panic!("Expected Number value");
        }
    }

    #[test]
    fn test_handle_template_param_bool() {
        let mut state = ParseState::default();
        state.in_template = true;
        
        let result = handle_template_param(
            &mut state,
            &create_bytes_start("bool", &[]),
            ResourceKind::Bool,
            &Some("enabled".to_string()),
            &None,
        );
        
        assert!(result);
        assert_eq!(state.template_params.len(), 1);
        assert!(matches!(
            state.template_params[0].value,
            crate::generator::parsing::ScalarValue::Bool(false)
        ));
    }

    #[test]
    fn test_handle_template_param_color() {
        let mut state = ParseState::default();
        state.in_template = true;
        
        let result = handle_template_param(
            &mut state,
            &create_bytes_start("color", &[]),
            ResourceKind::Color,
            &Some("primary".to_string()),
            &None,
        );
        
        assert!(result);
        assert_eq!(state.template_params.len(), 1);
        assert!(matches!(
            state.template_params[0].value,
            crate::generator::parsing::ScalarValue::Color(_)
        ));
    }

    #[test]
    fn test_handle_template_param_invalid() {
        let mut state = ParseState::default();
        state.in_template = true;
        
        let result = handle_template_param(
            &mut state,
            &create_bytes_start("unknown", &[]),
            ResourceKind::Array,
            &Some("test".to_string()),
            &None,
        );
        
        assert!(!result);
        assert!(state.template_params.is_empty());
    }

    #[test]
    fn test_handle_start_string() {
        let mut state = ParseState::default();
        let e = create_bytes_start("string", &[(b"name", b"app_name")]);
        
        handle_start(&mut state, &e);
        
        assert_eq!(state.current_tag, "string");
        assert_eq!(state.current_name, Some("app_name".to_string()));
        assert_eq!(state.current_number_type, None);
    }

    #[test]
    fn test_handle_start_number_with_type() {
        let mut state = ParseState::default();
        let e = create_bytes_start("number", &[
            (b"name", b"max_retries"),
            (b"type", b"i32"),
        ]);
        
        handle_start(&mut state, &e);
        
        assert_eq!(state.current_number_type, Some("i32".to_string()));
        assert_eq!(state.current_name, Some("max_retries".to_string()));
    }

    #[test]
    fn test_handle_start_array() {
        let mut state = ParseState::default();
        let e = create_bytes_start("array", &[
            (b"name", b"items"),
            (b"type", b"string"),
        ]);
        
        handle_start(&mut state, &e);
        
        assert!(state.in_array);
        assert_eq!(state.array_type, Some("string".to_string()));
        assert_eq!(state.current_name, Some("items".to_string()));
    }

    #[test]
    fn test_handle_start_item() {
        let mut state = ParseState::default();
        state.in_array = true;
        let e = create_bytes_start("item", &[]);
        
        handle_start(&mut state, &e);
        
        assert_eq!(state.current_tag, "item");
        // Should return early, so current_name shouldn't be set
    }
}
