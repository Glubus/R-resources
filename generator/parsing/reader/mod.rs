//! Streaming XML reader that converts [`RawResourceFile`] into parsed resources.

mod handlers;
mod state;
mod utils;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::generator::input::RawResourceFile;

use super::ast::ParsedResourceFile;
use super::error::ParserError;
use handlers::{handle_end, handle_mid, handle_start};
use state::ParseState;

pub(super) fn parse_single_file(
    raw: &RawResourceFile,
) -> Result<ParsedResourceFile, ParserError> {
    let mut reader = Reader::from_str(&raw.contents);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut state = ParseState::default();
    let mut resources = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => handle_start(&mut state, &e),
            Ok(Event::Empty(e)) => {
                // Handle self-closing tags like <param name="..." type="..."/>
                handle_start(&mut state, &e);
            }
            Ok(Event::Text(e)) => {
                if let Some(res) = handle_mid(&mut state, &e) {
                    resources.push(res);
                }
            }
            Ok(Event::End(e)) => {
                if let Some(res) = handle_end(&mut state, &e) {
                    resources.push(res);
                }
            }
            Ok(Event::Eof) => break,
            Err(err) => {
                return Err(ParserError::Xml {
                    path: raw.path.clone(),
                    message: format!(
                        "XML error at byte {}: {err}",
                        reader.buffer_position()
                    ),
                });
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(ParsedResourceFile::new(
        raw.path.clone(),
        raw.is_test,
        resources,
    ))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::generator::input::RawResourceFile;

    use super::parse_single_file;

    #[test]
    fn parse_basic_string() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"<resources><string name="app_name">Demo</string></resources>"#.into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 1);
        assert_eq!(file.resources[0].name, "app_name");
    }

    #[test]
    fn parse_namespaced_string() {
        let raw = RawResourceFile::new(
            PathBuf::from("namespaced.xml"),
            r#"
<resources>
    <ns name="auth">
        <string name="title">Login</string>
    </ns>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 1);
        assert_eq!(file.resources[0].name, "auth/title");
    }

    #[test]
    fn parse_numbers_and_bools() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <number name="max_retries">3</number>
    <int name="timeout">5000</int>
    <float name="ratio">0.75</float>
    <bool name="enabled">true</bool>
    <bool name="disabled">false</bool>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 5);

        let max_retries = file
            .resources
            .iter()
            .find(|r| r.name == "max_retries")
            .unwrap();
        assert_eq!(
            max_retries.kind,
            crate::generator::parsing::ResourceKind::Number
        );
        assert_eq!(max_retries.value.as_number(), Some("3"));

        let enabled = file
            .resources
            .iter()
            .find(|r| r.name == "enabled")
            .unwrap();
        assert_eq!(
            enabled.kind,
            crate::generator::parsing::ResourceKind::Bool
        );
        assert_eq!(enabled.value.as_bool(), Some(true));

        let disabled = file
            .resources
            .iter()
            .find(|r| r.name == "disabled")
            .unwrap();
        assert_eq!(disabled.value.as_bool(), Some(false));
    }

    #[test]
    fn parse_number_with_explicit_type() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <number name="small" type="i8">127</number>
    <number name="medium" type="i32">2147483647</number>
    <number name="unsigned" type="u32">4294967295</number>
    <number name="float32" type="f32">3.14</number>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 4);

        let small = file
            .resources
            .iter()
            .find(|r| r.name == "small")
            .unwrap();
        assert_eq!(small.value.number_explicit_type(), Some("i8"));

        let float32 = file
            .resources
            .iter()
            .find(|r| r.name == "float32")
            .unwrap();
        assert_eq!(float32.value.number_explicit_type(), Some("f32"));
    }

    #[test]
    fn parse_color() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <color name="primary">#FF5722</color>
    <color name="primary_alpha">#AAFF5722</color>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 2);

        let primary = file
            .resources
            .iter()
            .find(|r| r.name == "primary")
            .unwrap();
        assert_eq!(
            primary.kind,
            crate::generator::parsing::ResourceKind::Color
        );
        assert_eq!(primary.value.as_color(), Some("#FF5722"));

        let primary_alpha = file
            .resources
            .iter()
            .find(|r| r.name == "primary_alpha")
            .unwrap();
        assert_eq!(primary_alpha.value.as_color(), Some("#AAFF5722"));
    }

    #[test]
    fn parse_template_with_params() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <template name="welcome_message">
        <string name="name"/>
        <number name="count"/>
        Welcome to {name}, you have {count} messages!
    </template>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 1);

        let template = file
            .resources
            .iter()
            .find(|r| r.name == "welcome_message")
            .unwrap();
        assert_eq!(
            template.kind,
            crate::generator::parsing::ResourceKind::Template
        );
        
        if let crate::generator::parsing::ScalarValue::Template { text, params } = &template.value {
            assert_eq!(text.trim(), "Welcome to {name}, you have {count} messages!");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0].name, "name");
            assert!(matches!(params[0].value, crate::generator::parsing::ScalarValue::Text(_)));
            assert_eq!(params[1].name, "count");
            if let crate::generator::parsing::ScalarValue::Number { explicit_type, .. } = &params[1].value {
                assert_eq!(explicit_type, &None); // No explicit type in test
            } else {
                panic!("Expected Number value");
            }
        } else {
            panic!("Expected Template value");
        }
    }

    #[test]
    fn parse_template_with_explicit_type() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <template name="welcome_message">
        <string name="name"/>
        <number name="count" type="bigdecimal"/>
        Welcome to {name}, you have {count} messages!
    </template>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 1);

        let template = file
            .resources
            .iter()
            .find(|r| r.name == "welcome_message")
            .unwrap();
        
        if let crate::generator::parsing::ScalarValue::Template { params, .. } = &template.value {
            assert_eq!(params.len(), 2);
            if let crate::generator::parsing::ScalarValue::Number { explicit_type, .. } = &params[1].value {
                assert_eq!(explicit_type, &Some("bigdecimal".to_string()));
            } else {
                panic!("Expected Number value with explicit_type");
            }
        } else {
            panic!("Expected Template value");
        }
    }

    #[test]
    fn parse_string_array() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <array name="fruits" type="string">
        <item>Apple</item>
        <item>Banana</item>
        <item>Orange</item>
    </array>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 1);

        let array = file
            .resources
            .iter()
            .find(|r| r.name == "fruits")
            .unwrap();
        assert_eq!(
            array.kind,
            crate::generator::parsing::ResourceKind::Array
        );
        if let crate::generator::parsing::ScalarValue::Array {
            element_type,
            spec,
            items,
        } = &array.value
        {
            assert_eq!(element_type, "string");
            assert_eq!(spec, &None);
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], "Apple");
            assert_eq!(items[1], "Banana");
            assert_eq!(items[2], "Orange");
        } else {
            panic!("Expected Array value");
        }
    }

    #[test]
    fn parse_number_array_with_bigdecimal_spec() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <array name="big_numbers" type="number" spec="bigdecimal">
        <item>12345678901234567890.123456789</item>
        <item>98765432109876543210.987654321</item>
    </array>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 1);

        let array = file
            .resources
            .iter()
            .find(|r| r.name == "big_numbers")
            .unwrap();
        assert_eq!(
            array.kind,
            crate::generator::parsing::ResourceKind::Array
        );
        if let crate::generator::parsing::ScalarValue::Array {
            element_type,
            spec,
            items,
        } = &array.value
        {
            assert_eq!(element_type, "number");
            assert_eq!(spec, &Some("bigdecimal".to_string()));
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], "12345678901234567890.123456789");
            assert_eq!(items[1], "98765432109876543210.987654321");
        } else {
            panic!("Expected Array value");
        }
    }

    #[test]
    fn parse_number_array_auto_detect() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <array name="numbers" type="number">
        <item>1</item>
        <item>2</item>
        <item>3</item>
    </array>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 1);

        let array = file
            .resources
            .iter()
            .find(|r| r.name == "numbers")
            .unwrap();
        if let crate::generator::parsing::ScalarValue::Array {
            element_type,
            spec,
            items,
        } = &array.value
        {
            assert_eq!(element_type, "number");
            assert_eq!(spec, &None);
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], "1");
            assert_eq!(items[1], "2");
            assert_eq!(items[2], "3");
        } else {
            panic!("Expected Array value");
        }
    }

    #[test]
    fn parse_bool_array() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <array name="flags" type="bool">
        <item>true</item>
        <item>false</item>
        <item>true</item>
    </array>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 1);

        let array = file
            .resources
            .iter()
            .find(|r| r.name == "flags")
            .unwrap();
        if let crate::generator::parsing::ScalarValue::Array {
            element_type,
            spec,
            items,
        } = &array.value
        {
            assert_eq!(element_type, "bool");
            assert_eq!(spec, &None);
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], "true");
            assert_eq!(items[1], "false");
            assert_eq!(items[2], "true");
        } else {
            panic!("Expected Array value");
        }
    }

    #[test]
    fn parse_color_array() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <array name="colors" type="color">
        <item>#FF0000</item>
        <item>#00FF00</item>
        <item>#0000FF</item>
    </array>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 1);

        let array = file
            .resources
            .iter()
            .find(|r| r.name == "colors")
            .unwrap();
        if let crate::generator::parsing::ScalarValue::Array {
            element_type,
            spec,
            items,
        } = &array.value
        {
            assert_eq!(element_type, "color");
            assert_eq!(spec, &None);
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], "#FF0000");
            assert_eq!(items[1], "#00FF00");
            assert_eq!(items[2], "#0000FF");
        } else {
            panic!("Expected Array value");
        }
    }

    #[test]
    fn parse_array_in_namespace() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <ns name="ui">
        <array name="themes" type="string">
            <item>light</item>
            <item>dark</item>
        </array>
    </ns>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 1);

        let array = file
            .resources
            .iter()
            .find(|r| r.name == "ui/themes")
            .unwrap();
        assert_eq!(
            array.kind,
            crate::generator::parsing::ResourceKind::Array
        );
        if let crate::generator::parsing::ScalarValue::Array { items, .. } = &array.value {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], "light");
            assert_eq!(items[1], "dark");
        } else {
            panic!("Expected Array value");
        }
    }

    #[test]
    fn parse_empty_array() {
        let raw = RawResourceFile::new(
            PathBuf::from("values.xml"),
            r#"
<resources>
    <array name="empty" type="string">
    </array>
</resources>
"#
            .into(),
            false,
        );

        let file = parse_single_file(&raw).unwrap();
        assert_eq!(file.resources.len(), 1);

        let array = file
            .resources
            .iter()
            .find(|r| r.name == "empty")
            .unwrap();
        if let crate::generator::parsing::ScalarValue::Array { items, .. } = &array.value {
            assert_eq!(items.len(), 0);
        } else {
            panic!("Expected Array value");
        }
    }
}
