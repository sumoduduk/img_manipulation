mod convert;
mod file_operation;
mod img_watermark;
mod scale_image;
mod thumbnailing;

use std::{env, path::Path};

use file_operation::read_folder;

use crate::{
    convert::webp_convert, img_watermark::begin_watermarking, thumbnailing::create_thumbnail,
};

fn main() {
    let mut args = env::args();

    args.next().unwrap();

    let folder_path = args
        .next()
        .expect("Please provide folder path as first argument");

    println!("folder path : {}", folder_path);

    let process_picked = args
        .next()
        .expect("Please provide process name as second argument");

    println!("process picked : {}", process_picked);

    let folder_path = Path::new(&folder_path);
    let images = read_folder(&folder_path).expect("No image file in folder");

    match process_picked.to_lowercase().trim() {
        "watermark" => {
            let watermark_arg = args
                .next()
                .expect("watermark: you must specify image watermark path in 3rd args");

            let watermark_path = Path::new(&watermark_arg);
            begin_watermarking(folder_path, &images, watermark_path);
        }
        "thumbnail" => create_thumbnail(folder_path, images),
        "webp" => webp_convert(folder_path, images),
        _ => {
            println!("please type watermark or thumbail only");
        }
    }

    println!("DONE");
}
