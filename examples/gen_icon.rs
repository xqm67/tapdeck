//! Genera `res/icon.ico` con il marchio TapDeck ("T" bianca + 4 tasti colorati).
//! Usa la stessa funzione di `icons.rs` usata per l'icona della finestra e della PWA
//! (include `src/brand.rs`, che non dipende dal resto del crate).
//! Uso: cargo run --example gen_icon
use std::fs;
use std::path::PathBuf;

use image::ExtendedColorType;
use image::codecs::ico::{IcoEncoder, IcoFrame};

#[path = "../src/brand.rs"]
mod brand;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("res"));
    fs::create_dir_all(&out_dir)?;
    let mut frames = Vec::new();
    for size in [16u32, 24, 32, 48, 64, 128, 256] {
        let img = brand::brand_icon_rgba(size);
        let frame = IcoFrame::as_png(img.as_raw(), size, size, ExtendedColorType::Rgba8)?;
        frames.push(frame);
    }
    let path = out_dir.join("icon.ico");
    let file = fs::File::create(&path)?;
    IcoEncoder::new(file).encode_images(&frames)?;
    println!("Scritto {}", path.display());
    Ok(())
}
