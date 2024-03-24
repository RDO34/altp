use std::env;
use std::fs;
use std::path::Path;

use toml::Table;

pub struct Theme {
    pub name: String,
    pub path: String,
}

pub fn print_all_themes() {
    let themes = get_all_themes();
    for theme in themes {
        println!("{}", theme.name);
    }
}

pub fn get_all_themes() -> Vec<Theme> {
    let mut themes = Vec::new();

    let static_themes_dir = get_static_themes_dir();

    let paths = fs::read_dir(static_themes_dir)
        .expect("Failed to read directory")
        .map(|res| res.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .expect("Failed to collect directory entries");

    for _path in paths {
        let path = _path.to_str().unwrap();

        let theme_str = fs::read_to_string(Path::new(path)).unwrap();
        let theme = toml::from_str::<Table>(&theme_str).unwrap();

        let name_from_path = get_name_from_path(path);

        let name = match theme.get("name") {
            Some(name) => name.as_str().unwrap(),
            None => name_from_path.as_str(),
        };

        themes.push(Theme {
            name: name.to_string(),
            path: path.to_string(),
        });
    }

    themes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    themes
}

fn get_static_themes_dir() -> String {
    let cwd = env::current_dir().expect("Failed to get current working directory");

    let mut static_dir = cwd.join("static/themes");

    if !static_dir.exists() {
        let config_dir = resolve_altp_config_dir();
        static_dir = Path::new(&config_dir).join("themes");
    }

    static_dir.to_str().unwrap().to_string()
}

fn resolve_altp_config_dir() -> String {
    match cfg!(windows) {
        true => format!("C:/Users/{}/AppData/Roaming/altp", whoami::username()),
        false => format!("/home/{}/.config/altp", whoami::username()),
    }
}

fn get_name_from_path(path: &str) -> String {
    let sep = if cfg!(windows) { "\\" } else { "/" };
    let name = path.split(sep).last().unwrap();
    let name = name.split(".").next().unwrap();
    name.to_string()
}
