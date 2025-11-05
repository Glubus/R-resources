/// Build script for r-ressources
///
/// This build script generates Rust code from XML resource files at compile time.
/// It reads `res/values.xml` from the project root and generates type-safe constants
/// for all defined resources.
///
/// # Resource Types
///
/// Supported resource types:
/// - `<string>`: String constants
/// - `<int>`: Integer constants (i64)
/// - `<float>`: Float constants (f64)
/// - `<string-array>`: String array constants
/// - `<int-array>`: Integer array constants
/// - `<float-array>`: Float array constants
///
/// # Example
///
/// Given this `res/values.xml`:
///
/// ```xml
/// <?xml version="1.0" encoding="utf-8"?>
/// <resources>
///     <string name="app_name">My App</string>
///     <int name="max_retries">3</int>
/// </resources>
/// ```
///
/// The build script generates:
///
/// ```rust
/// pub mod string {
///     pub const APP_NAME: &str = "My App";
/// }
///
/// pub mod int {
///     pub const MAX_RETRIES: i64 = 3;
/// }
/// ```
mod codegen;

fn main() {
    codegen::build();
}
