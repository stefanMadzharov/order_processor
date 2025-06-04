use order_processor::{configs, parser};
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
                                prefixes.push(file_stem.to_string());
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
    println!("File names: {:?}", file_names.len());

    let parsed_orders = parser::parse_names(&*file_names);
    println!("Parsed orders: {:?}", parsed_orders.len());

    for result in parsed_orders {
        match result {
            Ok(order) => println!("Parsed: {}", order),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    // if let Err(e) = write_to_excel(&orders) {
    //     eprintln!("Failed to write Excel: {}", e);
    // }
}
