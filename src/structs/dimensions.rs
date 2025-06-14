use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl std::fmt::Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl FromStr for Dimensions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((w, h)) = s.trim().split_once('x') {
            let width = w.parse::<u32>().map_err(|e| format!("Invalid width {e}"))?;
            let height = h
                .parse::<u32>()
                .map_err(|e| format!("Invalid height {e}"))?;
            if width > 0 && height > 0 {
                Ok(Dimensions { width, height })
            } else {
                Err(format!(
                    "Width and height must be greater than 0: {width}x{height}"
                ))
            }
        } else {
            Err("Invalid format, expected WxH".into())
        }
    }
}

pub fn read_dimensions_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Dimensions>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut dims = Vec::new();

    for line in reader.lines() {
        let line = line?;
        match line.parse::<Dimensions>() {
            Ok(dim) => dims.push(dim),
            Err(err) => eprintln!("Skipping invalid line '{}': {}", line, err),
        }
    }

    Ok(dims)
}
