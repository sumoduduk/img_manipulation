use image::{
    imageops::{resize, FilterType},
    DynamicImage,
};

pub fn begin_scale(
    watermark_img: &DynamicImage,
    width: u32,
    height: u32,
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let scaled_img = resize(watermark_img, width, height, FilterType::Lanczos3);
    scaled_img
}
