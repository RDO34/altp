use std::fs;
use std::path::Path;
use std::process::exit;

use clap::Parser;
use dialoguer::Select;
use toml::{Table, Value};
use whoami;

mod themes;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The name of the theme to use
    theme: Option<String>,

    /// Print a list of available themes
    #[arg(short, long)]
    list: bool,

    /// Print the current theme name
    #[arg(short = 'C', long)]
    current: bool,

    /// The directory to search for the alacritty config file
    ///
    /// If not provided, the default directory will be used
    ///
    /// Default: None
    ///
    /// Example: "/home/user/.config/alacritty"
    ///
    /// Example: "C:/Users/user/AppData/Roaming/alacritty"
    #[arg(short, long)]
    dir: Option<String>,

    /// Whether to create a new config file if one does not already exist
    ///
    /// Default: false
    #[arg(short, long)]
    create: bool,
}

fn main() {
    let args = Args::parse();

    if args.list {
        themes::print_all_themes();
        exit(0);
    }

    if args.current {
        let config_path = resolve_altp_config_path(&args.dir);
        let config_str = match fs::read_to_string(&config_path) {
            Ok(config_str) => config_str,
            Err(_) => {
                println!("Config file not found.");
                exit(1);
            }
        };

        let config = toml::from_str::<Table>(&config_str).expect("Failed to parse config file");

        let theme_name = config
            .get("theme")
            .and_then(|theme| theme.as_str())
            .unwrap_or("default");

        println!("{}", theme_name);
        exit(0);
    }

    let theme_list = themes::get_all_themes();
    let selection: usize = match args.theme {
        Some(theme_name) => theme_list
            .iter()
            .position(|theme| theme.name == theme_name)
            .unwrap_or_else(|| {
                eprintln!("Theme not found");
                exit(1);
            }),
        None => Select::new()
            .with_prompt("Select a theme")
            .items(
                &theme_list
                    .iter()
                    .map(|theme| theme.name.as_str())
                    .collect::<Vec<&str>>(),
            )
            .default(0)
            .interact()
            .unwrap(),
    };

    let theme_path = Path::new(&theme_list[selection].path);
    let theme = get_or_create_config_table(theme_path.to_str().unwrap(), false);

    let alacritty_config_path = resolve_alacritty_config_path(&args.dir);
    let alacritty_config = get_or_create_config_table(&alacritty_config_path, args.create);

    let mut new_alacritty_config = alacritty_config.clone();
    new_alacritty_config.insert("colors".to_string(), theme.get("colors").unwrap().clone());

    write_config_file(&alacritty_config_path, &new_alacritty_config);

    let altp_config_path = resolve_altp_config_path(&args.dir);
    let mut altp_config = get_or_create_config_table(&altp_config_path, true);

    let theme_name_value = Value::String(theme_list[selection].name.clone());
    altp_config.insert("theme".to_string(), theme_name_value);

    let theme_author_value = Value::String(
        theme
            .get("author")
            .and_then(|author| author.as_str())
            .unwrap_or("unknown")
            .to_string(),
    );
    altp_config.insert("author".to_string(), theme_author_value);

    write_config_file(&altp_config_path, &altp_config);
}

fn resolve_alacritty_config_dir(dir: &Option<String>) -> String {
    match dir {
        Some(dir) => format!("{}", dir),
        None => match cfg!(windows) {
            true => format!("C:/Users/{}/AppData/Roaming/alacritty", whoami::username()),
            false => format!("/home/{}/.config/alacritty", whoami::username()),
        },
    }
}

fn resolve_alacritty_config_path(dir: &Option<String>) -> String {
    format!("{}/alacritty.toml", resolve_alacritty_config_dir(dir))
}

fn resolve_altp_config_path(dir: &Option<String>) -> String {
    format!("{}/altp.toml", resolve_alacritty_config_dir(dir))
}

fn get_or_create_config_file(path: &str, should_create: bool) -> String {
    match fs::read_to_string(path) {
        Ok(config_str) => config_str,
        Err(_) => {
            if !should_create {
                println!("Config file not found. Use --create to create a new config file.");
                exit(1);
            }

            let default_config = "";
            create_config_file(path, default_config);
            default_config.to_string()
        }
    }
}

fn get_or_create_config_table(path: &str, should_create: bool) -> Table {
    let file_str = get_or_create_config_file(path, should_create);
    toml::from_str::<Table>(&file_str).expect("Failed to parse config file")
}

fn write_config_file(path: &str, config: &Table) {
    let config_str = toml::to_string(config).unwrap();
    fs::write(path, config_str).expect("Failed to write config file");
}

fn create_config_file(path: &str, default_config: &str) {
    let config_dir = path
        .split("/")
        .take_while(|s| !s.ends_with(".toml"))
        .collect::<Vec<&str>>()
        .join("/");

    fs::create_dir_all(config_dir).expect("Failed to create config directory");

    fs::write(&path, default_config).expect("Failed to create alacritty config file");

    println!("Config file created at {}", path);
}
