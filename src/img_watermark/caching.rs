use std::sync::{Arc, RwLock};

use image::{imageops, DynamicImage};

use crate::scale_image::begin_scale;

use super::BufferImage;
use super::CacheTable;

pub fn process_cache(
    cache_state: Arc<RwLock<CacheTable>>,
    width: u32,
    height: u32,
    watermark_img: &DynamicImage,
) -> Arc<BufferImage> {
    let total = width * height;

    if let Some(scaled_watermark) = cache_state.read().unwrap().get(&total) {
        return Arc::clone(scaled_watermark);
    }

    let watermark_scale = begin_scale(&watermark_img, width, height, imageops::FilterType::Nearest);
    let rc_watermark = Arc::new(watermark_scale);

    let mut hash_map = cache_state.write().unwrap();
    hash_map.insert(total, Arc::clone(&rc_watermark));

    rc_watermark
}
