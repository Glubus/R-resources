/// Tests for v0.2.0 new resource types
use r_resources::include_resources;

include_resources!();

#[test]
fn test_bool_resources() {
    // Touch constants (avoids constant assertions under clippy strict)
    std::hint::black_box(r::DEBUG_MODE);
    std::hint::black_box(r::ENABLE_ANALYTICS);
    std::hint::black_box(r::SHOW_TUTORIAL);
}

#[test]
fn test_color_resources() {
    assert_eq!(r::PRIMARY, "#FF5722");
    assert_eq!(r::SECONDARY, "#03A9F4");
    assert_eq!(r::BACKGROUND, "#FFFFFF");
    assert_eq!(r::TEXT_DARK, "#212121");
}

#[test]
fn test_url_resources() {
    assert_eq!(r::API_BASE, "https://api.example.com");
    assert_eq!(r::WEBSITE, "https://example.com");
    assert_eq!(r::DOCS, "https://docs.example.com");
}

#[test]
fn test_dimension_resources() {
    assert_eq!(r::PADDING_SMALL, "8dp");
    assert_eq!(r::PADDING_MEDIUM, "16dp");
    assert_eq!(r::PADDING_LARGE, "24dp");
    assert_eq!(r::FONT_SIZE, "14sp");
}

#[test]
fn test_multi_file_loading() {
    // Resources from values.xml
    assert_eq!(r::APP_NAME, "My Awesome App");
    assert_eq!(r::MAX_RETRIES, 3);

    // Resources from config.xml
    std::hint::black_box(r::DEBUG_MODE);
    assert_eq!(r::PRIMARY, "#FF5722");
    assert_eq!(r::API_BASE, "https://api.example.com");
    assert_eq!(r::PADDING_SMALL, "8dp");

    // All accessible via r:: module
    assert_eq!(r::APP_NAME, "My Awesome App");
}
