use r_ressources::{Color, LatLng, Position, UrlParts};

fn main() {
    println!("=== r-ressources v0.4.0 (typed preview) ===\n");

    // Color typed usage
    let c = Color::new(0xFF, 0x57, 0x22, 0xFF);
    println!("Color rgba=({}, {}, {}, {}) rgba_u32=0x{:08X}", c.r(), c.g(), c.b(), c.a(), c.to_rgba_u32());

    // Url typed usage
    let api = UrlParts::new("https", "api.example.com", "/v1");
    println!("Url: {}://{}{}", api.scheme(), api.host(), api.path());

    // Position typed usage
    let p1 = Position::new(12.5, -3.75);
    let p2 = Position::new(10.0, -1.0);
    println!("Position: ({}, {}) -> dist {:.3}", p1.x(), p1.y(), p1.distance_to(&p2));

    // LatLng typed usage
    let paris = LatLng::new(48.8566, 2.3522);
    println!("LatLng: ({}, {})", paris.lat(), paris.lng());
}


