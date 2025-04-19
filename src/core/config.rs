use serde::Deserialize;
use std::io::ErrorKind;
use std::{fs, io};
use toml;

fn default_note_dir() -> String {
    String::from(dirs::home_dir().unwrap().join("notes").to_str().unwrap())
}

fn default_editor() -> String {
    "nvim".to_string()
}

#[derive(Deserialize)]
pub struct Core {
    #[serde(default = "default_note_dir")]
    pub note_dir: String,
    #[serde(default = "default_editor")]
    pub editor: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub core: Core,
}

pub fn load_config() -> Result<Config, io::Error> {
    let home_path = String::from(dirs::home_dir().unwrap().to_str().unwrap());
    let filename = home_path + "/.config/grom/grom.toml";

    let contents = fs::read_to_string(filename.clone())?;

    let data: Config = match toml::from_str(&contents) {
        Ok(d) => {
            let mut config: Config = d;
            if config.core.note_dir.starts_with("~") {
                config.core.note_dir = config
                    .core
                    .note_dir
                    .replace("~", dirs::home_dir().unwrap().to_str().unwrap())
            }
            config
        }
        Err(e) => return Err(io::Error::new(ErrorKind::InvalidData, e.to_string())),
    };
    Ok(data)
}
