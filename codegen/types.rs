/// Represents the explicit Rust type requested via `<number type="...">`
#[derive(Debug, Clone)]
pub enum NumberType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

impl NumberType {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::F32 => "f32",
            Self::F64 => "f64",
        }
    }
}

/// Represents a parsed numeric value
#[derive(Debug, Clone)]
pub enum NumberValue {
    /// Fits into i64
    Int(i64),
    /// Fits into f64
    Float(f64),
    /// Requires arbitrary precision
    BigDecimal(String),
    /// Explicitly typed numeric constant
    Typed { literal: String, ty: NumberType },
}

/// Represents a part of an interpolated string
#[derive(Debug, Clone)]
pub enum InterpolationPart {
    /// Literal text
    Text(String),
    /// Reference to another resource
    Reference { resource_type: String, key: String },
}

/// Represents a template parameter
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateParameterType {
    /// String parameter
    String,
    /// Integer parameter (i64)
    Int,
    /// Float parameter (f64)
    Float,
    /// Boolean parameter
    Bool,
}

/// Represents a template parameter definition
#[derive(Debug, Clone)]
pub struct TemplateParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: TemplateParameterType,
}

/// Represents a template with placeholders
#[derive(Debug, Clone)]
pub struct Template {
    /// Template string with {param} placeholders
    pub template: String,
    /// List of parameters in order
    pub parameters: Vec<TemplateParameter>,
}

/// Represents the different types of resource values that can be parsed from XML
#[derive(Debug, Clone)]
pub enum ResourceValue {
    /// A simple string value
    String(String),
    /// A numeric value with automatic precision handling
    Number(NumberValue),
    /// A boolean value
    Bool(bool),
    /// A color value (hex string like #FF5722 or #AAFF5722)
    Color(String),
    /// A URL string  
    Url(String),
    /// A dimension value with unit (e.g., "16dp", "24px", "1.5em")
    Dimension(String),
    /// An array of strings
    StringArray(Vec<String>),
    /// An array of integers
    IntArray(Vec<i64>),
    /// An array of floats
    FloatArray(Vec<f64>),
    /// A reference to another resource (e.g., @`string/app_name`)
    Reference { resource_type: String, key: String },
    /// An interpolated string with embedded references (e.g., "Welcome to @string/app_name!")
    InterpolatedString(Vec<InterpolationPart>),
    /// A template string with parameters (e.g., "Hello {name}, you have {count} messages!")
    Template(Template),
}

impl ResourceValue {
    /// Returns the type name of this resource value as a String
    #[allow(dead_code)]
    pub fn type_name(&self) -> String {
        match self {
            Self::String(_) => "string".to_string(),
            Self::Number(_) => "number".to_string(),
            Self::Bool(_) => "bool".to_string(),
            Self::Color(_) => "color".to_string(),
            Self::Url(_) => "url".to_string(),
            Self::Dimension(_) => "dimension".to_string(),
            Self::StringArray(_) => "string_array".to_string(),
            Self::IntArray(_) => "int_array".to_string(),
            Self::FloatArray(_) => "float_array".to_string(),
            Self::Reference { resource_type, .. } => resource_type.clone(),
            Self::InterpolatedString(_) => "string".to_string(),
            Self::Template(_) => "string".to_string(),
        }
    }

    /// Parses a string and returns a `ResourceValue` (detecting references and interpolations)
    pub fn parse_string_value(s: &str) -> Self {
        if let Some((resource_type, key)) = is_pure_reference(s) {
            return Self::Reference { resource_type, key };
        }

        if let Some(parts) = parse_interpolated_parts(s) {
            return Self::InterpolatedString(parts);
        }

        Self::String(s.to_string())
    }
}

/// Returns Some((type, key)) if the entire string is a pure reference like `@type/name`.
fn is_pure_reference(s: &str) -> Option<(String, String)> {
    if s.starts_with('@') && !s.contains(' ') && s.matches('@').count() == 1 {
        if let Some((resource_type, key)) = s[1..].split_once('/') {
            return Some((canonicalize_resource_type(resource_type), key.to_string()));
        }
    }
    None
}

/// Parses interpolated parts containing one or more `@type/name` references inside text.
/// Returns None if no interpolation markers are found or parsing yields no parts.
fn parse_interpolated_parts(s: &str) -> Option<Vec<InterpolationPart>> {
    if !s.contains('@') {
        return None;
    }

    let mut parts = Vec::new();
    let mut current_pos = 0;
    let bytes = s.as_bytes();

    while current_pos < bytes.len() {
        if let Some(at_pos) = find_next_at(bytes, current_pos) {
            push_text_part(bytes, current_pos, at_pos, &mut parts);

            let after_at = &s[at_pos + 1..];
            if let Some((resource_type, key, consumed)) = parse_ref_after_at(after_at) {
                parts.push(InterpolationPart::Reference { resource_type, key });
                current_pos = at_pos + 1 + consumed;
            } else {
                parts.push(InterpolationPart::Text("@".to_string()));
                current_pos = at_pos + 1;
            }
        } else {
            push_remaining_text(bytes, current_pos, &mut parts);
            break;
        }
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts)
    }
}

#[inline]
fn find_next_at(bytes: &[u8], from: usize) -> Option<usize> {
    bytes[from..]
        .iter()
        .position(|&b| b == b'@')
        .map(|rel| from + rel)
}

#[inline]
fn push_text_part(bytes: &[u8], start: usize, end: usize, parts: &mut Vec<InterpolationPart>) {
    if end > start {
        let text = String::from_utf8_lossy(&bytes[start..end]).to_string();
        if !text.is_empty() {
            parts.push(InterpolationPart::Text(text));
        }
    }
}

#[inline]
fn push_remaining_text(bytes: &[u8], start: usize, parts: &mut Vec<InterpolationPart>) {
    let text = String::from_utf8_lossy(&bytes[start..]).to_string();
    if !text.is_empty() {
        parts.push(InterpolationPart::Text(text));
    }
}

/// Parses a reference right after an '@' and returns (type, key, consumed_len)
fn parse_ref_after_at(after_at: &str) -> Option<(String, String, usize)> {
    let slash_pos = after_at.find('/')?;
    let resource_type = &after_at[..slash_pos];
    let after_slash = &after_at[slash_pos + 1..];

    let ref_end = after_slash
        .char_indices()
        .find(|(_, c)| !c.is_alphanumeric() && *c != '_' && *c != '/')
        .map(|(i, _)| i)
        .unwrap_or(after_slash.len());

    let mut key_slice = &after_slash[..ref_end];
    let had_trailing_slash = key_slice.ends_with('/');
    if had_trailing_slash {
        key_slice = &key_slice[..key_slice.len().saturating_sub(1)];
    }

    let consumed_after_at =
        1 /* '/' */ + slash_pos + ref_end - if had_trailing_slash { 1 } else { 0 };
    Some((
        canonicalize_resource_type(resource_type),
        key_slice.to_string(),
        consumed_after_at,
    ))
}

fn canonicalize_resource_type(resource_type: &str) -> String {
    match resource_type {
        "int" | "float" => "number".to_string(),
        _ => resource_type.to_string(),
    }
}
