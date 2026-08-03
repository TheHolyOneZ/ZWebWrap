use crate::paths;
use crate::site::IconCandidate;
use anyhow::{Context, Result};
use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

pub const ICON_SIZE: u32 = 256;

const MAX_ICON_BYTES: usize = 4 * 1024 * 1024;

pub async fn resolve_and_save(
    client: &reqwest::Client,
    candidates: &[IconCandidate],
    id: &str,
    title: &str,
) -> PathBuf {
    for candidate in candidates.iter().take(6) {
        match download_and_decode(client, &candidate.url).await {
            Ok(img) => match save_png(&normalize(img), id) {
                Ok(path) => return path,
                Err(e) => tracing::warn!(id, error = %e, "could not write icon file"),
            },
            Err(e) => tracing::debug!(url = %candidate.url, error = %e, "icon candidate rejected"),
        }
    }

    tracing::info!(id, "no usable icon found; generating a letter tile");
    let tile = DynamicImage::ImageRgba8(letter_tile(title, id));
    save_png(&tile, id).unwrap_or_else(|e| {
        tracing::error!(id, error = %e, "could not write fallback icon");
        PathBuf::new()
    })
}

pub fn save_custom(bytes: &[u8], id: &str) -> Result<PathBuf> {
    anyhow::ensure!(!bytes.is_empty(), "that file is empty");
    anyhow::ensure!(
        bytes.len() <= MAX_ICON_BYTES,
        "that image is {} MB; pick one under {} MB",
        bytes.len() / 1_048_576,
        MAX_ICON_BYTES / 1_048_576
    );

    let image =
        image::load_from_memory(bytes).context("that file is not an image ZWebWrap can read")?;
    save_png(&normalize(image), id)
}

async fn download_and_decode(client: &reqwest::Client, url: &str) -> Result<DynamicImage> {
    let response = client.get(url).send().await?.error_for_status()?;

    if let Some(len) = response.content_length() {
        anyhow::ensure!(len as usize <= MAX_ICON_BYTES, "icon is {len} bytes, too large");
    }
    let bytes = response.bytes().await?;
    anyhow::ensure!(!bytes.is_empty(), "icon response was empty");
    anyhow::ensure!(bytes.len() <= MAX_ICON_BYTES, "icon is {} bytes, too large", bytes.len());

    image::load_from_memory(&bytes).context("icon bytes did not decode as an image")
}

fn normalize(img: DynamicImage) -> DynamicImage {
    let (w, h) = (img.width(), img.height());
    if w == 0 || h == 0 {
        return DynamicImage::ImageRgba8(RgbaImage::new(ICON_SIZE, ICON_SIZE));
    }
    if w == ICON_SIZE && h == ICON_SIZE {
        return img;
    }
    let filter = if w.max(h) < 64 { FilterType::Nearest } else { FilterType::Lanczos3 };

    let scaled = img.resize(ICON_SIZE, ICON_SIZE, filter);
    let mut canvas = RgbaImage::new(ICON_SIZE, ICON_SIZE);
    let x = (ICON_SIZE - scaled.width()) / 2;
    let y = (ICON_SIZE - scaled.height()) / 2;
    image::imageops::overlay(&mut canvas, &scaled.to_rgba8(), x as i64, y as i64);
    DynamicImage::ImageRgba8(canvas)
}

fn save_png(img: &DynamicImage, id: &str) -> Result<PathBuf> {
    let path = paths::icon_dir()?.join(format!("{id}.png"));
    img.save_with_format(&path, ImageFormat::Png)
        .with_context(|| format!("writing {}", path.display()))?;
    Ok(path)
}

pub fn remove_icon(id: &str) -> Result<()> {
    let dir = paths::icon_dir()?;
    for extension in ["png", "ico"] {
        let path = dir.join(format!("{id}.{extension}"));
        match std::fs::remove_file(&path) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e).with_context(|| format!("removing {}", path.display())),
        }
    }
    Ok(())
}

