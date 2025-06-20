#[cfg(feature = "error_handling")]
use crate::{report, structs::parse_stcker_error::ParseStickerError};
#[cfg(feature = "error_handling")]
use either::Either;
#[cfg(feature = "error_handling")]
use itertools::Itertools;

use crate::{configs::Configs, excel, parser, structs::sticker::Sticker};
use std::collections::HashMap;

#[cfg(all(feature = "error_handling", not(feature = "no_inferring")))]
const INFERRED_MARKER: &str = " !!!INFERRED!!!";

#[cfg(all(feature = "error_handling", not(feature = "no_inferring")))]
pub fn run_inferring() {
    let configs = Configs::load_from_file("configs.txt");

    let file_names = parser::collect_cdr_prefixes(&configs.archive_path);
    let parsing_results = parser::parse_names(&file_names);

    let (stickers_nested, errors): (Vec<Vec<Sticker>>, Vec<ParseStickerError>) =
        parsing_results.into_iter().partition_map(|res| match res {
            Ok(sticker) => Either::Left(sticker),
            Err(error) => Either::Right(error),
        });

    let mut stickers: Vec<Sticker> = stickers_nested.into_iter().flatten().collect();
    let mut unrecoverable_errors = Vec::new();

    for error in errors {
        match error {
            ParseStickerError::MissingCode(_) => {
                match parser::try_infering_code_by_description_similiarity_measure(
                    error,
                    &stickers,
                    configs.inferring_levenshtein_distance,
                ) {
                    Ok(mut inferred_stickers) => {
                        for sticker in &mut inferred_stickers {
                            sticker.description.push_str(INFERRED_MARKER);
                        }
                        stickers.extend(inferred_stickers);
                    }
                    Err(e) => unrecoverable_errors.push(e),
                }
            }
            other => unrecoverable_errors.push(other),
        }
    }

    stickers.sort_by(|a, b| a.code.cmp(&b.code));
    stickers.dedup();

    let mut code_to_stickers_map: HashMap<u64, Vec<Sticker>> = HashMap::new();
    for sticker in &stickers {
        code_to_stickers_map
            .entry(sticker.code)
            .or_default()
            .push(sticker.clone());
    }

    if !unrecoverable_errors.is_empty() {
        report::print_relevant_errors(&unrecoverable_errors, &configs);
    }

    if let Err(e) = excel::write_tables(&configs, &code_to_stickers_map) {
        eprintln!("Failed to write tables: {e:?}");
    }

    // output all errors to see what kind of problems there are in the archive (also debugging)
    #[cfg(feature = "full_error_handling")]
    report::print_errors_grouped_by_type(&unrecoverable_errors);
}

#[cfg(all(feature = "error_handling", feature = "no_inferring"))]
pub fn run_no_inferring() {
    let configs = Configs::load_from_file("configs.txt");

    let file_names = parser::collect_cdr_prefixes(&configs.archive_path);
    let parsing_results = parser::parse_names(&file_names);

    let (stickers_nested, errors): (Vec<Vec<Sticker>>, Vec<ParseStickerError>) =
        parsing_results.into_iter().partition_map(|res| match res {
            Ok(sticker) => Either::Left(sticker),
            Err(error) => Either::Right(error),
        });

    let mut stickers: Vec<Sticker> = stickers_nested.into_iter().flatten().collect();

    stickers.sort_by(|a, b| a.code.cmp(&b.code));
    stickers.dedup();

    let mut code_to_stickers_map: HashMap<u64, Vec<Sticker>> = HashMap::new();
    for sticker in &stickers {
        code_to_stickers_map
            .entry(sticker.code)
            .or_default()
            .push(sticker.clone());
    }

    if !errors.is_empty() {
        report::print_relevant_errors(&errors, &configs);
    }

    if let Err(e) = excel::write_tables(&configs, &code_to_stickers_map) {
        eprintln!("Failed to write tables: {e:?}");
    }

    // output all errors to see what kind of problems there are in the archive (also debugging)
    #[cfg(feature = "full_error_handling")]
    report::print_errors_grouped_by_type(&errors);
}

#[cfg(not(feature = "error_handling"))]
pub fn run_optimized() {
    let configs = Configs::load_from_file("configs.txt");

    let file_names = parser::collect_cdr_prefixes(&configs.archive_path);
    let parsing_results = parser::parse_names(&file_names);

    let mut stickers: Vec<Sticker> = parsing_results
        .into_iter()
        .flat_map(|res| res.unwrap_or_default())
        .collect();

    stickers.sort_by(|a, b| a.code.cmp(&b.code));
    stickers.dedup();

    let mut code_to_stickers_map: HashMap<u64, Vec<Sticker>> = HashMap::new();
    for sticker in &stickers {
        code_to_stickers_map
            .entry(sticker.code)
            .or_default()
            .push(sticker.clone());
    }

    if let Err(e) = excel::write_tables(&configs, &code_to_stickers_map) {
        eprintln!("Failed to write tables: {e:?}");
    }
}
