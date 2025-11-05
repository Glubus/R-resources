fn main() {
    // Access namespaced strings
    println!("auth title: {}", r_ressources::string::auth::TITLE);
    println!(
        "auth invalid: {}",
        r_ressources::string::auth::errors::INVALID_CREDENTIALS
    );

    // Colors with reference inside namespace
    println!("primary color: {}", r_ressources::color::ui::colors::PRIMARY);
    println!(
        "primary button: {}",
        r_ressources::color::ui::colors::PRIMARY_BUTTON
    );

    // Dimension
    println!("padding: {}", r_ressources::dimension::ui::dimens::PADDING);

    // Arrays
    println!("langs: {:?}", r_ressources::string_array::lists::LANGS);
    println!(
        "small_numbers: {:?}",
        r_ressources::int_array::lists::SMALL_NUMBERS
    );
    println!("ratios: {:?}", r_ressources::float_array::lists::RATIOS);
}


