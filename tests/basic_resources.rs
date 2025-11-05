/// Tests for basic resource types (v0.1.0)
use r_ressources::*;

#[test]
fn test_string_resources() {
    assert_eq!(string::APP_NAME, "My Awesome App");
    assert_eq!(string::WELCOME_MESSAGE, "Welcome to our app!");
    
    // Test flat access
    assert_eq!(r::APP_NAME, "My Awesome App");
    assert_eq!(r::WELCOME_MESSAGE, "Welcome to our app!");
    
    // Both should be the same
    assert_eq!(string::APP_NAME, r::APP_NAME);
}

#[test]
fn test_int_resources() {
    assert_eq!(int::MAX_RETRIES, 3);
    // TIMEOUT_MS depends on build profile (debug: 10000, release: 5000)
    match std::env::var("PROFILE").as_deref() {
        Ok("release") => assert_eq!(int::TIMEOUT_MS, 5000),
        _ => assert_eq!(int::TIMEOUT_MS, 10000),
    }
}

#[test]
fn test_float_resources() {
    assert!((float::DEFAULT_RATE - 0.75).abs() < f64::EPSILON);
    assert!((float::TAX_RATE - 0.20).abs() < f64::EPSILON);
}

#[test]
fn test_array_resources() {
    assert_eq!(string_array::SUPPORTED_LANGS, &["en", "fr", "es"]);
    assert_eq!(int_array::FIBONACCI, &[1, 1, 2, 3, 5, 8]);
    assert_eq!(float_array::PRICES, &[9.99, 19.99, 29.99]);
}

