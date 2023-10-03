mod file_operation;
mod overlay;
mod scale_image;

use std::{env, path::Path};

use file_operation::read_folder;
use image::{imageops, GenericImageView};

use scale_image::begin_scale;

use file_operation::{create_folder, get_filename};

fn main() {
    let mut dimensions_main: (u32, u32) = (0, 0);
    let mut watermark_main = None;

    let mut args = env::args();

    let folder_path = args.nth(1).expect("no folder path found");
    println!("folde path : {}", &folder_path);

    let folder_path = Path::new(&folder_path);
    let out_name = folder_path.join("out");
    let images = read_folder(&folder_path).expect("No image file in folder");

    let watermark_img = image::open("bridal_watermark.png").expect("watermark image not found");

    for img_path in images {
        create_folder(&out_name);
        let mut image_main = image::open(&img_path).expect("error open image");

        let dimensions = image_main.dimensions();
        println!("w: {}, h: {}", dimensions.0, dimensions.1);

        if dimensions_main != dimensions {
            dimensions_main = dimensions;
            let watermark_scale = begin_scale(&watermark_img, dimensions.0, dimensions.1);
            watermark_main = Some(watermark_scale);
        }

        let watermark_main = watermark_main.clone();

        match watermark_main {
            Some(watermark_vec) => {
                let _ = imageops::overlay(&mut image_main, &watermark_vec, 0, 0);

                let file_name = get_filename(&img_path);

                match file_name {
                    Some(name) => {
                        let file_output = out_name.join(name);

                        println!("file output : {:?}", &file_output);

                        let _ = image_main.save(file_output);
                    }
                    None => {
                        println!("failed to get filename");
                    }
                }
            }
            None => {
                println!("None existent option");
            }
        }
    }

    println!("DONE");
}
