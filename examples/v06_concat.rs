use r_resources::include_resources;
include_resources!();
fn main() {
    println!("=== String Interpolation ===");
    println!("welcome: {}", r::WELCOME_TITLE);
    println!("api_url_with_version: {}", r::API_URL_WITH_VERSION);

    println!("\n=== Template Functions ===");
    println!("greeting: {}", r::greeting("Alice", 5));
    println!("status: {}", r::status("bob", true));
}
