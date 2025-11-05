/// Module containing type definitions for resource values
pub mod types;

/// Module for parsing XML resource files
pub mod parser;

/// Module for generating Rust code from parsed resources
pub mod generator;

use std::fs;
use std::path::Path;

/// Main entry point for the build process.
///
/// This function:
/// 1. Checks if res/values.xml exists
/// 2. Parses the XML file if it exists
/// 3. Generates Rust code from the parsed resources
/// 4. Writes the generated code to OUT_DIR/r_generated.rs
pub fn build() {
    println!("cargo:rerun-if-changed=res/values.xml");

    let res_path = Path::new("res/values.xml");

    if !res_path.exists() {
        eprintln!("Warning: res/values.xml not found, generating empty R struct");
        write_generated_code(&generator::generate_empty_code());
        return;
    }

    let content = fs::read_to_string(res_path).expect("Failed to read res/values.xml");

    match parser::parse_resources(&content) {
        Ok(resources) => {
            let code = generator::generate_code(&resources);
            write_generated_code(&code);
        }
        Err(e) => {
            eprintln!("Error parsing res/values.xml: {}", e);
            write_generated_code(&generator::generate_empty_code());
        }
    }
}

/// Writes the generated code to OUT_DIR/r_generated.rs
fn write_generated_code(code: &str) {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let dest_path = Path::new(&out_dir).join("r_generated.rs");

    fs::write(&dest_path, code).expect("Failed to write generated code");
}
