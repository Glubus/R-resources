use r_ressources::{color, int, r, url};

fn main() {
    println!("=== r-ressources v0.2.0 - New Types Demo ===\n");

    // Boolean values
    println!("Boolean values:");
    println!("  Debug Mode: {}", r::DEBUG_MODE);
    println!("  Enable Analytics: {}", r::ENABLE_ANALYTICS);
    println!("  Show Tutorial: {}", r::SHOW_TUTORIAL);

    // Color values (hex strings)
    println!("\nColor values:");
    println!("  Primary: {}", r::PRIMARY);
    println!("  Secondary: {}", r::SECONDARY);
    println!("  Background: {}", r::BACKGROUND);
    println!("  Text Dark: {}", r::TEXT_DARK);

    // URL values
    println!("\nURL values:");
    println!("  API Base: {}", r::API_BASE);
    println!("  Website: {}", r::WEBSITE);
    println!("  Docs: {}", r::DOCS);

    // Dimension values
    println!("\nDimension values:");
    println!("  Padding Small: {}", r::PADDING_SMALL);
    println!("  Padding Medium: {}", r::PADDING_MEDIUM);
    println!("  Padding Large: {}", r::PADDING_LARGE);
    println!("  Font Size: {}", r::FONT_SIZE);

    // Mixed usage - old and new types together
    println!("\n=== Practical Example ===");
    println!("App: {} (v{})", r::APP_NAME, r::VERSION);
    println!("Theme: Primary={}, Secondary={}", color::PRIMARY, color::SECONDARY);
    println!("API: {}", url::API_BASE);
    println!("Debug: {}", if r::DEBUG_MODE { "ON" } else { "OFF" });
    println!("Max Retries: {}", int::MAX_RETRIES);
}

