//! Disegno del marchio TapDeck: unico punto di verità per l'icona.
//! Nessuna dipendenza dal resto del crate: è incluso sia da `icons.rs`
//! (icona finestra e PWA) sia da `examples/gen_icon.rs` (res/icon.ico).

use image::RgbaImage;

/// Disegna il marchio a risoluzione 4x (bordi lisci, senza font esterni):
/// gradiente scuro + "T" bianca con angoli arrotondati + 4 tasti colorati in basso.
pub fn brand_icon_rgba(size: u32) -> RgbaImage {
    const SS: u32 = 4;
    let s = (size * SS).max(64);
    let mut img = RgbaImage::new(s, s);

    // Sfondo a gradiente verticale scuro.
    let top = [30u8, 33, 46];
    let bot = [11u8, 12, 18];
    for y in 0..s {
        let t = y as f32 / s as f32;
        let r = (top[0] as f32 * (1.0 - t) + bot[0] as f32 * t) as u8;
        let g = (top[1] as f32 * (1.0 - t) + bot[1] as f32 * t) as u8;
        let b = (top[2] as f32 * (1.0 - t) + bot[2] as f32 * t) as u8;
        for x in 0..s {
            img.put_pixel(x, y, image::Rgba([r, g, b, 255]));
        }
    }

    // "T" bianca: barra orizzontale + gambo verticale, angoli arrotondati.
    let white = [240u8, 242, 247];
    let f = |v: f32| v * s as f32;
    fill_rounded_rect(&mut img, f(0.20), f(0.17), f(0.80), f(0.34), f(0.05), white);
    fill_rounded_rect(&mut img, f(0.43), f(0.28), f(0.57), f(0.78), f(0.05), white);

    // 4 tasti tondi colorati (2x2) in basso a destra, come una Stream Deck.
    let colors: [[u8; 3]; 4] = [
        [88, 101, 242], // blu
        [145, 70, 255], // viola
        [29, 185, 84],  // verde
        [229, 9, 20],   // rosso
    ];
    let pos = [
        (0.695, 0.695),
        (0.825, 0.695),
        (0.695, 0.825),
        (0.825, 0.825),
    ];
    for (i, (cx, cy)) in pos.into_iter().enumerate() {
        fill_circle(&mut img, f(cx), f(cy), f(0.048), colors[i]);
    }

    image::imageops::resize(&img, size, size, image::imageops::FilterType::Triangle)
}

/// Riempie un cerchio pieno.
fn fill_circle(img: &mut RgbaImage, cx: f32, cy: f32, r: f32, col: [u8; 3]) {
    let r2 = r * r;
    let y0 = (cy - r).max(0.0) as u32;
    let y1 = ((cy + r).min(img.height() as f32 - 1.0)) as u32;
    let x0 = (cx - r).max(0.0) as u32;
    let x1 = ((cx + r).min(img.width() as f32 - 1.0)) as u32;
    for y in y0..=y1 {
        for x in x0..=x1 {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            if dx * dx + dy * dy <= r2 {
                img.put_pixel(x, y, image::Rgba([col[0], col[1], col[2], 255]));
            }
        }
    }
}

/// Riempie un rettangolo con angoli arrotondati (test di distanza dal bordo).
fn fill_rounded_rect(
    img: &mut RgbaImage,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    r: f32,
    col: [u8; 3],
) {
    let rr = r * r;
    let y_start = y0.floor().max(0.0) as u32;
    let y_end = y1.ceil().min(img.height() as f32 - 1.0) as u32;
    let x_start = x0.floor().max(0.0) as u32;
    let x_end = x1.ceil().min(img.width() as f32 - 1.0) as u32;
    for y in y_start..=y_end {
        for x in x_start..=x_end {
            let fx = x as f32 + 0.5;
            let fy = y as f32 + 0.5;
            // Punto più vicino del rettangolo interno; se distanza <= r, è dentro.
            let cx = fx.clamp(x0 + r, x1 - r);
            let cy = fy.clamp(y0 + r, y1 - r);
            let dx = fx - cx;
            let dy = fy - cy;
            if dx * dx + dy * dy <= rr {
                img.put_pixel(x, y, image::Rgba([col[0], col[1], col[2], 255]));
            }
        }
    }
}
