use std::str::FromStr;

// import the genereted during build time official dimensions
include!(concat!(env!("OUT_DIR"), "/generated_dimensions.rs"));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl Dimensions {
    fn rev(&self) -> Self {
        Dimensions {
            width: self.height,
            height: self.width,
        }
    }
}

impl std::fmt::Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl FromStr for Dimensions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((w, h)) = s
            .trim()
            .to_owned()
            .to_uppercase()
            .replace("Х", "X")
            .split_once("X")
        {
            let width = w.parse::<u32>().map_err(|e| format!("Invalid width {e}"))?;
            let height = h
                .parse::<u32>()
                .map_err(|e| format!("Invalid height {e}"))?;
            if width > 0 && height > 0 {
                let dims = Dimensions { width, height };
                if OFFICIAL_DIMENSIONS.contains(&dims) {
                    Ok(dims)
                } else {
                    let rev = dims.rev();
                    if OFFICIAL_DIMENSIONS.contains(&rev) {
                        Ok(rev)
                    } else {
                        Err(format!("{} is not in the official dimensions", dims))
                    }
                }
            } else {
                Err(format!(
                    "Width and height must be greater than 0: {width}x{height}"
                ))
            }
        } else {
            Err(format!("Invalid format, expected \'WXH\' got \'{s}\'"))
        }
    }
}
