mod file_operation;
mod imgproc;
mod overlay;
mod scale_image;
mod watermark;

use std::env;

use file_operation::read_folder;
use image::{imageops, GenericImageView};

use scale_image::begin_scale;

fn main() {
    let mut args = env::args();

    let folder_path = args.nth(1).expect("no folder path found");
    println!("folde path : {}", &folder_path);
    read_folder(&folder_path);

    // let mut image_main = image::open(&img_path).expect("error open image");
    //
    // let (width, height) = image_main.dimensions();
    //
    // let watermark_path = "watermark.png";
    //
    // let watermark_vec = begin_scale(watermark_path, width, height);
    //
    // let _ = imageops::overlay(&mut image_main, &watermark_vec, 0, 0);
    //
    // let _ = image_main.save("bride_output.png");
    //
    // println!("DONE");
}
