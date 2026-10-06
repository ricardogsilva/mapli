use crate::error::{MapliError,Result};

pub fn encode_png(img: &RgbaImage) -> Result<Vec<u8>> {
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::Png)
        .map_err(|e| MapliError::PngEncodingFailed(e.to_string()))?;
    Ok(buf.into_inner())
}