pub struct Sticker {
    pub code: String,
    pub description: String,
    pub dimensions: Vec<String>,
    pub material: String,
    pub text_color: String,
    pub full_name: String,
}

impl Sticker {
    pub fn new(
        code: &str,
        description: &str,
        dimensions: Vec<String>,
        material: &str,
        text_color: &str,
        full_name: String,
    ) -> Sticker {
        Sticker {
            code: code.to_owned(),
            description: description.to_owned(),
            dimensions,
            material: material.to_owned(),
            text_color: text_color.to_owned(),
            full_name,
        }
    }
}

impl std::fmt::Display for Sticker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let dims_str = if self.dimensions.len() > 1 {
            self.dimensions
                .iter()
                .enumerate()
                .map(|(i, d)| format!("{} - {}", i + 1, d))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            self.dimensions
                .first()
                .cloned()
                .unwrap_or_else(|| "N/A".to_string())
        };

        write!(
            f,
            "Code: {}, Description: {}, Dimensions: {}, Material: {}, Color: {}",
            self.code, self.description, dims_str, self.material, self.text_color,
        )
    }
}

impl std::fmt::Debug for Sticker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let dims_str = if self.dimensions.len() > 1 {
            self.dimensions
                .iter()
                .enumerate()
                .map(|(i, d)| format!("{} - {}", i + 1, d))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            self.dimensions
                .first()
                .cloned()
                .unwrap_or_else(|| "N/A".to_string())
        };

        write!(
            f,
            "Code: {}, Description: {}, Dimensions: {}, Material: {}, Color: {}, Full Name: {}",
            self.code, self.description, dims_str, self.material, self.text_color, self.full_name,
        )
    }
}
