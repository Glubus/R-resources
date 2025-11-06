use r_ressources::{color, r, string};

fn main() {
    println!("=== r-ressources v0.3.0 - References Demo ===\n");

    // Base resources
    println!("Base resources:");
    println!("  base_url: {}", string::BASE_URL);
    println!("  api_version: {}", string::API_VERSION);
    println!("  app_name: {}", string::APP_NAME);

    // Resources that reference other resources
    println!("\nReferenced resources:");
    println!(
        "  full_api_url: {} (references @string/base_url)",
        string::FULL_API_URL
    );
    println!(
        "  app_title: {} (references @string/app_name)",
        string::APP_TITLE
    );

    // Colors with references
    println!("\nColor references:");
    println!("  accent: {}", color::ACCENT);
    println!(
        "  button_color: {} (references @color/accent)",
        color::BUTTON_COLOR
    );

    // Verify references work correctly
    println!("\n=== Verification ===");
    println!(
        "full_api_url == base_url: {}",
        string::FULL_API_URL == string::BASE_URL
    );
    println!(
        "app_title == app_name: {}",
        string::APP_TITLE == string::APP_NAME
    );
    println!(
        "button_color == accent: {}",
        color::BUTTON_COLOR == color::ACCENT
    );

    // Flat access also works
    println!("\n=== Flat access ===");
    println!("r::FULL_API_URL: {}", r::FULL_API_URL);
    println!("r::BUTTON_COLOR: {}", r::BUTTON_COLOR);
}
