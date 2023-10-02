use std::fs;

pub fn read_folder(path: &str) {
    let file_names = fs::read_dir(path).expect("error: can't open folder");

    for file_name in file_names {
        let file_name = file_name.expect("can't open file");
        let path_file = file_name.path();

        if path_file.is_file() {
            let extension = path_file.extension().and_then(std::ffi::OsStr::to_str);

            match extension {
                Some("jpg") | Some("jpeg") | Some("png") | Some("webp") => {
                    println!("file path : {:?}", path_file);
                }
                _ => (),
            }
        }
    }
}
