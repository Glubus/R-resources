/// Tests for error types and handling
use r_ressources::*;

#[test]
fn test_error_display() {
    let err = RError::ResourceNotFound {
        resource_type: "string".to_string(),
        key: "missing_key".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "Resource not found: string.missing_key"
    );
}

#[test]
fn test_invalid_resource_file_error() {
    let err = RError::InvalidResourceFile {
        path: "res/invalid.xml".to_string(),
        reason: "Malformed XML".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "Invalid resource file 'res/invalid.xml': Malformed XML"
    );
}

#[test]
fn test_type_mismatch_error() {
    let err = RError::TypeMismatch {
        expected: "int".to_string(),
        found: "string".to_string(),
    };
    assert_eq!(err.to_string(), "Type mismatch: expected int, found string");
}

#[test]
fn test_error_trait_implementation() {
    let err = RError::ResourceNotFound {
        resource_type: "color".to_string(),
        key: "primary".to_string(),
    };
    
    // Test that it implements std::error::Error
    let _error_trait: &dyn std::error::Error = &err;
}

