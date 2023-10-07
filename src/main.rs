mod convert;
mod file_operation;
mod img_watermark;
mod overlay;
mod scale_image;
mod thumbnailing;

use std::{env, path::Path};

use file_operation::read_folder;

use crate::{
    convert::webp_convert, img_watermark::begin_watermarking, thumbnailing::create_thumbnail,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let folder_path = args
        .get(1)
        .expect("Please provide folder path as first argument");

    println!("folder path : {}", folder_path);

    let process_picked = args
        .get(2)
        .expect("Please provide process name as second argument");

    println!("process picked : {}", process_picked);

    let folder_path = Path::new(&folder_path);
    let images = read_folder(&folder_path).expect("No image file in folder");

    match process_picked.to_lowercase().trim() {
        "watermark" => begin_watermarking(folder_path, images),
        "thumbnail" => create_thumbnail(folder_path, images),
        "webp" => webp_convert(folder_path, images),
        _ => {
            println!("please type watermark or thumbail only");
        }
    }

    println!("DONE");
}
