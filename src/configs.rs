use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub struct Configs {
    pub archive_path: PathBuf,
    pub order_path: PathBuf,
    pub inferring_levenshtein_distance: f64,
    pub error_output_levenshtein_distance: f64,
}

const DEFAULT_INFERRING_LEVENSHTEIN_DISTANCE: f64 = 0.93; // corresponds to 1-2 edits
const DEFAULT_ERROR_OUTPUT_LEVENSHTEIN_DISTANCE: f64 = 0.7;

impl Configs {
    pub fn load_from_file<P: AsRef<Path>>(config_path: P) -> Self {
        let file = fs::File::open(&config_path)
            .unwrap_or_else(|_| panic!("Failed to open config file: {:?}", config_path.as_ref()));

        let reader = io::BufReader::new(file);

        let mut archive_path: Option<PathBuf> = None;
        let mut order_path: Option<PathBuf> = None;
        let mut inferring_levenshtein_distance: f64 = DEFAULT_INFERRING_LEVENSHTEIN_DISTANCE;
        let mut error_output_levenshtein_distance: f64 = DEFAULT_ERROR_OUTPUT_LEVENSHTEIN_DISTANCE;

        for line_result in reader.lines() {
            let line = line_result.expect("Failed to read line from config file");
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                match key {
                    "archive" => archive_path = Some(PathBuf::from(value)),
                    "order" => order_path = Some(PathBuf::from(value)),
                    "inferring_levenshtein_distance" => {
                        inferring_levenshtein_distance = value.parse().unwrap_or_else(|_| {
                            panic!(
                                "Invalid float for inferring_levenshtein_distance: {}",
                                value
                            )
                        });
                    }
                    "error_output_levenshtein_distance" => {
                        error_output_levenshtein_distance = value.parse().unwrap_or_else(|_| {
                            panic!(
                                "Invalid float for inferring_levenshtein_distance: {}",
                                value
                            )
                        });
                    }
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

        if inferring_levenshtein_distance == DEFAULT_INFERRING_LEVENSHTEIN_DISTANCE {
            println!(
                "!!!WARNING: USING DEFAULT INFERRING LEVENSHTEIN DISTANCE OF {:.2}!!!",
                DEFAULT_INFERRING_LEVENSHTEIN_DISTANCE
            );
            println!("For custom value set in \'configs.txt\', e.g.");
            println!("inferring_levenshtein_distance=0.9")
        }

        if error_output_levenshtein_distance == DEFAULT_ERROR_OUTPUT_LEVENSHTEIN_DISTANCE {
            println!(
                "!!!WARNING: USING DEFAULT ERROR OUTPUT LEVENSHTEIN DISTANCE OF {:.2}!!!",
                DEFAULT_ERROR_OUTPUT_LEVENSHTEIN_DISTANCE
            );
            println!("For custom value set in \'configs.txt\', e.g.");
            println!("error_output_levenshtein_distance=0.5")
        }

        Configs {
            archive_path,
            order_path,
            inferring_levenshtein_distance,
            error_output_levenshtein_distance,
        }
    }
}
