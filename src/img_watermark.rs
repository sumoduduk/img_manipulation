use std::path::{Path, PathBuf};

use crate::{
    file_operation::{create_folder, get_filename},
    scale_image::begin_scale,
};
use image::imageops;

pub fn begin_watermarking(folder_path: &Path, images: Vec<PathBuf>) {
    let watermark_folder = folder_path.join("watermark");
    let watermark_img = image::open("bridal_watermark.png").expect("watermark image not found");

    let watermark_scale = begin_scale(&watermark_img, 512, 512, imageops::FilterType::Lanczos3);

    for img_path in images {
        create_folder(&watermark_folder);
        let image_main = image::open(&img_path).expect("watermark: failed open image");
        let mut img_scaled = begin_scale(&image_main, 512, 512, imageops::FilterType::CatmullRom);

        let _ = imageops::overlay(&mut img_scaled, &watermark_scale, 0, 0);

        let file_name = get_filename(&img_path);

        match file_name {
            Some(name) => {
                let file_output = watermark_folder.join(name);

                println!("file output : {:?}", &file_output);

                let _ = img_scaled.save(file_output);
            }
            None => {
                println!("failed to get filename");
            }
        }
    }
}
