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
}

impl ResourceValue {
    /// Returns the type name of this resource value
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            ResourceValue::String(_) => "string",
            ResourceValue::Int(_) => "int",
            ResourceValue::Float(_) => "float",
            ResourceValue::Bool(_) => "bool",
            ResourceValue::Color(_) => "color",
            ResourceValue::Url(_) => "url",
            ResourceValue::Dimension(_) => "dimension",
            ResourceValue::StringArray(_) => "string_array",
            ResourceValue::IntArray(_) => "int_array",
            ResourceValue::FloatArray(_) => "float_array",
        }
    }
}
