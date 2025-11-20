use r_resources::include_resources;
include_resources!();
fn main() {
    println!("=== Namespaced Resources (v0.5.0+) ===\n");

    println!("All namespaced access goes through r:: now:");
    println!("  auth title: {}", r::auth::TITLE);
    println!("  auth invalid: {}", r::auth::errors::INVALID_CREDENTIALS);

    // Colors with reference inside namespace
    println!("\nColors:");
    println!("  primary: {}", r::ui::colors::PRIMARY);
    println!("  primary button: {}", r::ui::colors::PRIMARY_BUTTON);

    // Dimension
    println!("\nDimensions:");
    println!("  padding: {}", r::ui::dimens::PADDING);

    // Arrays
    println!("\nArrays:");
    println!("  langs: {:?}", r::lists::LANGS);
    println!("  small_numbers: {:?}", r::lists::SMALL_NUMBERS);
    println!("  ratios: {:?}", r::lists::RATIOS);
}
