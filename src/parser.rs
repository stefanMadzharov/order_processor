use crate::structs::{
    dimensions::Dimensions, parse_stcker_error::ParseStickerError, sticker::Sticker,
};
use rayon::prelude::*;
use regex::Regex;
use strsim::normalized_levenshtein;

pub fn extract_code(name: &str) -> Result<&str, ParseStickerError> {
    let code_re = Regex::new(r"^(\d{3,})").unwrap();
    code_re
        .captures(name)
        .and_then(|caps| caps.get(1).map(|m| m.as_str()))
        .ok_or_else(|| ParseStickerError::MissingCode(name.to_string()))
}

pub fn extract_dimensions_str(name: &str) -> Result<&str, ParseStickerError> {
    let dimensions_re = Regex::new(r"(\d+[ХX]\d+(?:[_ ]\d+X\d+)*)").unwrap();

    dimensions_re
        .captures(name)
        .and_then(|caps| caps.get(1).map(|m| m.as_str()))
        .ok_or_else(|| ParseStickerError::MissingDimensions(name.to_string()))
}

pub fn extract_dimensions(dimensions_str: &str) -> Vec<Dimensions> {
    dimensions_str
        .split(['_', ' ', '-'])
        .map(|s| {
            match s
                .to_owned()
                .to_lowercase()
                .replace('х', "x")
                .parse::<Dimensions>()
            {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{e}");
                    Dimensions {
                        width: 0,
                        height: 0,
                    }
                }
            }
        })
        .collect()
}

pub fn extract_description(
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

pub fn extract_material_and_color(
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
                        Ok("LEAFLET".to_string())
                    } else {
                        Err(ParseStickerError::MissingMaterial(name.to_string()))
                    },
                    "BLACK".to_string(),
                )
            }
        })
        .unwrap_or_else(|| {
            (
                Err(ParseStickerError::MissingMaterial(name.to_string())),
                "BLACK".to_string(),
            )
        })
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
    levenshtein_distance_bound: f64,
) -> Result<Vec<Sticker>, ParseStickerError> {
    // TODO add different similarity measures?
    match error {
        ParseStickerError::MissingCode(name) => {
            let dimensions_str = extract_dimensions_str(&name)?;

            let error_description = name
                .split_once(&dimensions_str)
                .map(|(description, _)| description.trim_matches(['_', ' '].as_ref()).to_string())
                .filter(|s| !s.is_empty())
                .ok_or_else(|| ParseStickerError::MissingCode(name.to_string()))?;

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
                .filter_map(|(i, _)| {
                    (parsed_stickers[i].code.clone().to_string() + &name)
                        .parse()
                        .ok()
                })
                .collect();

            if similar_stickers.is_empty() {
                Err(ParseStickerError::MissingCode(name.to_string()))
            } else {
                Ok(similar_stickers)
            }
        }
        _ => return Err(error),
    }
}
