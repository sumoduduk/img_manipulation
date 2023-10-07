pub fn webp_convert() {
    let img_main = image::open("bride.png").expect("webp_convert: cant open image webp");

    let _ = img_main
        .save("bride.webp")
        .expect("webp_convert: cant save image");

    println!("Done converting");
}
