use std::fs;
use std::path::Path;
use std::str::FromStr;
use log::{warn, LevelFilter};
use crate::errors::SettingsErrorType;

// TODO installation should move the settings file somewhere in the system
const SETTINGS_FILE_PATH: &str = "/home/malte/workspaces/file_converter/res/settings.ini";
const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Info;

pub struct GeneralSettings {
    pub log_level: LevelFilter,
    pub default_overwrite: bool,
}

impl GeneralSettings {
    pub fn new() -> Self {
        Self {
            log_level: DEFAULT_LOG_LEVEL,
            default_overwrite: false,
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        match key.as_str() {
            "log_level" => {
                self.log_level = LevelFilter::from_str(&*value).unwrap_or_else(|_| {
                    warn!("Invalid log level '{}'. Using default: info", value);
                    DEFAULT_LOG_LEVEL
                });
            },
            "default_overwrite" => self.default_overwrite = value.parse::<bool>().unwrap(),
            _ => warn!("Setting {} is not supported. Ignoring.", key)
        }
    }
}

pub struct JsonConverterSettings {
    pub indent: u8,
}

impl JsonConverterSettings {
    pub fn new() -> Self {
        Self {
            indent: 4,
        }
    }

    #[allow(clippy::duplicate_code)]
    pub fn set(&mut self, key: String, value: String) {
        match key.as_str() {
            "indent" => self.indent = value.parse::<u8>().unwrap(),
            _ => warn!("Setting {} is not supported. Ignoring.", key)
        }
    }
}

pub struct YamlConverterSettings {
    pub indent: u8,
}

impl YamlConverterSettings {
    pub fn new() -> Self {
        Self {
            indent: 2,
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        match key.as_str() {
            "indent" => self.indent = value.parse::<u8>().unwrap(),
            _ => warn!("Setting {} is not supported. Ignoring.", key)
        }
    }
}

pub struct Settings {
    pub general: GeneralSettings,
    pub json: JsonConverterSettings,
    pub yaml: YamlConverterSettings
}

impl Settings {
    pub fn load() -> Result<Settings, SettingsErrorType> {
        let settings_file = Path::new(SETTINGS_FILE_PATH);

        if !settings_file.is_file() {
            println!(
                "Error: Settings file not found: {:?}",
                settings_file.as_os_str().to_str().unwrap()
            );
            return Err(SettingsErrorType::SettingsFileNotFound);
        }

        let contents: String = fs::read_to_string(SETTINGS_FILE_PATH)
            .expect("Unable to read settings file");

        let mut general_settings: GeneralSettings = GeneralSettings::new();
        let mut json_settings: JsonConverterSettings = JsonConverterSettings::new();
        let mut yaml_settings: YamlConverterSettings = YamlConverterSettings::new();

        let mut current_section: String = String::new();
        for line in contents.lines() {
            let trimmed_line: String = line.trim().to_string();

            if trimmed_line.starts_with("[") && trimmed_line.ends_with("]") {
                current_section = trimmed_line[1..line.len()-1].to_string();
            } else {
                if trimmed_line.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = trimmed_line.split("=").collect();
                if parts.len() != 2 {
                    return Err(SettingsErrorType::InvalidSettingsFile(line.to_string()));
                }

                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();

                match current_section.as_str() {
                    "General" => { general_settings.set(key, value) }
                    "JSON_Converter" => { json_settings.set(key, value) }
                    "YAML_Converter" => { yaml_settings.set(key, value) }
                    _ => { return Err(SettingsErrorType::SectionNotFound) }
                }
            }
        }

        Ok(Settings { general: general_settings, json: json_settings, yaml: yaml_settings })
    }
}
