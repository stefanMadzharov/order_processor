#[derive(Clone)]
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
            ParseStickerError::MissingCode(_) => {
                write!(f, "Missing sticker code")
            }
            ParseStickerError::MissingDescription(_) => {
                write!(f, "Missing description")
            }
            ParseStickerError::MissingDimensions(_) => {
                write!(f, "Missing dimensions")
            }
            ParseStickerError::MissingMaterial(_) => {
                write!(f, "Missing material")
            }
            ParseStickerError::UnknownColor(_) => {
                write!(f, "Unknown color")
            }
            ParseStickerError::UnknownMaterial(_) => {
                write!(f, "Unknown material")
            }
        }
    }
}

impl fmt::Debug for ParseStickerError {
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
