/// Tests for basic resource types (v0.1.0)
use r_resources::include_resources;

include_resources!();
#[test]
fn test_string_resources() {
    assert_eq!(r::APP_NAME, "My Awesome App");
    assert_eq!(r::WELCOME_MESSAGE, "Welcome to our app!");
}

#[test]
fn test_number_resources() {
    assert_eq!(r::MAX_RETRIES, 3);
    assert_eq!(r::CACHE_SIZE, 100);
    assert!((r::DEFAULT_RATE - 0.75).abs() < f64::EPSILON);
    assert!((r::TAX_RATE - 0.20_f32).abs() < f32::EPSILON);
    assert!((r::PI_PRECISE - 3.141592653589793).abs() < 1e-12);

    match std::env::var("PROFILE").as_deref() {
        Ok("release") => assert_eq!(r::TIMEOUT_MS, 5000),
        _ => assert_eq!(r::TIMEOUT_MS, 10000),
    }
}

#[test]
fn test_big_decimal_numbers() {
    assert_eq!(
        r::HUGE_BALANCE.to_string(),
        "123456789012345678901234567890123456789"
    );
}

#[test]
fn test_array_resources() {
    assert_eq!(r::SUPPORTED_LANGS, &["en", "fr", "es"]);
    assert_eq!(r::FIBONACCI, &[1, 1, 2, 3, 5, 8]);
    assert_eq!(r::PRICES, &[9.99, 19.99, 29.99]);
}

#[test]
fn test_explicit_number_types() {
    fn expect_i32(_: i32) {}
    fn expect_u32(_: u32) {}
    expect_i32(r::MAX_RETRIES);
    expect_u32(r::CACHE_SIZE);
}
