use photon_rs::multiple::watermark;
use photon_rs::native::{open_image, save_image};
use photon_rs::PhotonImage;

pub fn insert_watermark(img_file: &str, watermark_vec: Vec<u8>, width: u32, height: u32) {
    let mut img = open_image(img_file).expect("image not exist");
    let water = PhotonImage::new(watermark_vec, width, height);

    watermark(&mut img, &water, 0, 0);

    let _ = save_image(img, "bride_output.png");
}
