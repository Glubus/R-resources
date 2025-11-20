use r_resources::{include_resources, Color, UrlParts};
include_resources!();

fn main() {
    println!("=== r-resources typed helpers ===\n");

    // You can still use the Color helper directly
    let accent = Color::new(0x33, 0x66, 0xFF, 0xFF);
    println!(
        "Color rgba=({}, {}, {}, {}) rgba_u32=0x{:08X}",
        accent.r(),
        accent.g(),
        accent.b(),
        accent.a(),
        accent.to_rgba_u32()
    );

    // UrlParts helper is also available without generated wrappers
    let api = UrlParts::new("https", "api.example.com", "/v1");
    println!("Url: {}://{}{}", api.scheme(), api.host(), api.path());

    // Generated resources remain accessible via r::
    println!("Accent hex literal from resources: {}", r::ACCENT);
}
