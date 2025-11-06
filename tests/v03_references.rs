/// Tests for v0.3.0 resource references
use r_ressources::include_resources;

include_resources!();

#[test]
fn test_string_references() {
    // Base resources
    assert_eq!(string::BASE_URL, "https://api.example.com");
    assert_eq!(string::API_VERSION, "v2");

    // References should resolve to the same value
    assert_eq!(string::FULL_API_URL, string::BASE_URL);
    assert_eq!(string::APP_TITLE, string::APP_NAME);
}

#[test]
fn test_color_references() {
    // Base color
    assert_eq!(color::ACCENT, "#FF5722");

    // Reference should resolve to same color
    assert_eq!(color::BUTTON_COLOR, color::ACCENT);
    assert_eq!(color::BUTTON_COLOR, "#FF5722");
}

#[test]
fn test_flat_access_with_references() {
    // Flat access should also work with references
    assert_eq!(r::FULL_API_URL, r::BASE_URL);
    assert_eq!(r::BUTTON_COLOR, r::ACCENT);
}

#[test]
fn test_reference_chains() {
    // References can point to other resources
    assert_eq!(string::APP_TITLE, "My Awesome App");
    assert_eq!(string::FULL_API_URL, "https://api.example.com");
}

#[test]
fn test_cross_type_resource_access() {
    // Verify that all resource types coexist properly by touching them
    std::hint::black_box(string::BASE_URL);
    std::hint::black_box(color::ACCENT);
    std::hint::black_box(int::MAX_RETRIES);
    std::hint::black_box(bool::DEBUG_MODE);
}
