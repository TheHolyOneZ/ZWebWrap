use anyhow::{Context, Result};
use std::path::Path;

pub const SIZES: [u32; 4] = [16, 32, 48, 256];

pub fn from_png(png: &[u8]) -> Result<Vec<u8>> {
    let source = image::load_from_memory(png).context("reading the app icon")?.to_rgba8();

    let mut dir = ico::IconDir::new(ico::ResourceType::Icon);
    for size in SIZES {
        let scaled =
            image::imageops::resize(&source, size, size, image::imageops::FilterType::Lanczos3);
        let image = ico::IconImage::from_rgba_data(size, size, scaled.into_raw());
        dir.add_entry(ico::IconDirEntry::encode(&image).context("encoding an icon size")?);
    }

    let mut out = std::io::Cursor::new(Vec::new());
    dir.write(&mut out).context("writing the .ico")?;
    Ok(out.into_inner())
}

pub fn convert_file(png_path: &Path) -> Result<std::path::PathBuf> {
    let png = std::fs::read(png_path).with_context(|| format!("reading {}", png_path.display()))?;
    let ico = from_png(&png)?;
    let out = png_path.with_extension("ico");
    std::fs::write(&out, ico).with_context(|| format!("writing {}", out.display()))?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_png() -> Vec<u8> {
        let mut buffer = std::io::Cursor::new(Vec::new());
        let image = image::RgbaImage::from_fn(64, 64, |x, y| {
            image::Rgba([(x * 4) as u8, (y * 4) as u8, 128, 255])
        });
        image::DynamicImage::ImageRgba8(image)
            .write_to(&mut buffer, image::ImageFormat::Png)
            .unwrap();
        buffer.into_inner()
    }

    #[test]
    fn every_size_windows_asks_for_is_present() {
        let ico = from_png(&sample_png()).unwrap();
        let dir = ico::IconDir::read(std::io::Cursor::new(ico)).unwrap();

        let mut widths: Vec<u32> = dir.entries().iter().map(|e| e.width()).collect();
        widths.sort_unstable();
        assert_eq!(widths, SIZES);

        for entry in dir.entries() {
            assert_eq!(entry.width(), entry.height());
        }
    }

    #[test]
    fn the_largest_size_survives_a_round_trip() {
        let ico = from_png(&sample_png()).unwrap();
        let dir = ico::IconDir::read(std::io::Cursor::new(ico)).unwrap();

        let biggest = dir.entries().iter().max_by_key(|e| e.width()).unwrap();
        let decoded = biggest.decode().unwrap();
        assert_eq!(decoded.width(), 256);
        assert_eq!(decoded.rgba_data().len(), 256 * 256 * 4);
    }

    #[test]
    fn a_file_that_is_not_an_image_is_an_error_not_a_panic() {
        assert!(from_png(b"this is not a PNG").is_err());
    }

    #[test]
    fn convert_file_writes_beside_the_source() {
        let dir = tempfile::tempdir().unwrap();
        let png = dir.path().join("youtube-com.png");
        std::fs::write(&png, sample_png()).unwrap();

        let ico = convert_file(&png).unwrap();
        assert_eq!(ico, dir.path().join("youtube-com.ico"));
        assert!(ico.is_file());
    }
}
