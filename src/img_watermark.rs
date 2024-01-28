use std::path::{Path, PathBuf};

use crate::{
    file_operation::{create_folder, get_filename},
    scale_image::begin_scale,
};
use image::{imageops, GenericImageView};

pub fn begin_watermarking(folder_path: &Path, images: &[PathBuf], watermark_path: &Path) {
    let watermark_folder = folder_path.join("watermark");
    let watermark_img = image::open(watermark_path).expect("ERROR: watermark image not found");

    for img_path in images {
        create_folder(&watermark_folder);
        let mut image_main = image::open(&img_path).expect("watermark: failed open image");
        let (w, h) = image_main.dimensions();

        let watermark_scale = begin_scale(&watermark_img, w, h, imageops::FilterType::Lanczos3);

        // let mut img_scaled = begin_scale(&image_main, 512, 512, imageops::FilterType::CatmullRom);

        let _ = imageops::overlay(&mut image_main, &watermark_scale, 0, 0);

        let file_name = get_filename(&img_path);

        match file_name {
            Some(name) => {
                let file_output = watermark_folder.join(name);

                println!("file output : {:?}", &file_output);
                let _ = image_main.save(file_output);

                println!("file saved success");
            }
            None => {
                println!("failed to get filename");
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::path::Path;
    use std::time::Instant;

    use crate::file_operation::read_folder;

    #[test]
    fn test_watermark() {
        let watermark_path = Path::new("bridal_watermark.png");
        let folder_path = Path::new("./test_img");
        let start = Instant::now();
        let images = read_folder(&folder_path).expect("ERROR: should read folder tes");
        begin_watermarking(folder_path, &images, watermark_path);
        let duration = start.elapsed();

        println!("Time duration : {:#?}", duration);
    }
}
