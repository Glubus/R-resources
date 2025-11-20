use r_resources::include_resources;
include_resources!();
fn main() {
    println!("=== r-resources v0.3.0 - References Demo ===\n");

    // Base resources
    println!("Base resources:");
    println!("  base_url: {}", r::BASE_URL);
    println!("  api_version: {}", r::API_VERSION);
    println!("  app_name: {}", r::APP_NAME);

    // Resources that reference other resources
    println!("\nReferenced resources:");
    println!(
        "  full_api_url: {} (references @string/base_url)",
        r::FULL_API_URL
    );
    println!(
        "  app_title: {} (references @string/app_name)",
        r::APP_TITLE
    );

    // Colors with references
    println!("\nColor references:");
    println!("  accent: {}", r::ACCENT);
    println!(
        "  button_color: {} (references @color/accent)",
        r::BUTTON_COLOR
    );

    // Verify references work correctly
    println!("\n=== Verification ===");
    println!(
        "full_api_url == base_url: {}",
        r::FULL_API_URL == r::BASE_URL
    );
    println!(
        "app_title == app_name: {}",
        r::APP_TITLE == r::APP_NAME
    );
    println!(
        "button_color == accent: {}",
        r::BUTTON_COLOR == r::ACCENT
    );

    println!("\nEverything is available via r:: by default 🚀");
    println!("r::FULL_API_URL: {}", r::FULL_API_URL);
    println!("r::BUTTON_COLOR: {}", r::BUTTON_COLOR);
}
