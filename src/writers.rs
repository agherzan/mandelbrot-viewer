use image::ColorType;
use image::codecs::png::PngEncoder;
use std::fs::File;

// Writes a png encoded file of a pixel map array
pub fn export_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> image::ImageResult<()> {
    let file = File::create(filename)?;
    let encoder = PngEncoder::new(file);
    encoder.encode(&pixels,
                   bounds.0 as u32,
                   bounds.1 as u32,
                   ColorType::L8)?;
    Ok(())
}
