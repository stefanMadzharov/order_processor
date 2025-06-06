use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub struct Configs {
    pub archive_path: PathBuf,
    pub order_path: PathBuf,
}

impl Configs {
    pub fn load_from_file<P: AsRef<Path>>(config_path: P) -> Self {
        let file = fs::File::open(&config_path)
            .unwrap_or_else(|_| panic!("Failed to open config file: {:?}", config_path.as_ref()));

        let reader = io::BufReader::new(file);

        let mut archive_path: Option<PathBuf> = None;
        let mut order_path: Option<PathBuf> = None;

        for line_result in reader.lines() {
            let line = line_result.expect("Failed to read line from config file");
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                match key {
                    "archive" => archive_path = Some(PathBuf::from(value)),
                    "order" => order_path = Some(PathBuf::from(value)),
                    _ => continue,
                }
            }
        }

        let archive_path = archive_path.expect("Missing 'archive' key in config file");
        let order_path = order_path.expect("Missing 'order' key in config file");

        if !archive_path.is_dir() {
            panic!(
                "'archive' path is not a valid directory: {:?}",
                archive_path
            );
        }

        if !order_path.is_file() || order_path.extension().and_then(|e| e.to_str()) != Some("xlsx")
        {
            panic!("'order' path is not a valid .xlsx file: {:?}", order_path);
        }

        Configs {
            archive_path,
            order_path,
        }
    }
}
