use std::io::BufWriter;

use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;
use image::{
    save_buffer, DynamicImage, ExtendedColorType, GenericImageView, ImageEncoder, ImageError,
};

use fast_image_resize::images::Image;
use fast_image_resize::{IntoImageView, PixelType, Resizer};

pub fn create_thumbnail(original_image_path: &str) -> Result<String, ImageError> {
    // Read source image from file
    let src_image: DynamicImage = ImageReader::open(original_image_path)
        .unwrap()
        .decode()
        .unwrap();

    // Create container for data of destination image
    let dst_width = 1024;
    let dst_height = 768;
    let mut dst_image = Image::new(dst_width, dst_height, PixelType::U8);

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = Resizer::new();
    resizer.resize(&src_image, &mut dst_image, None).unwrap();

    // Write destination image as JPEG-file
    let mut result_buf = BufWriter::new(Vec::new());
    JpegEncoder::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            dst_width,
            dst_height,
            ExtendedColorType::Rgba8,
        )
        .unwrap();

    let output_path = format!("{}_thumbnail", original_image_path);
    match save_buffer(
        &output_path,
        result_buf.buffer(),
        dst_width,
        dst_height,
        ExtendedColorType::Rgba8,
    ) {
        Ok(_) => Ok(output_path),
        Err(e) => Err(e),
    }
}
