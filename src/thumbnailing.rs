use std::path::{Path, PathBuf};

use crate::{
    file_operation::{create_folder, get_filename},
    scale_image::begin_scale,
};

pub fn create_thumbnail(folder_path: &Path, images: Vec<PathBuf>) {
    let thumbnail_folder = folder_path.join("thumbnail");

    for img_path in images {
        create_folder(&thumbnail_folder);

        let img_main = image::open(&img_path).expect("thumb: failed open image");
        let image_scaled = begin_scale(&img_main, 256, 256, image::imageops::FilterType::Nearest);

        let file_name = get_filename(&img_path);

        match file_name {
            Some(name) => {
                let file_output = thumbnail_folder.join(name);

                println!("file output : {:?}", &file_output);

                let _ = image_scaled.save(file_output);
            }
            None => {
                println!("failed to get filename");
            }
        }
    }
}
