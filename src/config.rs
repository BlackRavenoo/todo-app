use colored::Color;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub default_list: String,
    pub output: OutputSettings,
}

#[derive(Deserialize, Debug)]
pub struct OutputSettings {
    pub text: TextSettings,
    pub err: TextSettings
}

#[derive(Deserialize, Debug)]
pub struct TextSettings {
    #[serde(deserialize_with = "parse_color")]
    pub color: Color,
    pub bold: bool,
    pub italic: bool,
}

fn parse_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let c = s.chars().next(); 

    if c.is_none() {return Ok(Color::White)}

    if c.unwrap() == '#' {
        let hex_color = &s[1..];
        let r = u8::from_str_radix(&hex_color[0..2], 16).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        let g = u8::from_str_radix(&hex_color[2..4], 16).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        let b = u8::from_str_radix(&hex_color[4..6], 16).map_err(|e| serde::de::Error::custom(e.to_string()))?;

        Ok(Color::TrueColor {r, g, b})
    } else {
        Ok(s.parse::<Color>().map_err(|_| serde::de::Error::custom("Something went wrong."))?)
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let mut config_path = dirs::home_dir().expect("Failed to determine the home directory");
    config_path.push(".todo-app");

    let settings = config::Config::builder()
        .add_source(config::File::from(config_path.join("config")).required(true))
        .build()?;

    settings.try_deserialize::<Settings>()
}