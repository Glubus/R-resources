use crate::generator::ir::types::ResourceType;
use crate::generator::ir::{
    ResourceKey, ResourceKind, ResourceNode, ResourceOrigin,
    ResourceValue,
};
use crate::generator::parsing::{ParsedResource, ScalarValue};
use crate::generator::utils::{format_doc, sanitize_identifier};

pub struct ArrayType;

impl ResourceType for ArrayType {
    fn name(&self) -> &'static str {
        "array"
    }

    fn xml_tags(&self) -> &'static [&'static str] {
        &["array"]
    }

    fn resource_kind(&self) -> ResourceKind {
        ResourceKind::Array("".to_string()) // Type will be determined from value
    }

    fn build_node(
        &self,
        parsed: &ParsedResource,
        origin: ResourceOrigin,
    ) -> Option<ResourceNode> {
        if let ScalarValue::Array {
            element_type,
            spec,
            items,
        } = &parsed.value
        {
            Some(ResourceNode {
                kind: ResourceKind::Array(element_type.clone()),
                value: ResourceValue::Array {
                    element_type: element_type.clone(),
                    spec: spec.clone(),
                    items: items.clone(),
                },
                origin,
                doc: parsed.doc.clone(),
            })
        } else {
            None
        }
    }

    fn emit_rust(
        &self,
        key: &ResourceKey,
        node: &ResourceNode,
        indent: usize,
    ) -> Option<String> {
        if let ResourceValue::Array {
            element_type,
            spec,
            items,
        } = &node.value
        {
            let pad = " ".repeat(indent);
            let const_name = sanitize_identifier(&key.name).to_uppercase();
            let doc_str = format_doc(&node.doc, indent);

            match element_type.as_str() {
                "string" => {
                    let items_str: Vec<String> = items
                        .iter()
                        .map(|item| format!("\"{}\"", item.escape_debug()))
                        .collect();
                    Some(format!(
                        "{doc_str}{pad}pub const {const_name}: &[&str] = &[{}];\n",
                        items_str.join(", ")
                    ))
                }
                "number" => {
                    let rust_type = match spec.as_deref() {
                        Some("i8") => "i8",
                        Some("i16") => "i16",
                        Some("i32") => "i32",
                        Some("i64") => "i64",
                        Some("u8") => "u8",
                        Some("u16") => "u16",
                        Some("u32") => "u32",
                        Some("u64") => "u64",
                        Some("f32") => "f32",
                        Some("f64") => "f64",
                        Some("bigdecimal") => {
                            // For BigDecimal arrays, generate individual statics and reference them
                            let mut code = String::new();
                            code.push_str(&doc_str);
                            let items_refs: Vec<String> = items
                                .iter()
                                .enumerate()
                                .map(|(idx, item)| {
                                    let escaped = escape_literal(item);
                                    let static_name = format!("{const_name}_ITEM_{idx}");
                                    code.push_str(&format!(
                                        "{pad}static {static_name}: std::sync::LazyLock<r_resources::BigDecimal> = std::sync::LazyLock::new(|| {{\n\
                                        {pad}    r_resources::BigDecimal::from_str(\"{escaped}\").expect(\"valid decimal literal\")\n\
                                        {pad}}});\n"
                                    ));
                                    static_name
                                })
                                .collect();
                            code.push_str(&format!(
                                "{pad}pub static {const_name}: &[&std::sync::LazyLock<r_resources::BigDecimal>] = &[{}];\n",
                                items_refs.iter().map(|name| format!("&{name}")).collect::<Vec<_>>().join(", ")
                            ));
                            return Some(code);
                        }
                        _ => {
                            // Auto-detect type for each item
                            return emit_auto_number_array(&pad, const_name, items, &doc_str);
                        }
                    };
                    
                    // Parse and validate items for explicit types
                    let items_str: Vec<String> = items
                        .iter()
                        .map(|item| {
                            parse_number_for_type(item, spec.as_deref())
                                .unwrap_or_else(|_| item.clone())
                        })
                        .collect();
                    
                    Some(format!(
                        "{doc_str}{pad}pub const {const_name}: &[{rust_type}] = &[{}];\n",
                        items_str.join(", ")
                    ))
                }
                "bool" => {
                    let items_str: Vec<String> = items
                        .iter()
                        .map(|item| {
                            item.trim().parse::<bool>()
                                .map(|b| b.to_string())
                                .unwrap_or_else(|_| format!("/* invalid: {item} */false"))
                        })
                        .collect();
                    Some(format!(
                        "{doc_str}{pad}pub const {const_name}: &[bool] = &[{}];\n",
                        items_str.join(", ")
                    ))
                }
                "color" => {
                    // Colors are strings, but we could generate Color structs
                    let items_str: Vec<String> = items
                        .iter()
                        .map(|item| format!("\"{}\"", item.escape_debug()))
                        .collect();
                    Some(format!(
                        "{doc_str}{pad}pub const {const_name}: &[&str] = &[{}];\n",
                        items_str.join(", ")
                    ))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

fn emit_auto_number_array(pad: &str, const_name: String, items: &[String], doc_str: &str) -> Option<String> {
    // Try to determine the best type for all items
    let mut all_int = true;
    let mut all_fit_i64 = true;
    let mut has_decimal = false;
    let mut max_significant_digits = 0;

    for item in items {
        let trimmed = item.trim();
        if trimmed.contains('.') || trimmed.contains('e') || trimmed.contains('E') {
            has_decimal = true;
            all_int = false;
        }
        
        let sig_digits = count_significant_digits(trimmed);
        if sig_digits > max_significant_digits {
            max_significant_digits = sig_digits;
        }

        if all_int {
            if trimmed.parse::<i64>().is_err() {
                all_fit_i64 = false;
            }
        }
    }

    // Determine the type
    if all_int && all_fit_i64 {
        // All integers that fit in i64
        let items_str: Vec<String> = items.iter().map(|s| s.trim().to_string()).collect();
        Some(format!(
            "{doc_str}{pad}pub const {const_name}: &[i64] = &[{}];\n",
            items_str.join(", ")
        ))
    } else if has_decimal && max_significant_digits <= 15 {
        // Decimals that fit in f64
        let items_str: Vec<String> = items
            .iter()
            .map(|item| {
                let f = item.trim().parse::<f64>().unwrap_or(0.0);
                format_float(f)
            })
            .collect();
        Some(format!(
            "{doc_str}{pad}pub const {const_name}: &[f64] = &[{}];\n",
            items_str.join(", ")
        ))
    } else {
        // Need BigDecimal - generate individual statics and reference them
        let mut code = String::new();
        code.push_str(doc_str);
        let items_refs: Vec<String> = items
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let escaped = escape_literal(item.trim());
                let static_name = format!("{const_name}_ITEM_{idx}");
                code.push_str(&format!(
                    "{pad}static {static_name}: std::sync::LazyLock<r_resources::BigDecimal> = std::sync::LazyLock::new(|| {{\n\
                    {pad}    r_resources::BigDecimal::from_str(\"{escaped}\").expect(\"valid decimal literal\")\n\
                    {pad}}});\n"
                ));
                static_name
            })
            .collect();
        code.push_str(&format!(
            "{pad}pub static {const_name}: &[&std::sync::LazyLock<r_resources::BigDecimal>] = &[{}];\n",
            items_refs.iter().map(|name| format!("&{name}")).collect::<Vec<_>>().join(", ")
        ));
        Some(code)
    }
}

fn parse_number_for_type(literal: &str, spec: Option<&str>) -> Result<String, String> {
    let trimmed = literal.trim();
    if let Some(type_hint) = spec {
        match type_hint {
            "i8" => trimmed.parse::<i8>().map(|v| v.to_string()).map_err(|_| format!("'{trimmed}' does not fit in i8")),
            "i16" => trimmed.parse::<i16>().map(|v| v.to_string()).map_err(|_| format!("'{trimmed}' does not fit in i16")),
            "i32" => trimmed.parse::<i32>().map(|v| v.to_string()).map_err(|_| format!("'{trimmed}' does not fit in i32")),
            "i64" => trimmed.parse::<i64>().map(|v| v.to_string()).map_err(|_| format!("'{trimmed}' does not fit in i64")),
            "u8" => trimmed.parse::<u8>().map(|v| v.to_string()).map_err(|_| format!("'{trimmed}' does not fit in u8")),
            "u16" => trimmed.parse::<u16>().map(|v| v.to_string()).map_err(|_| format!("'{trimmed}' does not fit in u16")),
            "u32" => trimmed.parse::<u32>().map(|v| v.to_string()).map_err(|_| format!("'{trimmed}' does not fit in u32")),
            "u64" => trimmed.parse::<u64>().map(|v| v.to_string()).map_err(|_| format!("'{trimmed}' does not fit in u64")),
            "f32" => trimmed.parse::<f32>().map(format_float32).map_err(|_| format!("'{trimmed}' is not a valid f32 literal")),
            "f64" => trimmed.parse::<f64>().map(format_float64).map_err(|_| format!("'{trimmed}' is not a valid f64 literal")),
            _ => Ok(trimmed.to_string()),
        }
    } else {
        Ok(trimmed.to_string())
    }
}

fn count_significant_digits(literal: &str) -> usize {
    let mut digits = 0;
    let mut in_exponent = false;
    
    for ch in literal.chars() {
        match ch {
            '0'..='9' => {
                if !in_exponent {
                    digits += 1;
                }
            }
            'e' | 'E' => {
                in_exponent = true;
            }
            '+' | '-' if in_exponent => {}
            '.' => {}
            _ => {}
        }
    }
    
    digits
}

fn format_float(value: f64) -> String {
    let s = value.to_string();
    if s.contains('.') || s.contains('e') || s.contains('E') {
        s
    } else {
        format!("{s}.0")
    }
}

fn format_float32(value: f32) -> String {
    let s = value.to_string();
    if s.contains('.') || s.contains('e') || s.contains('E') {
        s
    } else {
        format!("{s}.0")
    }
}

fn format_float64(value: f64) -> String {
    let s = value.to_string();
    if s.contains('.') || s.contains('e') || s.contains('E') {
        s
    } else {
        format!("{s}.0")
    }
}

fn escape_literal(literal: &str) -> String {
    literal.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::ir::model::ResourceKind as ModelResourceKind;
    use crate::generator::parsing::{ParsedResource, ResourceKind as AstResourceKind, ScalarValue};
    use std::path::PathBuf;

    #[test]
    fn test_handler_name() {
        let handler = ArrayType;
        assert_eq!(handler.name(), "array");
    }

    #[test]
    fn test_handler_xml_tags() {
        let handler = ArrayType;
        let tags = handler.xml_tags();
        assert!(tags.contains(&"array"));
    }

    #[test]
    fn test_build_node_string_array() {
        let handler = ArrayType;
        let parsed = ParsedResource {
            name: "fruits".to_string(),
            kind: AstResourceKind::Array,
            value: ScalarValue::Array {
                element_type: "string".to_string(),
                spec: None,
                items: vec!["Apple".to_string(), "Banana".to_string()],
            },
            doc: None,
        };
        let origin = crate::generator::ir::ResourceOrigin::new(PathBuf::from("test.xml"), false);

        let result = handler.build_node(&parsed, origin).unwrap();
        assert_eq!(result.kind, ModelResourceKind::Array("string".to_string()));
        if let ResourceValue::Array {
            element_type,
            spec,
            items,
        } = result.value
        {
            assert_eq!(element_type, "string");
            assert_eq!(spec, None);
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], "Apple");
            assert_eq!(items[1], "Banana");
        } else {
            panic!("Expected Array value");
        }
    }

    #[test]
    fn test_build_node_number_array_with_spec() {
        let handler = ArrayType;
        let parsed = ParsedResource {
            name: "big_numbers".to_string(),
            kind: AstResourceKind::Array,
            value: ScalarValue::Array {
                element_type: "number".to_string(),
                spec: Some("bigdecimal".to_string()),
                items: vec!["123.456".to_string(), "789.012".to_string()],
            },
            doc: None,
        };
        let origin = crate::generator::ir::ResourceOrigin::new(PathBuf::from("test.xml"), false);

        let result = handler.build_node(&parsed, origin).unwrap();
        if let ResourceValue::Array {
            element_type,
            spec,
            items,
        } = result.value
        {
            assert_eq!(element_type, "number");
            assert_eq!(spec, Some("bigdecimal".to_string()));
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected Array value");
        }
    }

    #[test]
    fn test_emit_rust_string_array() {
        let handler = ArrayType;
        let key = crate::generator::ir::ResourceKey {
            namespace: vec![],
            name: "fruits".to_string(),
        };
        let node = crate::generator::ir::ResourceNode {
            kind: ModelResourceKind::Array("string".to_string()),
            value: ResourceValue::Array {
                element_type: "string".to_string(),
                spec: None,
                items: vec!["Apple".to_string(), "Banana".to_string()],
            },
            origin: crate::generator::ir::ResourceOrigin::new(PathBuf::from("test.xml"), false),
            doc: None,
        };

        let result = handler.emit_rust(&key, &node, 4).unwrap();
        assert!(result.contains("pub const FRUITS: &[&str]"));
        assert!(result.contains("\"Apple\""));
        assert!(result.contains("\"Banana\""));
    }

    #[test]
    fn test_emit_rust_number_array_i64() {
        let handler = ArrayType;
        let key = crate::generator::ir::ResourceKey {
            namespace: vec![],
            name: "numbers".to_string(),
        };
        let node = crate::generator::ir::ResourceNode {
            kind: ModelResourceKind::Array("number".to_string()),
            value: ResourceValue::Array {
                element_type: "number".to_string(),
                spec: None,
                items: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            },
            origin: crate::generator::ir::ResourceOrigin::new(PathBuf::from("test.xml"), false),
            doc: None,
        };

        let result = handler.emit_rust(&key, &node, 4).unwrap();
        assert!(result.contains("pub const NUMBERS: &[i64]"));
        assert!(result.contains("1"));
        assert!(result.contains("2"));
        assert!(result.contains("3"));
    }

    #[test]
    fn test_emit_rust_number_array_bigdecimal() {
        let handler = ArrayType;
        let key = crate::generator::ir::ResourceKey {
            namespace: vec![],
            name: "big_numbers".to_string(),
        };
        let node = crate::generator::ir::ResourceNode {
            kind: ModelResourceKind::Array("number".to_string()),
            value: ResourceValue::Array {
                element_type: "number".to_string(),
                spec: Some("bigdecimal".to_string()),
                items: vec!["123.456".to_string()],
            },
            origin: crate::generator::ir::ResourceOrigin::new(PathBuf::from("test.xml"), false),
            doc: None,
        };

        let result = handler.emit_rust(&key, &node, 4).unwrap();
        assert!(result.contains("pub static BIG_NUMBERS"));
        assert!(result.contains("LazyLock"));
        assert!(result.contains("BigDecimal"));
        assert!(result.contains("123.456"));
    }

    #[test]
    fn test_emit_rust_bool_array() {
        let handler = ArrayType;
        let key = crate::generator::ir::ResourceKey {
            namespace: vec![],
            name: "flags".to_string(),
        };
        let node = crate::generator::ir::ResourceNode {
            kind: ModelResourceKind::Array("bool".to_string()),
            value: ResourceValue::Array {
                element_type: "bool".to_string(),
                spec: None,
                items: vec!["true".to_string(), "false".to_string()],
            },
            origin: crate::generator::ir::ResourceOrigin::new(PathBuf::from("test.xml"), false),
            doc: None,
        };

        let result = handler.emit_rust(&key, &node, 4).unwrap();
        assert!(result.contains("pub const FLAGS: &[bool]"));
        assert!(result.contains("true"));
        assert!(result.contains("false"));
    }

    #[test]
    fn test_emit_rust_color_array() {
        let handler = ArrayType;
        let key = crate::generator::ir::ResourceKey {
            namespace: vec![],
            name: "colors".to_string(),
        };
        let node = crate::generator::ir::ResourceNode {
            kind: ModelResourceKind::Array("color".to_string()),
            value: ResourceValue::Array {
                element_type: "color".to_string(),
                spec: None,
                items: vec!["#FF0000".to_string()],
            },
            origin: crate::generator::ir::ResourceOrigin::new(PathBuf::from("test.xml"), false),
            doc: None,
        };

        let result = handler.emit_rust(&key, &node, 4).unwrap();
        assert!(result.contains("pub const COLORS: &[&str]"));
        assert!(result.contains("#FF0000"));
    }
}

