use r_resources::include_resources;
include_resources!();
fn main() {
    println!("=== Namespaced Resources (v0.5.0+) ===\n");

    // Access namespaced strings via type-organized modules
    println!("Type-organized access:");
    println!("  auth title: {}", string::auth::TITLE);
    println!(
        "  auth invalid: {}",
        string::auth::errors::INVALID_CREDENTIALS
    );

    // Access via flat r:: module (Kotlin-style: r::auth::title)
    println!("\nFlat access (Kotlin-style):");
    println!("  auth title: {}", r::auth::TITLE);
    println!("  auth invalid: {}", r::auth::errors::INVALID_CREDENTIALS);

    // Colors with reference inside namespace
    println!("\nColors:");
    println!("  primary: {}", color::ui::colors::PRIMARY);
    println!("  primary (via r::): {}", r::ui::colors::PRIMARY);
    println!("  primary button: {}", color::ui::colors::PRIMARY_BUTTON);

    // Dimension
    println!("\nDimensions:");
    println!("  padding: {}", dimension::ui::dimens::PADDING);
    println!("  padding (via r::): {}", r::ui::dimens::PADDING);

    // Arrays
    println!("\nArrays:");
    println!("  langs: {:?}", string_array::lists::LANGS);
    println!("  langs (via r::): {:?}", r::lists::LANGS);
    println!("  small_numbers: {:?}", int_array::lists::SMALL_NUMBERS);
    println!("  ratios: {:?}", float_array::lists::RATIOS);
}
