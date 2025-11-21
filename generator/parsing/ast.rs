use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ParsedResourceFile {
    pub path: PathBuf,
    pub is_test: bool,
    pub resources: Vec<ParsedResource>,
}

impl ParsedResourceFile {
    pub fn new(
        path: PathBuf,
        is_test: bool,
        resources: Vec<ParsedResource>,
    ) -> Self {
        Self {
            path,
            is_test,
            resources,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedResource {
    pub name: String,
    pub kind: ResourceKind,
    pub value: ScalarValue,
    pub doc: Option<String>, // Documentation comment
}

impl ParsedResource {
    pub fn string(
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            kind: ResourceKind::String,
            value: ScalarValue::Text(value.into()),
            doc: None,
        }
    }

    pub fn number(
        name: impl Into<String>,
        value: impl Into<String>,
        explicit_type: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            kind: ResourceKind::Number,
            value: ScalarValue::Number {
                value: value.into(),
                explicit_type,
            },
            doc: None,
        }
    }

    pub fn bool(name: impl Into<String>, value: bool) -> Self {
        Self {
            name: name.into(),
            kind: ResourceKind::Bool,
            value: ScalarValue::Bool(value),
            doc: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceKind {
    String,
    Number,
    Bool,
    Color,
    Template,
    Array,
    Namespace, // For <ns> tags
    Item,      // For <item> tags in arrays
    Doc,       // For <doc> tags
}

impl ResourceKind {
    /// Converts a string tag name to a `ResourceKind`.
    pub fn from_str(tag: &str) -> Self {
        match tag {
            "ns" => Self::Namespace,
            "template" => Self::Template,
            "array" => Self::Array,
            "string" => Self::String,
            "number" | "int" | "float" => Self::Number,
            "bool" => Self::Bool,
            "color" => Self::Color,
            "item" => Self::Item,
            _ => Self::String, // Default fallback
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalarValue {
    Text(String),
    Number {
        value: String,
        explicit_type: Option<String>, // For type="i32", etc.
    },
    Bool(bool),
    Color(String),
    Template {
        text: String,
        params: Vec<TemplateParam>,
    },
    Array {
        element_type: String, // "string", "number", "bool", "color"
        spec: Option<String>, // For numbers: "i64", "f64", "bigdecimal", etc.
        items: Vec<String>, // Array items as strings (will be parsed based on type)
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateParam {
    pub name: String,
    pub value: ScalarValue, // Use ScalarValue to represent the parameter (reuses existing parsing logic)
}

impl ScalarValue {
    #[allow(dead_code)] // Used in tests
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value.as_str()),
            Self::Number { value, .. } => Some(value.as_str()),
            Self::Bool(_) => None,
            Self::Color(_) => None,
            Self::Template { text, .. } => Some(text.as_str()),
            Self::Array { .. } => None,
        }
    }

    #[allow(dead_code)] // Used in tests
    pub fn as_number(&self) -> Option<&str> {
        match self {
            Self::Number { value, .. } => Some(value.as_str()),
            _ => None,
        }
    }

    #[allow(dead_code)] // Used in tests
    pub fn number_explicit_type(&self) -> Option<&str> {
        match self {
            Self::Number { explicit_type, .. } => {
                explicit_type.as_deref()
            }
            _ => None,
        }
    }

    #[allow(dead_code)] // Used in tests
    pub fn as_color(&self) -> Option<&str> {
        match self {
            Self::Color(value) => Some(value.as_str()),
            _ => None,
        }
    }

    #[allow(dead_code)] // Used in tests
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }
}
