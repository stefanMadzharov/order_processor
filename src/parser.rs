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

pub fn extract_material_and_color(
    name_parts: (&str, &str),
) -> (Result<Material, ParseStickerError>, Color) {
    // Regex for matching material and optional color
    let re = Regex::new(
        r"(?i)(?P<material>paper(?:[_ (]GR[_ )])?|PVC(?:[_ ]R(?:[_ ]SLV)?)?|LEAFLET|PP)(?:[_ ]+(?P<color>BLK|BLACK|RED|GREEN|BLUE))?"
    ).unwrap();

    let name = format!("{}{}", name_parts.0, name_parts.1);
    let end = name_parts.1.trim_matches(['_', ' ']);

    if let Some(caps) = re.captures(end) {
        let material = caps
            .name("material")
            .and_then(|m| m.as_str().parse().ok())
            .ok_or(ParseStickerError::MissingMaterial(name));

        let color = caps
            .name("color")
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(Color::Black);

        (material, color)
    } else {
        (
            if name.contains("LEAFLET") {
                Ok(Material::LEAFLET)
            } else {
                Err(ParseStickerError::MissingMaterial(name))
            },
            Color::Black,
        )
    }
}

pub fn parse_names(names: &[String]) -> Vec<Result<Vec<Sticker>, ParseStickerError>> {
    let code_re = Regex::new(r"^(\d{3,})").unwrap();
    let dimensions_re = Regex::new(r"\d+[ХX]\d+").unwrap();

    names
        .par_iter()
        .map(|name| Sticker::parse_stickers(name, &code_re, &dimensions_re))
        .collect()
}

pub fn try_infering_code_by_description_similiarity_measure(
    error: ParseStickerError,
    parsed_stickers: &Vec<Sticker>,
    levenshtein_distance_bound: f64,
) -> Result<Vec<Sticker>, ParseStickerError> {
    if let ParseStickerError::MissingCode(name) = &error {
        let code_re = Regex::new(r"^(\d{3,})").unwrap();
        let dimensions_re = Regex::new(r"\d+[ХX]\d+").unwrap();

        let error_description = split_at_dimensions(&name, &dimensions_re)?
            .0
            .trim_matches(['_', ' '].as_ref());

        let similar_stickers: Vec<Sticker> = parsed_stickers
            .iter()
            .enumerate()
            .map(|(i, sticker)| {
                (
                    i,
                    normalized_levenshtein(&error_description, &sticker.description).abs(),
                )
            })
            .filter(|(_, levenshtein)| *levenshtein >= levenshtein_distance_bound)
            .flat_map(|(i, _)| {
                Sticker::parse_stickers(
                    (parsed_stickers[i].code.clone().to_string() + &name).as_str(),
                    &code_re,
                    &dimensions_re,
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
