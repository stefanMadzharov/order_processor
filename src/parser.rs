use crate::structs::{
    color::Color, dimensions::Dimensions, material::Material,
    parse_stcker_error::ParseStickerError, sticker::Sticker,
};
use rayon::prelude::*;
use regex::Regex;
use std::{fs, path::Path};
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
        .map(|(_, tail)| tail.trim_matches(['_', ' ']).to_string())
        .filter(|desc| !desc.is_empty())
        .map(|mut desc| {
            if name_parts.1.contains("PROMO") {
                desc.push_str(" PROMO");
            }
            if name_parts.1.contains("TESTER") {
                desc.push_str(" TESTER");
            }
            desc
        })
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
        r"(?i)(PAPER([_ ().&-]+GR)?|LEAFLET|PP|PVC([_ ().&-]+R([_ ().&-]+SLV)?)?|SLV)([_ ().&-]+|$)",
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
        r"(?i)(PAPER([_ ().&-]+GR)?|LEAFLET|PP|PVC([_ ().&-]+R([_ ().&-]+SLV)?)?|SLV)([_ ().&-]+|$)",
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
                .unwrap_or_default()
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

const EXCLUDED_PREFIX_CHAR: char = 'C';

pub fn collect_cdr_prefixes(dir: &Path) -> Vec<String> {
    fn visit_dir(path: &Path, prefixes: &mut Vec<String>) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    visit_dir(&entry_path, prefixes);
                } else if entry_path
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("cdr"))
                {
                    if let Some(file_stem) = entry_path.file_stem().and_then(|s| s.to_str()) {
                        let upper_stem = file_stem.to_uppercase();
                        if !upper_stem.contains("BACKUP")
                            && !upper_stem.starts_with(EXCLUDED_PREFIX_CHAR)
                        {
                            prefixes.push(upper_stem.replace(" _", "_"));
                        }
                    }
                }
            }
        }
    }

    let mut prefixes = Vec::new();
    visit_dir(dir, &mut prefixes);
    prefixes
}
