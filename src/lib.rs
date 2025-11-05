//! # r-ressources
//!
//! Android-style resource management for Rust with compile-time type safety.
//!
//! This library provides a build-time resource management system inspired by Android's `R` class.
//! Resources are defined in an XML file and compiled into type-safe Rust constants at build time,
//! resulting in zero runtime overhead.
//!
//! ## Quick Start
//!
//! 1. Create a `res/values.xml` file in your project root:
//!
//! ```xml
//! <?xml version="1.0" encoding="utf-8"?>
//! <resources>
//!     <string name="app_name">My Application</string>
//!     <int name="max_retries">3</int>
//!     <float name="version">1.0</float>
//! </resources>
//! ```
//!
//! 2. Access resources in your code:
//!
//! ```rust
//! use r_ressources::*;
//!
//! // Option 1: Type-organized access
//! println!("App: {}", string::APP_NAME);
//! println!("Max retries: {}", int::MAX_RETRIES);
//! println!("Version: {}", float::VERSION);
//!
//! // Option 2: Flat access via r module
//! println!("App: {}", r::APP_NAME);
//! println!("Max retries: {}", r::MAX_RETRIES);
//! println!("Version: {}", r::VERSION);
//! ```
//!
//! ## Supported Resource Types
//!
//! - **Strings**: `<string name="key">value</string>` → `string::KEY` or `r::KEY`
//! - **Integers**: `<int name="key">42</int>` → `int::KEY` or `r::KEY`
//! - **Floats**: `<float name="key">3.14</float>` → `float::KEY` or `r::KEY`
//! - **String Arrays**: `<string-array name="key">...</string-array>` → `string_array::KEY` or `r::KEY`
//! - **Integer Arrays**: `<int-array name="key">...</int-array>` → `int_array::KEY` or `r::KEY`
//! - **Float Arrays**: `<float-array name="key">...</float-array>` → `float_array::KEY` or `r::KEY`
//!
//! Both access methods are available:
//! - Type-organized: `string::APP_NAME` (clearer, avoids naming conflicts)
//! - Flat access: `r::APP_NAME` (shorter, more convenient)
//!
//! ## Features
//!
//! - **Build-time compilation**: All resources are compiled into your binary
//! - **Type-safe**: Each resource type has its own module
//! - **Zero runtime cost**: Direct constant access, no parsing or lookups
//! - **Thread-safe**: All resources are `const` and can be safely accessed from any thread
//! - **Async-safe**: Works seamlessly in async contexts (tokio, async-std, etc.)
//! - **Familiar syntax**: Inspired by Android's resource system
//!
//! ## Thread Safety
//!
//! All generated resources are `const` values, making them inherently thread-safe:
//!
//! ```rust
//! use std::thread;
//! use r_ressources::*;
//!
//! let handles: Vec<_> = (0..10)
//!     .map(|_| {
//!         thread::spawn(|| {
//!             // Safe to access from multiple threads
//!             println!("{}", string::APP_NAME);
//!         })
//!     })
//!     .collect();
//!
//! for handle in handles {
//!     handle.join().unwrap();
//! }
//! ```

// Include the generated R struct and modules
include!(concat!(env!("OUT_DIR"), "/r_generated.rs"));

/// Error types for resource operations
///
/// This enum represents all possible errors that can occur when working with resources.
/// Currently, errors are primarily used for validation and future extensibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RError {
    /// The requested resource does not exist
    ///
    /// Contains the resource type (e.g., "string") and the key that was not found
    ResourceNotFound {
        /// The type of resource that was requested (e.g., "string", "int")
        resource_type: String,
        /// The key that was not found
        key: String,
    },
    /// The resource file is invalid or cannot be parsed
    ///
    /// Contains the path to the file and the reason it's invalid
    InvalidResourceFile {
        /// Path to the invalid resource file
        path: String,
        /// Description of why the file is invalid
        reason: String,
    },
    /// A type mismatch occurred when accessing a resource
    ///
    /// Contains the expected type and the actual type found
    TypeMismatch {
        /// The type that was expected
        expected: String,
        /// The type that was actually found
        found: String,
    },
}

impl std::fmt::Display for RError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RError::ResourceNotFound { resource_type, key } => {
                write!(f, "Resource not found: {}.{}", resource_type, key)
            }
            RError::InvalidResourceFile { path, reason } => {
                write!(f, "Invalid resource file '{}': {}", path, reason)
            }
            RError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
        }
    }
}

impl std::error::Error for RError {}

/// Result type for resource operations
///
/// This is a convenience type alias for `Result<T, RError>`.
pub type RResult<T> = Result<T, RError>;