const GLYPHS: [(char, [u8; 8]); 36] = [
    ('A', [0x38, 0x6C, 0xC6, 0xC6, 0xFE, 0xC6, 0xC6, 0x00]),
    ('B', [0xFC, 0x66, 0x66, 0x7C, 0x66, 0x66, 0xFC, 0x00]),
    ('C', [0x3C, 0x66, 0xC0, 0xC0, 0xC0, 0x66, 0x3C, 0x00]),
    ('D', [0xF8, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0xF8, 0x00]),
    ('E', [0xFE, 0x62, 0x68, 0x78, 0x68, 0x62, 0xFE, 0x00]),
    ('F', [0xFE, 0x62, 0x68, 0x78, 0x68, 0x60, 0xF0, 0x00]),
    ('G', [0x3C, 0x66, 0xC0, 0xCE, 0xC6, 0x66, 0x3A, 0x00]),
    ('H', [0xC6, 0xC6, 0xC6, 0xFE, 0xC6, 0xC6, 0xC6, 0x00]),
    ('I', [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00]),
    ('J', [0x1E, 0x0C, 0x0C, 0x0C, 0xCC, 0xCC, 0x78, 0x00]),
    ('K', [0xE6, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0xE6, 0x00]),
    ('L', [0xF0, 0x60, 0x60, 0x60, 0x62, 0x66, 0xFE, 0x00]),
    ('M', [0xC6, 0xEE, 0xFE, 0xFE, 0xD6, 0xC6, 0xC6, 0x00]),
    ('N', [0xC6, 0xE6, 0xF6, 0xDE, 0xCE, 0xC6, 0xC6, 0x00]),
    ('O', [0x38, 0x6C, 0xC6, 0xC6, 0xC6, 0x6C, 0x38, 0x00]),
    ('P', [0xFC, 0x66, 0x66, 0x7C, 0x60, 0x60, 0xF0, 0x00]),
    ('Q', [0x38, 0x6C, 0xC6, 0xC6, 0xCE, 0x6C, 0x3E, 0x00]),
    ('R', [0xFC, 0x66, 0x66, 0x7C, 0x6C, 0x66, 0xE6, 0x00]),
    ('S', [0x3C, 0x66, 0x70, 0x38, 0x0E, 0x66, 0x3C, 0x00]),
    ('T', [0x7E, 0x5A, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00]),
    ('U', [0xC6, 0xC6, 0xC6, 0xC6, 0xC6, 0xC6, 0x7C, 0x00]),
    ('V', [0xC6, 0xC6, 0xC6, 0xC6, 0xC6, 0x6C, 0x38, 0x00]),
    ('W', [0xC6, 0xC6, 0xC6, 0xD6, 0xFE, 0xEE, 0xC6, 0x00]),
    ('X', [0xC6, 0xC6, 0x6C, 0x38, 0x6C, 0xC6, 0xC6, 0x00]),
    ('Y', [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x3C, 0x00]),
    ('Z', [0xFE, 0xC6, 0x8C, 0x18, 0x32, 0x66, 0xFE, 0x00]),
    ('0', [0x3C, 0x66, 0x6E, 0x7E, 0x76, 0x66, 0x3C, 0x00]),
    ('1', [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00]),
    ('2', [0x3C, 0x66, 0x06, 0x1C, 0x30, 0x66, 0x7E, 0x00]),
    ('3', [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00]),
    ('4', [0x1C, 0x3C, 0x6C, 0xCC, 0xFE, 0x0C, 0x1E, 0x00]),
    ('5', [0x7E, 0x62, 0x60, 0x7C, 0x06, 0x66, 0x3C, 0x00]),
    ('6', [0x1C, 0x30, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00]),
    ('7', [0x7E, 0x66, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x00]),
    ('8', [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00]),
    ('9', [0x3C, 0x66, 0x66, 0x3E, 0x06, 0x0C, 0x38, 0x00]),
];

