use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::{Path, PathBuf};

use crate::file_operation::{create_folder, get_stemame};

pub fn webp_convert(folder_path: &Path, images: Vec<PathBuf>) {
    let webp_folder = folder_path.join("webp");

    images.par_iter().for_each(|img_path| {
        create_folder(&webp_folder);

        let img_main = image::open(&img_path).expect("webp_convert: cant open image");

        let file_name = get_stemame(&img_path);

        match file_name {
            Some(name_raw) => {
                let webp_name = format!("{name_raw}.webp");
                let file_out = webp_folder.join(&webp_name);

                let result = img_main.save_with_format(&file_out, image::ImageFormat::WebP);

                match result {
                    Ok(_) => println!("result image path : {:?}", &file_out),
                    Err(err) => println!("image error : {:?}", err),
                }
            }
            None => {
                println!("failed to get stemname");
            }
        }
    });
}
