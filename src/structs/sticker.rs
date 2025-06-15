use super::{
    color::Color, dimensions::Dimensions, material::Material, parse_stcker_error::ParseStickerError,
};
use crate::parser::{
    extract_code, extract_description, extract_dimensions, extract_dimensions_str,
    extract_material_and_color,
};

#[derive(Debug, Clone, Eq)]
pub struct Sticker {
    pub code: u64,
    pub description: String,
    pub dimensions: Vec<Dimensions>,
    pub material: Material,
    pub text_color: Color,
    pub full_name: String,
}

impl Sticker {
    pub fn new(
        code: &str,
        description: &str,
        dimensions: Vec<Dimensions>,
        material: Material,
        text_color: Color,
        full_name: String,
    ) -> Sticker {
        Sticker {
            code: code
                .to_owned()
                .parse()
                .expect(("Coulndn't parse code into u64".to_owned() + &code.to_owned()).as_str()),
            description: description.to_owned(),
            dimensions,
            material,
            text_color,
            full_name,
        }
    }

    pub fn split(&self) -> Vec<Self> {
        self.clone()
            .dimensions
            .into_iter()
            .map(|sub_sticker_dims| {
                let mut substicker = self.clone();
                substicker.dimensions = vec![sub_sticker_dims];
                substicker
            })
            .collect()
    }
}

use std::str::FromStr;

impl FromStr for Sticker {
    type Err = ParseStickerError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let code = extract_code(name)?;
        let dimensions_str = extract_dimensions_str(name)?;
        let dimensions = extract_dimensions(dimensions_str);
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

impl std::fmt::Display for Sticker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Code: {}, Description: {}, Dimensions: {:?}, Material: {}, Color: {}",
            self.code, self.description, self.dimensions, self.material, self.text_color,
        )
    }
}

impl PartialEq for Sticker {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.description == other.description
            && self.dimensions == other.dimensions
            && self.material == other.material
            && self.text_color == other.text_color
    }
}
