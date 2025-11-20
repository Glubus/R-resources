use r_resources::include_resources;
include_resources!();

fn main() {
    println!("=== R Resources Demo ===\n");

    // Strings
    println!("Strings:");
    println!("  App Name: {}", r::APP_NAME);
    println!("  Welcome: {}", r::WELCOME_MESSAGE);
    println!("  Error: {}", r::ERROR_NETWORK);

    // Ints
    println!("\nIntegers:");
    println!("  Max Retries: {}", r::MAX_RETRIES);
    println!("  Timeout: {}ms", r::TIMEOUT_MS);
    println!("  Cache Size: {}", r::CACHE_SIZE);

    // Floats
    println!("\nFloats:");
    println!("  Default Rate: {}", r::DEFAULT_RATE);
    println!("  Tax Rate: {}%", r::TAX_RATE * 100.0_f32);
    println!("  Version: {}", r::VERSION);

    // Arrays
    println!("\nArrays:");
    println!("  Supported Languages: {:?}", r::SUPPORTED_LANGS);
    println!("  Fibonacci: {:?}", r::FIBONACCI);
    println!("  Prices: {:?}", r::PRICES);
}
