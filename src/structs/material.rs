use super::parse_stcker_error::ParseStickerError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Material {
    Paper,
    PaperGR,
    PVC,
    PVCR,
    PVCRSLV,
    LEAFLET,
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let material_str = match self {
            Material::Paper => "PAPER",
            Material::PaperGR => "PAPER GR",
            Material::PVC => "PVC",
            Material::PVCR => "PVC R",
            Material::PVCRSLV => "PVC R SLV",
            Material::LEAFLET => "LEAFLET",
        };
        write!(f, "{}", material_str)
    }
}

use std::str::FromStr;

impl FromStr for Material {
    type Err = ParseStickerError;

    fn from_str(material_string: &str) -> Result<Self, Self::Err> {
        match material_string {
            s if s.contains("GR") => Ok(Material::PaperGR),
            s if s.contains("PAP") | s.contains("PP") => Ok(Material::Paper),
            s if s.contains("LEAFLET") => Ok(Material::LEAFLET),
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
