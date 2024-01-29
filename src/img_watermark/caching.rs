use std::{collections::HashMap, rc::Rc};

use image::{imageops, DynamicImage};

use crate::scale_image::begin_scale;

use super::BufferImage;

pub fn process_cache(
    cache_state: &mut HashMap<u32, Rc<BufferImage>>,
    width: u32,
    height: u32,
    watermark_img: &DynamicImage,
) -> Rc<BufferImage> {
    let total = width * height;
    println!("INFO : dimension key {total}");
    if let Some(scaled_watermark) = cache_state.get(&total) {
        Rc::clone(scaled_watermark)
    } else {
        let watermark_scale = begin_scale(
            &watermark_img,
            width,
            height,
            imageops::FilterType::Lanczos3,
        );
        let rc_watermark = Rc::new(watermark_scale);
        cache_state.insert(total, Rc::clone(&rc_watermark));
        Rc::clone(&rc_watermark)
    }
}
