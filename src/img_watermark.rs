mod caching;

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::Instant;

use crate::file_operation::{create_folder, get_filename};
use image::{imageops, GenericImageView, ImageBuffer};

use caching::process_cache;
use rayon::prelude::*;
use std::collections::HashMap;

type BufferImage = ImageBuffer<image::Rgba<u8>, Vec<u8>>;
type CacheTable = HashMap<u32, Arc<BufferImage>>;

pub fn begin_watermarking(folder_path: &Path, images: &[PathBuf], watermark_path: &Path) {
    let watermark_folder = folder_path.join("watermark");
    let watermark_img = image::open(watermark_path).expect("ERROR: watermark image not found");

    let cache_state: Arc<RwLock<CacheTable>> =
        Arc::new(RwLock::new(HashMap::with_capacity(images.len())));

    images.par_iter().for_each(move |img_path| {
        create_folder(&watermark_folder);
        let cache_state = Arc::clone(&cache_state);

        let start_main = Instant::now();
        let mut image_main = image::open(img_path).expect("watermark: failed open image");
        let duration_main = start_main.elapsed();
        println!(
            "MAIN: Time duration when opening main image : {:#?}",
            duration_main
        );

        let (w, h) = image_main.dimensions();

        let start_cache = Instant::now();
        let watermark_scale = process_cache(cache_state, w, h, &watermark_img);
        let water_ref = watermark_scale.as_ref();

        let duration_cache = start_cache.elapsed();
        println!(
            "CACHING: Time on dimension {} duration when caching image : {:#?}",
            w * h,
            duration_cache
        );
        // let mut img_scaled = begin_scale(&image_main, 512, 512, imageops::FilterType::CatmullRom);

        let start_watermark = Instant::now();
        let _ = imageops::overlay(&mut image_main, water_ref, 0, 0);
        let duration_watermark = start_watermark.elapsed();
        println!(
            "WATERMARKING: Time duration when watermarking image : {:#?}",
            duration_watermark
        );

        let file_name = get_filename(img_path);

        match file_name {
            Some(name) => {
                let file_output = watermark_folder.join(name);

                // println!("file output : {:?}", &file_output);
                let start_saving = Instant::now();
                let _ = image_main.save(file_output);
                let duration_saving = start_saving.elapsed();
                println!(
                    "SAVING: Time duration when saving image : {:#?}",
                    duration_saving
                );

                // println!("file saved success");
            }
            None => {
                println!("failed to get filename");
            }
        }
    });
}

#[cfg(test)]
mod test {

    use super::*;
    use std::path::Path;

    use crate::file_operation::read_folder;

    #[test]
    fn test_watermark() {
        let watermark_path = Path::new("bridal_watermark.png");
        let folder_path = Path::new("./test_img");
        let start = Instant::now();
        let images = read_folder(&folder_path).expect("ERROR: should read folder tes");
        begin_watermarking(folder_path, &images, watermark_path);
        let duration = start.elapsed();

        println!("TEST: Time duration : {:#?}", duration);
    }
}
