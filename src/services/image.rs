use std::io::BufWriter;

use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ExtendedColorType, GenericImageView, ImageEncoder};

use fast_image_resize::images::Image;
use fast_image_resize::{IntoImageView, PixelType, Resizer};

pub fn create_thumbnail(original_image_path: &str) {
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

    // Write destination image as PNG-file
    let mut result_buf = BufWriter::new(Vec::new());
    PngEncoder::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            dst_width,
            dst_height,
            ExtendedColorType::Rgba8,
        )
        .unwrap();
}
