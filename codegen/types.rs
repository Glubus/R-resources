/// Represents the different types of resource values that can be parsed from XML
#[derive(Debug, Clone)]
pub enum ResourceValue {
    /// A simple string value
    String(String),
    /// An integer value (i64)
    Int(i64),
    /// A floating-point value (f64)
    Float(f64),
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
}

impl ResourceValue {
    /// Returns the type name of this resource value as a String
    #[allow(dead_code)]
    pub fn type_name(&self) -> String {
        match self {
            Self::String(_) => "string".to_string(),
            Self::Int(_) => "int".to_string(),
            Self::Float(_) => "float".to_string(),
            Self::Bool(_) => "bool".to_string(),
            Self::Color(_) => "color".to_string(),
            Self::Url(_) => "url".to_string(),
            Self::Dimension(_) => "dimension".to_string(),
            Self::StringArray(_) => "string_array".to_string(),
            Self::IntArray(_) => "int_array".to_string(),
            Self::FloatArray(_) => "float_array".to_string(),
            Self::Reference { resource_type, .. } => resource_type.clone(),
        }
    }
    
    /// Parses a string and returns a `ResourceValue` (detecting references)
    pub fn parse_string_value(s: &str) -> Self {
        // Check if it's a pure reference (entire string is @type/name)
        if s.starts_with('@') && !s.contains(' ') && s.matches('@').count() == 1 {
            // Reference format: @type/name
            if let Some((resource_type, key)) = s[1..].split_once('/') {
                return Self::Reference {
                    resource_type: resource_type.to_string(),
                    key: key.to_string(),
                };
            }
        }
        
        // Otherwise it's a string (possibly with embedded references)
        Self::String(s.to_string())
    }
}


