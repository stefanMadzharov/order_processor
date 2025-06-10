use either::Either;
use itertools::Itertools;
use order_processor::parser::ParseStickerError;
use order_processor::{configs, excel, parser, sticker::Sticker};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use strsim::normalized_levenshtein;

fn get_cdr_prefixes_recursively(dir: &Path) -> Vec<String> {
    let mut prefixes = Vec::new();

    fn visit_dir(path: &Path, prefixes: &mut Vec<String>) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    visit_dir(&entry_path, prefixes);
                } else if let Some(ext) = entry_path.extension() {
                    if ext.eq_ignore_ascii_case("cdr") {
                        if let Some(file_stem) = entry_path.file_stem().and_then(|s| s.to_str()) {
                            if !file_stem.to_owned().to_uppercase().contains("BACKUP")
                                && !file_stem.chars().take(1).contains(&'C')
                            {
                                prefixes
                                    .push(file_stem.to_string().to_uppercase().replace(" _", "_"));
                            }
                        }
                    }
                }
            }
        }
    }

    visit_dir(dir, &mut prefixes);
    prefixes
}

fn print_errors(errors: &[ParseStickerError], configs: &configs::Configs) {
    let orders = match excel::parse_orders(
        configs
            .order_path
            .to_str()
            .ok_or("Invalid file path")
            .expect("Wrong order path"),
    ) {
        Ok(orders) => orders,
        Err(e) => {
            eprintln!("Failed to parse orders: {:?}", e);
            return;
        }
    };

    eprintln!("\nFiltered Errors Based on Description Similarity:");

    for error in errors {
        let maybe_error_str = match error {
            ParseStickerError::MissingCode(desc) => Some(desc),
            ParseStickerError::MissingDescription(desc) => Some(desc),
            ParseStickerError::MissingDimensions(desc) => Some(desc),
            ParseStickerError::MissingMaterial(desc) => Some(desc),
            ParseStickerError::UnknownColor(desc) => Some(desc),
            ParseStickerError::UnknownMaterial(desc) => Some(desc),
        };

        if let Some(error_str) = maybe_error_str {
            for order in &orders {
                let similarity = normalized_levenshtein(error_str, &order.description);
                if similarity >= configs.error_output_levenshtein_distance {
                    eprintln!(
                        "Error: {}\n  ↳ Similar to Order: \"{}\" (similarity: {:.2})",
                        error, order.description, similarity
                    );
                    break;
                }
            }
        }
    }
}

fn main() {
    let configs = configs::Configs::load_from_file("configs.txt");

    let file_names = get_cdr_prefixes_recursively(&configs.archive_path);

    let parsing_results = parser::parse_names(&*file_names);

    let (mut stickers, errors): (Vec<Sticker>, Vec<ParseStickerError>) =
        parsing_results.into_iter().partition_map(|res| match res {
            Ok(sticker) => Either::Left(sticker),
            Err(error) => Either::Right(error),
        });

    let mut unrecoverable_errors = vec![];

    for error in errors {
        if let ParseStickerError::MissingCode(_) = error {
            match parser::try_infering_code_by_description_similiarity_measure(
                error,
                &stickers,
                configs.inferring_levenshtein_distance,
            ) {
                Ok(similar_stickers) => {
                    for mut similar_sticker in similar_stickers {
                        similar_sticker.description =
                            similar_sticker.description + &" !!!INFERRED!!!".to_owned();
                        stickers.push(similar_sticker);
                    }
                }
                Err(error) => {
                    unrecoverable_errors.push(error);
                }
            }
        } else {
            unrecoverable_errors.push(error);
        }
    }

    stickers.sort_by(|a, b| a.code.cmp(&b.code));
    stickers.dedup();

    let mut code_to_stickers_hashmap: HashMap<u64, Vec<Sticker>> = HashMap::new();

    for sticker in &stickers {
        code_to_stickers_hashmap
            .entry(sticker.code)
            .or_insert_with(Vec::new)
            .push(sticker.clone());
    }

    if !unrecoverable_errors.is_empty() {
        print_errors(&unrecoverable_errors, &configs);
    }

    if let Err(e) = excel::write_tables(configs.order_path, &code_to_stickers_hashmap) {
        eprintln!("Failed to write tables: {e:?}");
    }
}
