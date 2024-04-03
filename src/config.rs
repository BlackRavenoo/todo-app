use colored::Color;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub default_list: String,
    pub output: OutputSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self{
            default_list: "default".to_string(),
            output: OutputSettings{
                text: TextSettings{
                    color: Color::BrightYellow,
                    bold: false,
                    italic: true,
                },
                err: TextSettings{
                    color: Color::Red,
                    bold: false,
                    italic: false,
                },
                list: TextSettings{
                    color: Color::BrightBlue,
                    bold: true,
                    italic: false,
                },
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OutputSettings {
    pub text: TextSettings,
    pub err: TextSettings,
    pub list: TextSettings,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TextSettings {
    #[serde(deserialize_with = "parse_color", serialize_with = "serialize_color")]
    pub color: Color,
    pub bold: bool,
    pub italic: bool,
}

fn serialize_color<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let color_string = match color {
        Color::Black => "black".to_string(),
        Color::Red => "red".to_string(),
        Color::Green => "green".to_string(),
        Color::Yellow => "yellow".to_string(),
        Color::Blue => "blue".to_string(),
        Color::Magenta => "magenta".to_string(),
        Color::Cyan => "cyan".to_string(),
        Color::White => "white".to_string(),
        Color::BrightBlack => "bright black".to_string(),
        Color::BrightRed => "bright red".to_string(),
        Color::BrightGreen => "bright green".to_string(),
        Color::BrightYellow => "bright yellow".to_string(),
        Color::BrightBlue => "bright blue".to_string(),
        Color::BrightMagenta => "bright magenta".to_string(),
        Color::BrightCyan => "bright cyan".to_string(),
        Color::BrightWhite => "bright white".to_string(),
        Color::TrueColor {r, g, b} => format!("#{:02x}{:02x}{:02x}", r, g, b),
    };

    serializer.serialize_str(&color_string)
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