use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::io::Error;

// Writes a png encoded file of a pixel map array
pub fn export_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), Error> {
    let file = File::create(filename)?;
    let encoder = PNGEncoder::new(file);
    encoder.encode(&pixels,
                   bounds.0 as u32,
                   bounds.1 as u32,
                   ColorType::Gray(8))?;
    Ok(())
}
