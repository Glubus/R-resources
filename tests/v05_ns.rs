#[test]
fn namespaced_strings_and_refs() {
    assert_eq!(r_ressources::string::auth::TITLE, "Login");
    assert_eq!(
        r_ressources::string::auth::errors::INVALID_CREDENTIALS,
        "Invalid credentials"
    );
}

#[test]
fn namespaced_colors_and_dimension() {
    // Raw string constants
    assert_eq!(r_ressources::color::ui::colors::PRIMARY, "#3366FF");
    // PRIMARY_BUTTON references PRIMARY
    assert_eq!(
        r_ressources::color::ui::colors::PRIMARY_BUTTON,
        r_ressources::color::ui::colors::PRIMARY
    );

    assert_eq!(r_ressources::dimension::ui::dimens::PADDING, "16dp");
}

#[test]
fn namespaced_arrays() {
    assert_eq!(
        r_ressources::string_array::lists::LANGS,
        &["en", "fr", "es"][..]
    );
    assert_eq!(r_ressources::int_array::lists::SMALL_NUMBERS, &[1, 2, 3][..]);
    assert_eq!(
        r_ressources::float_array::lists::RATIOS,
        &[0.5f64, 1.0, 2.0][..]
    );
}


