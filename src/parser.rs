use crate::sticker::Sticker;
use rayon::prelude::*;
use regex::Regex;

pub fn parse_names(names: &[String]) -> Vec<Result<Sticker, ParseStickerError>> {
    let code_re = Regex::new(r"^(\d{3,})").unwrap(); // Matches 3+ digits at start
    let dimensions_re = Regex::new(r"(\d+x\d+(?:_\d+x\d+)*)").unwrap();
    let material_re = Regex::new(r"(PVC(?:_R)?|paper(?: [a-z]+)?)").unwrap();

    names
        .par_iter()
        .map(|name| {
            // --- CODE ---
            let code = code_re
                .captures(name)
                .and_then(|caps| caps.get(1).map(|m| m.as_str()))
                .ok_or_else(|| ParseStickerError::MissingCode(name.clone()))?;

            // --- DIMENSIONS ---
            let dimensions_str = dimensions_re
                .captures(name)
                .and_then(|caps| caps.get(1).map(|m| m.as_str()))
                .ok_or_else(|| ParseStickerError::MissingDimensions(name.clone()))?;

            let dimensions: Vec<String> =
                dimensions_str.split('_').map(|s| s.to_string()).collect();

            // --- DESCRIPTION (between code and dimensions) ---
            let description = name
                .split_once(code)
                .and_then(|(_, tail)| tail.split_once(dimensions_str))
                .map(|(middle, _)| middle.trim_matches(['_', ' '].as_ref()).to_string())
                .filter(|s| !s.is_empty())
                .ok_or_else(|| ParseStickerError::MissingDescription(name.clone()))?;

            // --- MATERIAL / COLOR ---
            let (material, color) = material_re
                .find(name)
                .map(|mat| {
                    let material_str = mat.as_str();
                    if let Some(space_index) = material_str.find(' ') {
                        let (mat, col) = material_str.split_at(space_index);
                        (mat.trim(), col.trim())
                    } else {
                        (material_str, "black")
                    }
                })
                .unwrap_or(("", "black"));

            // --- DOUBLE STICKER ---
            let double_sticker = name.to_lowercase().contains("dvoen");

            Ok(Sticker::new(
                code,
                &description,
                dimensions,
                material,
                color,
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
    InvalidMaterial(String),
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
            ParseStickerError::InvalidMaterial(name) => {
                write!(f, "Invalid or missing material in: {}", name)
            }
        }
    }
}
