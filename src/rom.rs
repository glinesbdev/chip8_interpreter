use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub struct RomOptions {
    pub tickrate: u8,
    pub fill_color: Option<String>,
    pub background_color: Option<String>,
}

impl RomOptions {
    fn parse_tickrate(value: &Value) -> u8 {
        let value = value.to_string();

        if let Ok(value) = value.parse::<u8>() {
            return value;
        }

        0
    }
}

impl From<&Map<String, Value>> for RomOptions {
    fn from(json: &Map<String, Value>) -> Self {
        let tickrate = json.get("tickrate").map(Self::parse_tickrate).unwrap();
        let fill_color = json.get("fillColor").map(|value| value.to_string());
        let background_color = json.get("backgroundColor").map(|value| value.to_string());

        Self {
            tickrate,
            fill_color,
            background_color,
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub struct Rom {
    pub title: String,
    pub authors: Vec<String>,
    pub images: Vec<String>,
    pub desc: String,
    pub platform: String,
    pub options: RomOptions,
}

impl From<(&String, &Value)> for Rom {
    fn from(json: (&String, &Value)) -> Self {
        let authors = json.1["authors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();

        let images: Vec<String> = json.1["images"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();

        Self {
            title: json.0.to_string(),
            authors,
            desc: json.1["desc"].to_string(),
            images,
            platform: json.1["platform"].to_string(),
            options: RomOptions::from(json.1["options"].as_object().unwrap()),
        }
    }
}
