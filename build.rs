use std::fs;
use std::path::Path;

use whoami;

fn main() {
    let src_dir = "static/themes";
    let config_dir = resolve_altp_config_dir();

    let dst_dir = Path::new(&config_dir).join("themes");

    fs::create_dir_all(&dst_dir).unwrap();

    let files = fs::read_dir(src_dir).unwrap();
    for file in files {
        let file = file.unwrap();
        let src = file.path();
        let dst = dst_dir.join(file.file_name());

        fs::copy(&src, &dst).unwrap();
    }
}

fn resolve_altp_config_dir() -> String {
    match cfg!(windows) {
        true => format!("C:/Users/{}/AppData/Roaming/altp", whoami::username()),
        false => format!("/home/{}/.config/altp", whoami::username()),
    }
}
