use r_resources::include_resources;
include_resources!();

#[test]
fn namespaced_strings_and_refs() {
    assert_eq!(r::auth::TITLE, "Login");
    assert_eq!(r::auth::errors::INVALID_CREDENTIALS, "Invalid credentials");
}

#[test]
fn namespaced_colors_and_dimension() {
    assert_eq!(r::ui::colors::PRIMARY, "#3366FF");
    assert_eq!(r::ui::colors::PRIMARY_BUTTON, r::ui::colors::PRIMARY);
    assert_eq!(r::ui::dimens::PADDING, "16dp");
}

#[test]
fn namespaced_arrays() {
    assert_eq!(r::lists::LANGS, &["en", "fr", "es"][..]);
    assert_eq!(r::lists::SMALL_NUMBERS, &[1, 2, 3][..]);
    assert_eq!(r::lists::RATIOS, &[0.5f64, 1.0, 2.0][..]);
}
