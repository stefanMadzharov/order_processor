use either::Either;
use itertools::Itertools;
use order_processor::parser::ParseStickerError;
use order_processor::{configs, excel, parser, sticker::Sticker};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
            match parser::try_infering_code_by_description_similiarity_measure(error, &stickers) {
                Ok(similar_stickers) => {
                    for mut similar_sticker in similar_stickers {
                        similar_sticker.description =
                            similar_sticker.description + &" !!!INFERRED!!!".to_owned();
                        stickers.push(similar_sticker);
                    }
                }
                Err(error) => {
                    // TODO output only similar names
                    unrecoverable_errors.push(error);
                }
            }
        } else {
            unrecoverable_errors.push(error);
        }
    }

    if !unrecoverable_errors.is_empty() {
        println!("\nUnparsed Errors:");
        for error in unrecoverable_errors {
            eprintln!("{}", error)
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

    if let Err(e) = excel::write_tables(configs.order_path, &code_to_stickers_hashmap) {
        eprintln!("Failed to write tables: {e:?}");
    }
}
