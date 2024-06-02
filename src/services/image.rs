use std::io::BufWriter;

use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;
use image::{save_buffer, ExtendedColorType, ImageEncoder, ImageError};

use fast_image_resize::images::Image;
use fast_image_resize::{IntoImageView, Resizer};

pub fn create_thumbnail(original_image_path: &str) -> Result<String, ImageError> {
    // Read source image from file
    let src_image = ImageReader::open(original_image_path)
        .expect("failed to read buffer")
        .decode()
        .expect("image decode error");
    // .as_rgb8()
    //.expect("failed to transform image to rbg8");

    let pixel_type = src_image.pixel_type().expect("could not get pixel type");

    // Create container for data of destination image
    let dst_width = 150;
    let dst_height = 125;
    let mut dst_image = Image::new(dst_width, dst_height, pixel_type);

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = Resizer::new();
    resizer
        .resize(&src_image, &mut dst_image, None)
        .expect("failed to resize image");

    // Write destination image as JPEG-file
    let mut result_buf = BufWriter::new(Vec::new());
    JpegEncoder::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            dst_width,
            dst_height,
            ExtendedColorType::Rgb8,
        )
        .expect("image encoded error");

    // let output_path = format!("{}_thumbnail", original_image_path);
    let output_path = format!("static/thumbnail2.jpg");
    println!("REACHED SAVE!!!");
    match save_buffer(
        &output_path,
        result_buf.buffer(),
        dst_width,
        dst_height,
        ExtendedColorType::Rgb8,
    ) {
        Ok(_) => Ok(output_path),
        Err(e) => {
            println!("save buffer error: {}", e);
            Err(e)
        }
    }
}
