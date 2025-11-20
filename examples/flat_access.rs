use r_resources::include_resources;
include_resources!();

fn main() {
    println!("=== Flat Access Demo ===\n");

    // Using the flat r:: syntax - much shorter!
    println!("Strings:");
    println!("  App Name: {}", r::APP_NAME);
    println!("  Welcome: {}", r::WELCOME_MESSAGE);
    println!("  Error: {}", r::ERROR_NETWORK);

    println!("\nIntegers:");
    println!("  Max Retries: {}", r::MAX_RETRIES);
    println!("  Timeout: {}ms", r::TIMEOUT_MS);
    println!("  Cache Size: {}", r::CACHE_SIZE);

    println!("\nFloats:");
    println!("  Default Rate: {}", r::DEFAULT_RATE);
    println!("  Tax Rate: {}%", r::TAX_RATE * 100.0_f32);
    println!("  Version: {}", r::VERSION);

    println!("\nArrays:");
    println!("  Supported Languages: {:?}", r::SUPPORTED_LANGS);
    println!("  Fibonacci: {:?}", r::FIBONACCI);
    println!("  Prices: {:?}", r::PRICES);

    println!("\nAccess stays flat via r:: — no extra modules needed 🎉");
}
