use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::{Encoder, ColorType};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("Image dimensions must be positive")]
    InvalidDimensions,
    #[error("RGB data length {0} doesn't match dimensions {1}x{2}")]
    InvalidDataLength(usize, u32, u32),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("PNG encoding error: {0}")]
    PngError(#[from] png::EncodingError),
}

/// Writes RGB data to a PNG file.
///
/// # Arguments
/// * `path` - Path where the PNG file will be written
/// * `width` - Width of the image in pixels
/// * `height` - Height of the image in pixels
/// * `rgb_data` - Raw RGB data as bytes, must be width * height * 3 bytes long
///
/// # Returns
/// * `Result<(), ImageError>` - Ok(()) if successful, or an error if something went wrong
pub fn write_rgb_to_png(
    path: impl AsRef<Path>,
    width: u32,
    height: u32,
    rgb_data: &[u8],
) -> Result<(), ImageError> {
    // Validate dimensions
    if width == 0 || height == 0 {
        return Err(ImageError::InvalidDimensions);
    }

    // Validate data length
    let expected_len = (width * height * 3) as usize;
    if rgb_data.len() != expected_len {
        return Err(ImageError::InvalidDataLength(rgb_data.len(), width, height));
    }

    // Create file and writer
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    // Create PNG encoder
    let mut encoder = Encoder::new(writer, width, height);
    encoder.set_color(ColorType::Rgb);

    // Write the image
    let mut writer = encoder.write_header()?;
    writer.write_image_data(rgb_data)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_write_valid_image() {
        let temp_file = NamedTempFile::new().unwrap();
        let rgb_data = vec![255, 0, 0, 0, 255, 0]; // 2x1 image: red and green pixels

        let result = write_rgb_to_png(temp_file.path(), 2, 1, &rgb_data);
        assert!(result.is_ok());

        // Verify file exists and has content
        let metadata = fs::metadata(temp_file.path()).unwrap();
        assert!(metadata.len() > 0);
    }

    #[test]
    fn test_invalid_dimensions() {
        let temp_file = NamedTempFile::new().unwrap();
        let rgb_data = vec![255, 0, 0];

        let result = write_rgb_to_png(temp_file.path(), 0, 1, &rgb_data);
        assert!(matches!(result, Err(ImageError::InvalidDimensions)));

        let result = write_rgb_to_png(temp_file.path(), 1, 0, &rgb_data);
        assert!(matches!(result, Err(ImageError::InvalidDimensions)));
    }

    #[test]
    fn test_invalid_data_length() {
        let temp_file = NamedTempFile::new().unwrap();
        let rgb_data = vec![255, 0, 0]; // Only one pixel worth of data

        let result = write_rgb_to_png(temp_file.path(), 2, 1, &rgb_data);
        assert!(matches!(result, Err(ImageError::InvalidDataLength(3, 2, 1))));
    }
}
