use either::Either;
use itertools::Itertools;
use order_processor::excel::parse_orders;
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
                            if !file_stem.to_owned().to_lowercase().contains("backup") {
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

    let file_names = get_cdr_prefixes_recursively(&configs.archive);

    let parsed_names = parser::parse_names(&*file_names);

    let (mut stickers, errors): (Vec<Sticker>, Vec<ParseStickerError>) =
        parsed_names.into_iter().partition_map(|res| match res {
            Ok(sticker) => Either::Left(sticker),
            Err(error) => Either::Right(error),
        });

    let mut unrecoverable_errors = vec![];
    for error in errors {
        if let ParseStickerError::MissingCode(_) = error {
            match parser::try_infering_code_by_description_similiarity_measure(error, &stickers) {
                Ok(sticker) => {
                    stickers.push(sticker);
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

    // println!("Parsed Stickers:");
    // for sticker in &stickers {
    //     println!("{sticker}");
    // }

    // if unrecoverable_errors.len() > 1 {
    //     println!("\nUnparsed Errors:");
    //     for error in unrecoverable_errors {
    //         eprintln!("{}", error)
    //     }
    // }

    let mut code_to_orders_hashmap: HashMap<u64, Vec<Sticker>> = HashMap::new();

    for sticker in &stickers {
        code_to_orders_hashmap
            .entry(sticker.code)
            .or_insert_with(Vec::new)
            .push(sticker.clone());
    }

    for (key, vec) in &code_to_orders_hashmap {
        if vec.len() > 1 {
            println!("With {key} we have stickers:");
            for sticker in vec {
                println!("\t{sticker}");
            }
        }
    }

    let orders = parse_orders("./test.xlsx");
    dbg!(&orders);
    for order in orders.unwrap() {
        println!("{}-{}", order.0, order.1);
    }

    if let Err(e) = excel::write_table(&stickers) {
        eprintln!("Failed to write Excel: {}", e);
    }
}