fn glyph_for(title: &str) -> Option<[u8; 8]> {
    let first = title.chars().find(|c| c.is_ascii_alphanumeric())?.to_ascii_uppercase();
    GLYPHS.iter().find(|(c, _)| *c == first).map(|(_, bits)| *bits)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [u8; 3] {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = (h % 360.0) / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    [
        (((r + m) * 255.0).round()).clamp(0.0, 255.0) as u8,
        (((g + m) * 255.0).round()).clamp(0.0, 255.0) as u8,
        (((b + m) * 255.0).round()).clamp(0.0, 255.0) as u8,
    ]
}

pub fn letter_tile(title: &str, seed: &str) -> RgbaImage {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    seed.hash(&mut hasher);
    let hue = (hasher.finish() % 360) as f32;
    let [r, g, b] = hsl_to_rgb(hue, 0.55, 0.45);

    let mut img = RgbaImage::from_pixel(ICON_SIZE, ICON_SIZE, Rgba([r, g, b, 255]));

    let Some(glyph) = glyph_for(title) else { return img };

    let scale = ICON_SIZE / 16;
    let origin = (ICON_SIZE - scale * 8) / 2;
    for (row, bits) in glyph.iter().enumerate() {
        for col in 0..8u32 {
            if bits & (0x80 >> col) == 0 {
                continue;
            }
            for dy in 0..scale {
                for dx in 0..scale {
                    let x = origin + col * scale + dx;
                    let y = origin + row as u32 * scale + dy;
                    if x < ICON_SIZE && y < ICON_SIZE {
                        img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                    }
                }
            }
        }
    }
    img
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removing_an_icon_takes_the_windows_ico_with_it() {
        let dir = tempfile::tempdir().unwrap();
        temp_env::with_var(paths::DATA_DIR_VAR, Some(dir.path()), || {
            let icons = paths::icon_dir().unwrap();
            let png = icons.join("probe-app.png");
            let ico = icons.join("probe-app.ico");
            std::fs::write(&png, "x").unwrap();
            std::fs::write(&ico, "x").unwrap();

            remove_icon("probe-app").unwrap();

            assert!(!png.exists(), "the PNG should be gone");
            assert!(!ico.exists(), "the .ico should be gone too");

            remove_icon("probe-app").unwrap();
        });
    }

    #[test]
    fn tile_is_the_expected_size_and_opaque() {
        let tile = letter_tile("YouTube", "youtube-com");
        assert_eq!(tile.dimensions(), (ICON_SIZE, ICON_SIZE));
        assert!(tile.pixels().all(|p| p.0[3] == 255));
    }

    #[test]
    fn tile_colour_is_stable_per_seed_and_varies_across_seeds() {
        assert_eq!(
            letter_tile("A", "youtube-com").get_pixel(0, 0),
            letter_tile("Different Title", "youtube-com").get_pixel(0, 0)
        );
        assert_ne!(
            letter_tile("A", "youtube-com").get_pixel(0, 0),
            letter_tile("A", "claude-ai").get_pixel(0, 0)
        );
    }

    #[test]
    fn tile_draws_the_initial() {
        let with_letter = letter_tile("Y", "seed");
        let white = with_letter.pixels().filter(|p| p.0 == [255, 255, 255, 255]).count();
        assert!(white > 0, "expected the initial to be drawn");

        let no_letter = letter_tile("!!!", "seed");
        assert_eq!(no_letter.pixels().filter(|p| p.0 == [255, 255, 255, 255]).count(), 0);
    }

    #[test]
    fn non_ascii_titles_do_not_panic() {
        let _ = letter_tile("日本語", "seed");
        let _ = letter_tile("", "seed");
        let _ = letter_tile("Ünicode", "seed");
    }

    #[test]
    fn a_custom_icon_is_rejected_when_it_is_not_an_image() {
        assert!(save_custom(b"", "x").is_err());
        assert!(save_custom(b"this is not a picture", "x").is_err());
    }

    #[test]
    fn normalize_centres_and_squares_non_square_input() {
        let wide = DynamicImage::ImageRgba8(RgbaImage::from_pixel(64, 32, Rgba([1, 2, 3, 255])));
        let out = normalize(wide);
        assert_eq!((out.width(), out.height()), (ICON_SIZE, ICON_SIZE));
    }
}
