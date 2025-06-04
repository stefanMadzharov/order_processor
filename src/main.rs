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
                            prefixes.push(file_stem.to_string());
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
    // let file_names = vec![
    //     "297_CAU Resveratrol Lift Instant Firming Serum - 30 mL_30x70_PVC_R_OK",
    //     "205043_AV ETA Collect 50ml_40x45_PVC_R_OK_PF",
    //     "205475_RF VITALFAN PROGR Single 30k_58x75_36x73_paper green_dvoen stiker_OK",
    //     "205671_AV COUV STICK KORAL Spf30 4gr_25x80_PVC_OK_PF",
    //     "205813_KL BBC GD FIGUIER 75ml_40x20_PVC_R_OK",
    // ];

    let configs = configs::Configs::load_from_file("configs.txt");

    let file_names = get_cdr_prefixes_recursively(&configs.archive);
    println!("File names: {:?}", file_names.len());

    let orders = parser::parse_names(&*file_names);
    println!("Parsed orders: {:?}", orders.len());

    for order in orders {
        println!("{order}");
    }

    // if let Err(e) = write_to_excel(&orders) {
    //     eprintln!("Failed to write Excel: {}", e);
    // }
}
