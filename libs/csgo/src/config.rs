use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::io::Write;

const DEFAULT_CONFIG: &str = r#"
# The directory containing your CS:GO server game logs, e.g. <server install dir>/server/logs
log_dir = ''

# How many seconds to wait before checking for new logs
delay = 2

# Optionally, specify whether Steam ID translation should occurs (set active = true)
# You will need to specify some combination of either:
# 1) a hash algorithm (MD5, SHA1, or SHA256)
# 2) a series of mappings between specific Steam IDs and your desired alternative names
# 3) both of the above (in which case the hash will only be used if there is no Steam ID mapping that applies)
#[steam_id_translation]
#active = false
#hash = 'SHA1'

#[[steam_id_translation.mappings]]
#steam_id = 'STEAM_1:1:00000001'
#name = 'Alice'

#[[steam_id_translation.mappings]]
#steam_id = 'STEAM_1:0:00000002'
#name = 'Bob'

"#;

const CONFIG_FILE_NAME: &str = "Config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub enum HashAlgo {
    MD5,
    SHA1,
    SHA256,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamIdTranslationMapping {
    pub steam_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamIdTranslation {
    pub active: bool,
    pub hash: Option<HashAlgo>,
    pub mappings: Option<Vec<SteamIdTranslationMapping>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub log_dir: std::path::PathBuf,
    pub delay: u64,
    pub steam_id_translation: Option<SteamIdTranslation>,
}

impl Config {
    pub fn default() -> Config {
        toml::from_str(DEFAULT_CONFIG).unwrap()
    }

    pub fn read_from_file(file_path: &std::path::Path) -> Option<Config> {
        if file_path.exists() {
            log::info!("Found file. Loading config...");

            // Read the file contents
            let contents =
                std::fs::read_to_string(file_path).expect("Failed to read data from file!");

            let config: Config = toml::from_str(&contents).unwrap();

            Some(config)
        } else {
            log::warn!(
                "Config file not found! Writing new default config to: {}",
                file_path.display()
            );
            // Write the default config to file
            Self::write_to_file(DEFAULT_CONFIG, file_path).unwrap();

            // Return the default config
            Some(Config::default())
        }
    }

    /// Write the specified config string to file. This will create the file and any necessary parent directories, if they do not already exist
    pub fn write_to_file(
        config_str: &str,
        file_path: &std::path::Path,
    ) -> Result<(), std::io::Error> {
        // Is there no config file?
        if !file_path.exists() {
            log::warn!(
                "No config file found... Creating one at: {}",
                file_path.display()
            );

            // Create any missing dirs
            let prefix = file_path.parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();
        }

        // Create a new config file
        let mut config_file = match std::fs::File::create(&file_path) {
            Err(why) => panic!("Failed to create file at {}: {}", file_path.display(), why),
            Ok(file) => file,
        };

        // Write the default config to the file
        config_file.write_all(config_str.as_bytes())
    }

    pub fn locate_config_file(path: &std::path::Path) -> Option<std::path::PathBuf> {
        // Determine where the config file should be found
        //let project_dirs = ProjectDirs::from("", "", "csgolp").unwrap();
        if let Some(project_dirs) = ProjectDirs::from("", "", "csgolp") {
            let config_dir = project_dirs.config_dir();
            let config_path = config_dir.join(path);
            log::info!("Config file location: {:?}", config_path);
            Some(config_path)
        } else {
            None
        }
    }

    pub fn from_file_or_default() -> Config {
        if let Some(config_file_path) =
            Self::locate_config_file(std::path::Path::new(CONFIG_FILE_NAME))
        {
            if let Some(config) = Self::read_from_file(&config_file_path) {
                config
            } else {
                Self::default()
            }
        } else {
            Self::default()
        }
    }
}
