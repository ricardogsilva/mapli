use image::RgbaImage;

use crate::error::{MapliError, Result};
use crate::types::{OutputFormat, Rendered};

pub fn encode_png(img: &RgbaImage) -> Result<Vec<u8>> {
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::Png)
        .map_err(|e| MapliError::PngEncodingFailed(e.to_string()))?;
    Ok(buf.into_inner())
}

/// Convert a freshly rendered image into the requested output format.
pub(crate) fn encode(img: RgbaImage, format: OutputFormat) -> Result<Rendered> {
    match format {
        OutputFormat::Raw => Ok(Rendered::Raw(img)),
        OutputFormat::Png => encode_png(&img).map(Rendered::Png),
    }
}
