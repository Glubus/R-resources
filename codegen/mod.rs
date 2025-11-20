//! Build-time code generation for resources

/// Module containing type definitions for resource values
pub mod types;

/// Module for parsing XML resource files
pub mod parser;

/// Code generation modules
pub mod generator;

/// Utility functions
pub mod utils;

/// Multi-file resource loading
pub mod multi_file;

/// Reference resolution
pub mod references;

/// Environment/profile support
pub mod environment;

use std::fs;
use std::path::{Path, PathBuf};

/// Main entry point for the build process.
///
/// This function:
/// 1. Scans the res/ directory for all XML files
/// 2. Parses and merges resources from all files
/// 3. Generates Rust code from the parsed resources
/// 4. Writes the generated code to `OUT_DIR/r_generated.rs`
pub fn build() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable not set");
    let res_dir = Path::new(&manifest_dir).join("res");
    let tests_dir = res_dir.join("tests");
    let include_tests =
        tests_dir.exists()
            && (std::env::var("CARGO_CFG_TEST").is_ok()
                || std::env::var("R_RESOURCES_INCLUDE_TESTS").is_ok());
    let options = BuildOptions {
        res_dir: Some(res_dir),
        tests_res_dir: include_tests.then_some(tests_dir),
    };
    build_with_options(&options);
}

#[derive(Clone, Default)]
pub struct BuildOptions {
    pub res_dir: Option<PathBuf>,
    pub tests_res_dir: Option<PathBuf>,
}

pub fn build_with_options(options: &BuildOptions) {
    println!("cargo:rustc-check-cfg=cfg(r_resources_has_tests)");
    let res_dir = options
        .res_dir
        .clone()
        .or_else(|| {
            std::env::var("CARGO_MANIFEST_DIR")
                .ok()
                .map(|dir| Path::new(&dir).join("res"))
        })
        .unwrap_or_else(|| PathBuf::from("res"));

    println!("cargo:rerun-if-changed={}", res_dir.display());
    if let Some(tests) = &options.tests_res_dir {
        println!("cargo:rerun-if-changed={}", tests.display());
    }

    if !res_dir.exists() {
        eprintln!("Warning: res/ directory not found, generating empty R struct");
        write_generated_code(&generator::generate_empty_code());
        return;
    }

    let resources = match multi_file::load_all_resources(&res_dir) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: Failed to load resources: {e}");
            std::process::exit(1);
        }
    };

            let ref_errors = references::validate_references(&resources);
            if !ref_errors.is_empty() {
                eprintln!("error: Reference validation failed:");
                for error in &ref_errors {
                    eprintln!("  {error}");
                }
                std::process::exit(1);
            }

    let test_code = if let Some(tests_dir) = &options.tests_res_dir {
        if tests_dir.exists() {
            match multi_file::load_all_resources(tests_dir) {
                Ok(test_resources) => Some(test_resources),
        Err(e) => {
                    eprintln!("warning: Failed to load test resources: {e}");
                    None
                }
            }
        } else {
            None
        }
    } else {
        None
    };

    if test_code.is_some() {
        println!("cargo:rustc-cfg=r_resources_has_tests");
    }

    let code = match test_code {
        Some(tests) => generator::generate_code_with_tests(&resources, &tests),
        None => generator::generate_code(&resources),
    };

    write_generated_code(&code);
}

/// Writes the generated code to `OUT_DIR/r_generated.rs`
fn write_generated_code(code: &str) {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let dest_path = Path::new(&out_dir).join("r_generated.rs");

    fs::write(&dest_path, code).expect("Failed to write generated code");
}
