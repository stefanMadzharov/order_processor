use crate::sticker::{Color, Material, Sticker};
use rayon::prelude::*;
use regex::Regex;
use strsim::normalized_levenshtein;

fn extract_code(name: &str) -> Result<&str, ParseStickerError> {
    let code_re = Regex::new(r"^(\d{3,})").unwrap();
    code_re
        .captures(name)
        .and_then(|caps| caps.get(1).map(|m| m.as_str()))
        .ok_or_else(|| ParseStickerError::MissingCode(name.to_string()))
}

fn extract_dimensions_str(name: &str) -> Result<&str, ParseStickerError> {
    let dimensions_re = Regex::new(r"(\d+[ХX]\d+(?:[_ ]\d+X\d+)*)").unwrap();

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
        r"(?i)(?P<material>paper(?:[_ (]GR[_ )])?|PVC(?:[_ ]R(?:[_ ]SLV)?)?|LEAFLET|PP)(?:[_ ]+(?P<color>BLK|BLACK|RED|GREEN|BLUE))?"
    ).unwrap();

    name.split_once(dimensions_str)
        .map(|(_, end)| {
            let end = end.trim_matches(['_', ' ']);

            if let Some(caps) = re.captures(end) {
                let material = caps
                    .name("material")
                    .map(|m| m.as_str().to_owned())
                    .ok_or_else(|| ParseStickerError::MissingMaterial(name.to_string()));

                let color = caps
                    .name("color")
                    .map(|c| c.as_str().to_owned())
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

use std::str::FromStr;

impl FromStr for Color {
    type Err = ParseStickerError;

    fn from_str(color_string: &str) -> Result<Self, Self::Err> {
        match color_string {
            s if s.contains("RED") => Ok(Color::Red),
            s if s.contains("GREEN") => Ok(Color::Green),
            s if s.contains("BLUE") => Ok(Color::Blue),
            s if s.contains("BLACK") || s.contains("BLK") => Ok(Color::Black),
            _ => Err(ParseStickerError::UnknownColor(color_string.to_string())),
        }
    }
}

impl FromStr for Material {
    type Err = ParseStickerError;

    fn from_str(material_string: &str) -> Result<Self, Self::Err> {
        match material_string {
            s if s.contains("GR") => Ok(Material::PaperGR),
            s if s.contains("PAP") | s.contains("PP")=> Ok(Material::Paper),
            //-------------------------------------------------------------------------
            s if s.contains("SLV") => Ok(Material::PVCRSLV),
            s if s.contains("R") => Ok(Material::PVCR),
            s if s.contains("PVC") => Ok(Material::PVC),
            _ => Err(ParseStickerError::UnknownMaterial(
                material_string.to_string(),
            )),
        }
    }
}

impl FromStr for Sticker {
    type Err = ParseStickerError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let code = extract_code(name)?;
        let dimensions_str = extract_dimensions_str(name)?;
        let dimensions = extract_dimensions(dimensions_str)
            .into_iter()
            .map(|d| d.to_lowercase())
            .collect();
        let description = extract_description(name, code, dimensions_str)?;
        let (material_result, color) = extract_material_and_color(name, dimensions_str);
        let material = material_result?;

        Ok(Sticker::new(
            code,
            &description,
            dimensions,
            material.parse()?,
            color.parse()?,
            name.to_string(), // Preserve original name
        ))
    }
}

pub fn parse_names(names: &[String]) -> Vec<Result<Sticker, ParseStickerError>> {
    names
        .par_iter()
        .flat_map(|name| {
            let sticker_parse_result = name.parse::<Sticker>();
            if let Ok(sticker) = &sticker_parse_result {
                sticker.split().into_iter().map(Ok).collect()
            } else {
                vec![sticker_parse_result]
            }
        })
        .collect()
}

pub fn try_infering_code_by_description_similiarity_measure(
    error: ParseStickerError,
    parsed_stickers: &Vec<Sticker>,
) -> Result<Sticker, ParseStickerError> {
    // TODO add different similarity measures?
    match error {
        ParseStickerError::MissingCode(name) => {
            let dimensions_str = extract_dimensions_str(&name)?;

            let error_description = name
                .split_once(&dimensions_str)
                .map(|(description, _)| description.trim_matches(['_', ' '].as_ref()).to_string())
                .filter(|s| !s.is_empty())
                .ok_or_else(|| ParseStickerError::MissingCode(name.to_string()))?;

            let (i, _) = parsed_stickers
                .iter()
                .enumerate()
                .map(|(i, sticker)| {
                    (
                        i,
                        normalized_levenshtein(&error_description, &sticker.description).abs(),
                    )
                })
                .filter(|(_, levenshtein)| *levenshtein >= 0.93)
                .max_by_key(|(_, levensthein)| (levensthein * 100.0) as u32)
                .ok_or_else(|| ParseStickerError::MissingCode(name.to_string()))?;

            return (parsed_stickers[i].code.clone().to_string() + &name).parse();
        }
        _ => return Err(error),
    }
}

#[derive(Debug, Clone)]
pub enum ParseStickerError {
    MissingCode(String),
    MissingDescription(String),
    MissingDimensions(String),
    MissingMaterial(String),
    UnknownColor(String),
    UnknownMaterial(String),
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
            ParseStickerError::UnknownColor(color_string) => {
                write!(f, "Unknown color in: {}", color_string)
            }
            ParseStickerError::UnknownMaterial(material_string) => {
                write!(f, "Unknown material in: {}", material_string)
            }
        }
    }
}
