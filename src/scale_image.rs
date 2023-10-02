use image::imageops::{resize, FilterType};

pub fn begin_scale(
    watermark_path: &str,
    width: u32,
    height: u32,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let img = image::open(watermark_path).expect("watermark image not found");

    let scaled_img = resize(&img, width, height, FilterType::Lanczos3);
    scaled_img
}
