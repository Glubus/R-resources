// Inclut le code généré (modules r, string, int, float, bool, etc.)
use r_resources::include_resources;
include_resources!();


fn main() {

    // Accès plat
    println!("App: {}", r::APP_NAME);
    println!("Max retries: {}", r::MAX_RETRIES);
    println!("Tax rate: {}%", r::TAX_RATE * 100.0);
    println!("Debug mode: {}", r::DEBUG_MODE);
    println!("Langs: {:?}", r::SUPPORTED_LANGS);
    // Accès typé
    println!("(typed) App: {}", string::APP_NAME);
    println!("(typed) Max retries: {}", int::MAX_RETRIES);
}