pub struct Order {
    pub code: String,
    pub description: String,
    pub dimensions: Vec<String>,
    pub material: String,
    pub text_color: String,
    pub double_sticker: bool,
    pub full_name: String,
}

impl Order {
    pub fn new(
        code: &str,
        description: &str,
        dimensions: Vec<String>,
        material: &str,
        text_color: &str,
        double_sticker: bool,
        full_name: String,
    ) -> Order {
        Order {
            code: code.to_owned(),
            description: description.to_owned(),
            dimensions,
            material: material.to_owned(),
            text_color: text_color.to_owned(),
            double_sticker,
            full_name,
        }
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let dims_str = if self.double_sticker {
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

        if self.double_sticker {
            write!(
            f,
            "Code: {}, Description: {}, Dimensions: {}, Material: {}, Color: {}, Double Sticker: {}, Full Name: {}",
            self.code, self.description, dims_str, self.material, self.text_color, self.double_sticker, self.full_name,

        )
        } else {
            write!(
                f,
                "Code: {}, Description: {}, Dimensions: {}, Material: {}, Color: {}, Full Name: {}",
                self.code,
                self.description,
                dims_str,
                self.material,
                self.text_color,
                self.full_name,
            )
        }
    }
}
