use super::{
    color::Color, dimensions::Dimensions, material::Material, parse_stcker_error::ParseStickerError,
};
use crate::parser::{
    extract_code, extract_color, extract_description, extract_dimensions, extract_material,
    split_at_dimensions, DIMENSIONS_RE,
};

#[derive(Debug, Clone, Eq)]
pub struct Sticker {
    pub code: u64,
    pub description: String,
    pub dimensions: Dimensions,
    pub material: Material,
    pub text_color: Color,
    pub full_name: String,
}

impl Sticker {
    pub fn new(
        code: u64,
        description: &str,
        dimensions: Dimensions,
        material: Material,
        text_color: Color,
        full_name: String,
    ) -> Self {
        Self {
            code,
            description: description.to_owned(),
            dimensions,
            material,
            text_color,
            full_name,
        }
    }

    pub fn parse_stickers(name: &str) -> Result<Vec<Self>, ParseStickerError> {
        let code = extract_code(name)?;
        let name_parts = split_at_dimensions(name)?; // before and after first WxH
        let description = extract_description(name_parts, code)?;
        let dimensions = extract_dimensions(name_parts.1);
        let materials: Vec<Material> = DIMENSIONS_RE
            .split(name_parts.1)
            .filter_map(|material_part| extract_material(material_part, name).ok())
            .collect();
        let color = extract_color(name_parts.1).unwrap_or_default();

        if materials.is_empty() {
            return Err(ParseStickerError::MissingMaterial(name.to_owned()));
        }

        Ok(dimensions
            .iter()
            .cloned()
            .zip(if materials.len() == dimensions.len() {
                materials
            } else {
                std::iter::repeat_n(materials[0].clone(), dimensions.len()).collect()
            })
            .map(|(dimensions, material)| {
                Self::new(
                    code,
                    &description,
                    dimensions,
                    material,
                    color.clone(),
                    name.to_string(), // Preserve original name
                )
            })
            .collect())
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
