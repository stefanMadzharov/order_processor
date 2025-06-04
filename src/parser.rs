use crate::sticker::Sticker;
use rayon::prelude::*;
use regex::Regex;

fn extract_code<'a>(name: &'a str, code_re: &Regex) -> Result<&'a str, ParseStickerError> {
    code_re
        .captures(name)
        .and_then(|caps| caps.get(1).map(|m| m.as_str()))
        .ok_or_else(|| ParseStickerError::MissingCode(name.to_string()))
}

fn extract_dimensions_str<'a>(
    name: &'a str,
    dimensions_re: &Regex,
) -> Result<&'a str, ParseStickerError> {
    dimensions_re
        .captures(name)
        .and_then(|caps| caps.get(1).map(|m| m.as_str()))
        .ok_or_else(|| ParseStickerError::MissingDimensions(name.to_string()))
}

fn extract_dimensions(dimensions_str: &str) -> Vec<String> {
    dimensions_str
        .split(['_', ' ', '-'])
        .map(|s| s.to_string())
        .collect()
}

fn extract_description(
    name: &str,
    code: &str,
    dimensions_str: &str,
) -> Result<String, ParseStickerError> {
    name.split_once(code)
        .and_then(|(_, tail)| tail.split_once(dimensions_str))
        .map(|(middle, _)| middle.trim_matches(['_', ' '].as_ref()).to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ParseStickerError::MissingDescription(name.to_string()))
}

fn extract_material_and_color(
    name: &str,
    dimensions_str: &str,
) -> (Result<String, ParseStickerError>, String) {
    // Regex for matching material and optional color
    let re = Regex::new(
        r"(?i)(?P<material>paper(?: GR)?|PVC(?: R(?: SLV)?)?|LEAFLET)(?:[_ ]+(?P<color>BLK|BLACK|RED|GREEN|BLUE))?"
    ).unwrap();

    name.split_once(dimensions_str)
        .map(|(_, end)| {
            let end = end.trim_matches(['_', ' ']);

            if let Some(caps) = re.captures(end) {
                let material = caps
                    .name("material")
                    .map(|m| m.as_str().to_lowercase())
                    .ok_or_else(|| ParseStickerError::MissingMaterial(name.to_string()));

                let color = caps
                    .name("color")
                    .map(|c| c.as_str().to_uppercase())
                    .unwrap_or_else(|| "BLACK".to_string());

                (material, color)
            } else {
                (
                    if name.contains("LEAFLET") {
                        Ok("paper".to_string())
                    } else {
                        Err(ParseStickerError::MissingMaterial(name.to_string()))
                    },
                    "Black".to_string(),
                )
            }
        })
        .unwrap_or_else(|| {
            (
                Err(ParseStickerError::MissingMaterial(name.to_string())),
                "Black".to_string(),
            )
        })
}

fn is_double_sticker(name: &str) -> bool {
    name.to_lowercase().contains("dvoen")
}

pub fn parse_names(names: &[String]) -> Vec<Result<Sticker, ParseStickerError>> {
    let code_re = Regex::new(r"^(\d{3,})").unwrap();
    let dimensions_re = Regex::new(r"(\d+X\d+(?:_\d+X\d+)*)").unwrap();

    names
        .par_iter()
        .map(|name| {
            let code = extract_code(name, &code_re)?;
            let dimensions_str = extract_dimensions_str(name, &dimensions_re)?;
            let dimensions = extract_dimensions(dimensions_str)
                .iter()
                .map(|dimensions| dimensions.to_lowercase())
                .collect();
            let description = extract_description(name, code, dimensions_str)?;
            let (material, color) = extract_material_and_color(name, dimensions_str);
            let material = material?;
            let double_sticker = is_double_sticker(name);

            Ok(Sticker::new(
                code,
                &description,
                dimensions,
                &*material,
                &*color,
                double_sticker,
                name.clone(),
            ))
        })
        .collect()
}

#[derive(Debug)]
pub enum ParseStickerError {
    MissingCode(String),
    MissingDescription(String),
    MissingDimensions(String),
    MissingMaterial(String),
}

use std::fmt;

impl fmt::Display for ParseStickerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseStickerError::MissingCode(name) => {
                write!(f, "Missing sticker code in: {}", name)
            }
            ParseStickerError::MissingDescription(name) => {
                write!(f, "Missing description in: {}", name)
            }
            ParseStickerError::MissingDimensions(name) => {
                write!(f, "Missing dimensions in: {}", name)
            }
            ParseStickerError::MissingMaterial(name) => {
                write!(f, "Missing material in: {}", name)
            }
        }
    }
}
