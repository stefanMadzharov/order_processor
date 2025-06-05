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
    let dimensions_re = Regex::new(r"(\d+X\d+(?:_\d+X\d+)*)").unwrap();

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

fn parse_material(mat: &str) -> Option<Material> {
    match mat.to_lowercase().as_str() {
        "paper" => Some(Material::Paper),
        "paper gr" => Some(Material::PaperGR),
        "pvc" => Some(Material::PVC),
        "pvc r" => Some(Material::PVCR),
        "pvc r slv" => Some(Material::PVCRSLV),
        _ => None,
    }
}

fn parse_color(color: &str) -> Option<Color> {
    match color.to_lowercase().as_str() {
        "black" | "blk" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "blue" => Some(Color::Blue),
        _ => None,
    }
}

fn extract_material_and_color(
    name: &str,
    dimensions_str: &str,
) -> (Result<Material, ParseStickerError>, Color) {
    let re = Regex::new(
        r"(?i)(?P<material>paper(?: GR)?|PVC(?: R(?: SLV)?)?|LEAFLET)(?:[_ ]+(?P<color>BLK|BLACK|RED|GREEN|BLUE))?"
    ).unwrap();

    name.split_once(dimensions_str)
        .map(|(_, end)| {
            let end = end.trim_matches(['_', ' ']);

            if let Some(caps) = re.captures(end) {
                let material_str = caps.name("material").map(|m| m.as_str()).unwrap_or("");
                let color_str = caps.name("color").map(|c| c.as_str()).unwrap_or("black");

                let material = parse_material(material_str)
                    .ok_or_else(|| ParseStickerError::MissingMaterial(name.to_string()));

                let color = parse_color(color_str).unwrap_or(Color::Black);

                (material, color)
            } else {
                (
                    if name.contains("LEAFLET") {
                        Ok(Material::Paper)
                    } else {
                        Err(ParseStickerError::MissingMaterial(name.to_string()))
                    },
                    Color::Black,
                )
            }
        })
        .unwrap_or_else(|| {
            (
                Err(ParseStickerError::MissingMaterial(name.to_string())),
                Color::Black,
            )
        })
}

use std::str::FromStr;

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
            material,
            color,
            name.to_string(), // Preserve original name
        ))
    }
}

pub fn parse_names(names: &[String]) -> Vec<Result<Sticker, ParseStickerError>> {
    names.par_iter().map(|name| name.parse()).collect()
}

pub fn try_infering_code_by_description_similiarity_measure(
    error: ParseStickerError,
    parsed_stickers: &Vec<Sticker>,
) -> Result<Sticker, ParseStickerError> {
    // TODO add different similarity measures?
    match error {
        ParseStickerError::MissingCode(name) => {
            let dimensions_str = extract_dimensions_str(&name)?;

            println!("{name}");

            let error_description = name
                .split_once(&dimensions_str)
                .map(|(description, _)| description.trim_matches(['_', ' '].as_ref()).to_string())
                .filter(|s| !s.is_empty())
                .ok_or_else(|| ParseStickerError::MissingCode(name.to_string()))?;

            let (i, _) = parsed_stickers
                .iter()
                .enumerate()
                .map(|(i, sticker)| {
                    let levenshtein =
                        normalized_levenshtein(&error_description, &sticker.description).abs();
                    println!(
                        "Sim between {}, {} is {levenshtein}",
                        &error_description, &sticker.description
                    );
                    (i, levenshtein)
                })
                .filter(|(_, levenshtein)| *levenshtein > 0.95)
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
