use crate::structs::{
    color::Color, dimensions::Dimensions, material::Material,
    parse_stcker_error::ParseStickerError, sticker::Sticker,
};
use rayon::prelude::*;
use regex::Regex;
use strsim::normalized_levenshtein;

pub fn extract_code(name: &str, code_re: &Regex) -> Result<u64, ParseStickerError> {
    code_re
        .captures(name)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().parse::<u64>())
        .transpose()
        .map_err(|_| ParseStickerError::MissingCode(name.to_string()))?
        .ok_or_else(|| ParseStickerError::MissingCode(name.to_string()))
}

pub fn split_at_dimensions<'a>(
    name: &'a str,
    dimensions_re: &Regex,
) -> Result<(&'a str, &'a str), ParseStickerError> {
    let output = dimensions_re
        .find_iter(name)
        .find_map(|m| {
            m.as_str()
                .parse::<Dimensions>()
                .ok()
                .map(|_| name.split_at(m.start()))
        })
        .ok_or_else(|| ParseStickerError::MissingDimensions(name.to_string()));
    output
}

pub fn extract_dimensions(end_string: &str, dimensions_re: &Regex) -> Vec<Dimensions> {
    dimensions_re
        .find_iter(end_string)
        .filter_map(|m| m.as_str().parse::<Dimensions>().ok())
        .collect()
}

pub fn extract_description(
    name_parts: (&str, &str),
    code: u64,
) -> Result<String, ParseStickerError> {
    name_parts
        .0
        .split_once(code.to_string().as_str())
        .map(|(_, tail)| tail.trim_matches(['_', ' '].as_ref()).to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            ParseStickerError::MissingDescription(format!("{}{}", name_parts.0, name_parts.1))
        })
}

pub fn extract_material(
    material_part: &str,
    material_re: &Regex,
    name: &str,
) -> Result<Material, ParseStickerError> {
    let material_part = material_part
        .replace("OK", "")
        .replace("DV", "")
        .replace("PF", "")
        .replace("ST", "");
    let material_part = material_part.trim_matches(['_', ' ', '.']);

    let longest_match = material_re
        .find_iter(material_part)
        .max_by_key(|m| m.as_str().len());

    if let Some(m) = longest_match {
        m.as_str().parse()
    } else if name.contains("LEAFLET") {
        Ok(Material::LEAFLET)
    } else {
        Err(ParseStickerError::MissingMaterial(name.to_owned()))
    }
}

pub fn extract_color(end_part: &str, color_re: &Regex) -> Option<Color> {
    let end_part = end_part.trim_matches(['_', ' ', '.']);

    if let Some(m) = color_re.find(end_part) {
        m.as_str().parse().ok()
    } else {
        None
    }
}

pub fn parse_names(names: &[String]) -> Vec<Result<Vec<Sticker>, ParseStickerError>> {
    let code_re = Regex::new(r"^(\d{3,})").unwrap();
    let dimensions_re = Regex::new(r"\d+[ХX]\d+").unwrap();
    let material_re = Regex::new(
        r"(?i)PAPER(?:[_ (.]*GR[_ ).])?|LEAFLET|PP|PVC(?:[_ ().]*R(?:[_ ().]*SLV)?)?|SLV",
    )
    .unwrap();
    let color_re = Regex::new(r"(?i)BLK|BLACK|RED|GREEN|BLUE").unwrap();

    names
        .par_iter()
        .map(|name| {
            Sticker::parse_stickers(name, &code_re, &dimensions_re, &material_re, &color_re)
        })
        .collect()
}

pub fn try_infering_code_by_description_similiarity_measure(
    error: ParseStickerError,
    parsed_stickers: &[Sticker],
    levenshtein_distance_bound: f64,
) -> Result<Vec<Sticker>, ParseStickerError> {
    let code_re = Regex::new(r"^(\d{3,})").unwrap();
    let dimensions_re = Regex::new(r"\d+[ХX]\d+").unwrap();
    let material_re = Regex::new(
        r"(?i)PAPER(?:[_ (.]*GR[_ ).])?|LEAFLET|PP|PVC(?:[_ ().]*R(?:[_ ().]*SLV)?)?|SLV",
    )
    .unwrap();
    let color_re = Regex::new(r"(?i)BLK|BLACK|RED|GREEN|BLU").unwrap();

    if let ParseStickerError::MissingCode(name) = &error {
        let error_description = split_at_dimensions(name, &dimensions_re)?
            .0
            .trim_matches(['_', ' '].as_ref());

        let similar_stickers: Vec<Sticker> = parsed_stickers
            .iter()
            .enumerate()
            .map(|(i, sticker)| {
                (
                    i,
                    normalized_levenshtein(error_description, &sticker.description).abs(),
                )
            })
            .filter(|(_, levenshtein)| *levenshtein >= levenshtein_distance_bound)
            .flat_map(|(i, _)| {
                Sticker::parse_stickers(
                    (parsed_stickers[i].code.clone().to_string() + name).as_str(),
                    &code_re,
                    &dimensions_re,
                    &material_re,
                    &color_re,
                )
                .unwrap()
            })
            .collect();

        if similar_stickers.is_empty() {
            Err(error)
        } else {
            Ok(similar_stickers)
        }
    } else {
        Err(error)
    }
}
