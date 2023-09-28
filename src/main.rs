use image::{GenericImageView, Rgba};
use imageproc::drawing;
use rusttype::{Font, Scale};

fn main() {
    let mut img = image::open("example.png").expect("no image found");
    let (width, height) = img.dimensions();

    let color = Rgba([255, 255, 255, 220]);

    let scale = Scale {
        x: (width / 8) as f32,
        y: (height / 8) as f32,
    };

    let font = Vec::from(include_bytes!("../CroissantOne.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    drawing::draw_text_mut(
        &mut img,
        color,
        (width / 4).try_into().unwrap(),
        (height / 2).try_into().unwrap(),
        scale,
        &font,
        "Hello World",
    );

    img.save("img_ouput.png").unwrap()
}
