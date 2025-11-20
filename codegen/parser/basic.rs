use std::collections::HashMap;
use std::str::FromStr;

use crate::codegen::types::{NumberType, NumberValue, ResourceValue};

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

pub fn handle_number(
    text: &str,
    current_name: &str,
    explicit_type: Option<&str>,
    resources: &mut HashMap<String, Vec<(String, ResourceValue)>>,
) {
    if let ResourceValue::Reference {
        resource_type,
        key,
    } = ResourceValue::parse_string_value(text)
    {
        resources
            .entry("number".to_string())
            .or_default()
            .push((
                current_name.to_string(),
                ResourceValue::Reference {
                    resource_type,
                    key,
                },
            ));
        return;
    }

    match parse_number_value(text, explicit_type) {
        Ok(number_value) => {
        resources
                .entry("number".to_string())
            .or_default()
                .push((current_name.to_string(), ResourceValue::Number(number_value)));
        }
        Err(err) => panic!("{err}"),
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

fn parse_number_value(text: &str, explicit_type: Option<&str>) -> Result<NumberValue, String> {
    let literal = text.trim();
    if literal.is_empty() {
        return Err("Number literal cannot be empty".to_string());
    }

    if let Some(type_hint) = explicit_type {
        return parse_explicit_number(literal, type_hint);
    }

    if looks_like_integer(literal) {
        return literal
            .parse::<i64>()
            .map(NumberValue::Int)
            .or_else(|_| parse_big_decimal(literal));
    }

    f64::from_str(literal)
        .map(NumberValue::Float)
        .or_else(|_| parse_big_decimal(literal))
}

fn parse_big_decimal(literal: &str) -> Result<NumberValue, String> {
    bigdecimal::BigDecimal::from_str(literal)
        .map(|_| NumberValue::BigDecimal(literal.to_string()))
        .map_err(|_| format!("Invalid number literal '{literal}'"))
}

fn parse_explicit_number(literal: &str, type_hint: &str) -> Result<NumberValue, String> {
    let type_name = type_hint.trim().to_ascii_lowercase();
    let ty = match type_name.as_str() {
        "i8" => NumberType::I8,
        "i16" => NumberType::I16,
        "i32" => NumberType::I32,
        "i64" => NumberType::I64,
        "u8" => NumberType::U8,
        "u16" => NumberType::U16,
        "u32" => NumberType::U32,
        "u64" => NumberType::U64,
        "f32" => NumberType::F32,
        "f64" => NumberType::F64,
        "bigdecimal" => return parse_big_decimal(literal),
        other => return Err(format!("Unsupported number type '{other}'")),
    };

    let trimmed = literal.trim();

    let formatted = match ty {
        NumberType::I8 => trimmed
            .parse::<i8>()
            .map(|v| v.to_string())
            .map_err(|_| format!("'{trimmed}' does not fit in i8"))?,
        NumberType::I16 => trimmed
            .parse::<i16>()
            .map(|v| v.to_string())
            .map_err(|_| format!("'{trimmed}' does not fit in i16"))?,
        NumberType::I32 => trimmed
            .parse::<i32>()
            .map(|v| v.to_string())
            .map_err(|_| format!("'{trimmed}' does not fit in i32"))?,
        NumberType::I64 => trimmed
            .parse::<i64>()
            .map(|v| v.to_string())
            .map_err(|_| format!("'{trimmed}' does not fit in i64"))?,
        NumberType::U8 => trimmed
            .parse::<u8>()
            .map(|v| v.to_string())
            .map_err(|_| format!("'{trimmed}' does not fit in u8"))?,
        NumberType::U16 => trimmed
            .parse::<u16>()
            .map(|v| v.to_string())
            .map_err(|_| format!("'{trimmed}' does not fit in u16"))?,
        NumberType::U32 => trimmed
            .parse::<u32>()
            .map(|v| v.to_string())
            .map_err(|_| format!("'{trimmed}' does not fit in u32"))?,
        NumberType::U64 => trimmed
            .parse::<u64>()
            .map(|v| v.to_string())
            .map_err(|_| format!("'{trimmed}' does not fit in u64"))?,
        NumberType::F32 => trimmed
            .parse::<f32>()
            .map(format_float32)
            .map_err(|_| format!("'{trimmed}' is not a valid f32 literal"))?,
        NumberType::F64 => trimmed
            .parse::<f64>()
            .map(format_float64)
            .map_err(|_| format!("'{trimmed}' is not a valid f64 literal"))?,
    };

    Ok(NumberValue::Typed {
        literal: formatted,
        ty,
    })
}

fn looks_like_integer(literal: &str) -> bool {
    !(literal.contains('.') || literal.contains('e') || literal.contains('E'))
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
