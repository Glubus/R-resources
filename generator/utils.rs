//! Utility functions for code generation.

/// Sanitizes an identifier to be a valid Rust identifier
///
/// Replaces non-alphanumeric characters (except underscores) with underscores
/// Formats documentation comment for Rust code generation
pub fn format_doc(doc: &Option<String>, indent: usize) -> String {
    if let Some(doc_text) = doc {
        let pad = " ".repeat(indent);
        // Split by newlines and add /// prefix to each line
        let lines: Vec<String> = doc_text
            .lines()
            .map(|line| format!("{pad}/// {}", line.trim()))
            .collect();
        if lines.is_empty() {
            String::new()
        } else {
            format!("{}\n", lines.join("\n"))
        }
    } else {
        String::new()
    }
}

pub fn sanitize_identifier(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_identifier() {
        assert_eq!(sanitize_identifier("hello-world"), "hello_world");
        assert_eq!(sanitize_identifier("app.name"), "app_name");
        assert_eq!(sanitize_identifier("my_var"), "my_var");
        assert_eq!(sanitize_identifier("test123"), "test123");
    }
}
