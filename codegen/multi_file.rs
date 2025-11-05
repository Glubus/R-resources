/// Multi-file resource loading
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::parser;
use super::types::ResourceValue;

/// Scans the res/ directory and loads all XML resource files
pub fn load_all_resources(res_dir: &Path) -> Result<HashMap<String, Vec<(String, ResourceValue)>>, String> {
    if !res_dir.exists() {
        return Err(format!("Resource directory {:?} does not exist", res_dir));
    }

    let xml_files = find_xml_files(res_dir)?;

    if xml_files.is_empty() {
        return Err("No XML files found in res/ directory".to_string());
    }

    let mut all_resources: HashMap<String, Vec<(String, ResourceValue)>> = HashMap::new();

    for file_path in &xml_files {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read {:?}: {}", file_path, e))?;

        match parser::parse_resources(&content) {
            Ok(resources) => {
                // Merge resources from this file
                for (res_type, items) in resources {
                    all_resources
                        .entry(res_type)
                        .or_insert_with(Vec::new)
                        .extend(items);
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse {:?}: {}", file_path, e);
            }
        }
    }

    if all_resources.is_empty() {
        return Err("No resources were successfully parsed".to_string());
    }

    Ok(all_resources)
}

/// Finds all XML files in a directory (non-recursive)
fn find_xml_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut xml_files = Vec::new();

    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {:?}: {}", dir, e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "xml" {
                    xml_files.push(path);
                }
            }
        }
    }

    xml_files.sort();
    Ok(xml_files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_xml_files_nonexistent() {
        let result = find_xml_files(Path::new("nonexistent"));
        assert!(result.is_err());
    }
}

