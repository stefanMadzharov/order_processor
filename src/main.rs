use colored::*;
use either::Either;
use itertools::Itertools;
use order_processor::{
    configs, excel, parser,
    structs::{parse_stcker_error::ParseStickerError, sticker::Sticker},
};
use std::{collections::HashMap, fmt::Write, fs, path::Path};
use strsim::normalized_levenshtein; // Import the colored traits

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
    let orders = match excel::parse_orders(configs) {
        Ok(orders) => orders,
        Err(e) => {
            eprintln!("{}: {:?}", "Failed to parse orders".red().bold(), e);
            return;
        }
    };

    eprintln!(
        "\n{}: [{} {} {}]\n",
        "Filtered Errors Based on Description Similarity"
            .underline()
            .bold()
            .blue(),
        "Limit".bold(),
        configs
            .error_output_levenshtein_distance
            .to_string()
            .yellow(),
        "similarity".dimmed()
    );

    for order in &orders {
        let mut similarity_matches = String::new();
        let mut code_matches = String::new();

        for error in errors {
            let error_str = error.get_description();
            let similarity = normalized_levenshtein(
                error_str.as_str(),
                format!("{}_{}", order.code, &order.description).as_str(),
            );

            if similarity >= configs.error_output_levenshtein_distance {
                let _ = writeln!(
                    &mut similarity_matches,
                    "\t\t{} \"{}\" {} \"{}\" {} {:.2}{}",
                    "↳ Similar to file name:".cyan(),
                    error_str.yellow(),
                    "with error".dimmed(),
                    format!("{:?}", error).italic(),
                    "(similarity:".dimmed(),
                    similarity,
                    ")".dimmed(),
                );
            }

            if error_str.contains(&order.code.to_string()) {
                let _ = writeln!(
                    &mut code_matches,
                    "\t\t{} {}: {}",
                    "↳ Error contains code".magenta(),
                    order.code.to_string().yellow(),
                    format!("{:?}", error).italic()
                );
            }
        }

        if !similarity_matches.is_empty() || !code_matches.is_empty() {
            eprintln!(
                "{}: \"{}_{}\"",
                "Order".bright_blue().bold(),
                order.code.to_string().yellow(),
                order.description.green()
            );

            if !similarity_matches.is_empty() {
                eprintln!(
                    "\t{}:\n{}",
                    "Similarity Matches".bold().cyan(),
                    similarity_matches
                );
            }

            if !code_matches.is_empty() {
                eprintln!(
                    "\t{}:\n{}",
                    "Code Containment Matches".bold().magenta(),
                    code_matches
                );
            }

            eprintln!("{}", "_".repeat(100).dimmed());
        }
    }
}

fn main() {
    let configs = configs::Configs::load_from_file("configs.txt");

    let file_names = get_cdr_prefixes_recursively(&configs.archive_path);

    let parsing_results = parser::parse_names(&file_names);

    let (stickers, errors): (Vec<Vec<Sticker>>, Vec<ParseStickerError>) =
        parsing_results.into_iter().partition_map(|res| match res {
            Ok(sticker) => Either::Left(sticker),
            Err(error) => Either::Right(error),
        });

    let mut stickers: Vec<Sticker> = stickers.into_iter().flatten().collect();

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
                        similar_sticker.description += " !!!INFERRED!!!";
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
            .or_default()
            .push(sticker.clone());
    }

    if !unrecoverable_errors.is_empty() {
        print_errors(&unrecoverable_errors, &configs);
    }

    if let Err(e) = excel::write_tables(&configs, &code_to_stickers_hashmap) {
        eprintln!("Failed to write tables: {e:?}");
    }
}
