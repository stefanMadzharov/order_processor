use super::{color::Color, material::Material};

#[derive(Debug, Clone, Eq)]
pub struct Sticker {
    pub code: u64,
    pub description: String,
    pub dimensions: Vec<String>,
    pub material: Material,
    pub text_color: Color,
    pub full_name: String,
}

impl Sticker {
    pub fn new(
        code: &str,
        description: &str,
        dimensions: Vec<String>,
        material: Material,
        text_color: Color,
        full_name: String,
    ) -> Sticker {
        Sticker {
            code: code
                .to_owned()
                .parse()
                .expect(("Coulndn't parse code into u64".to_owned() + &code.to_owned()).as_str()),
            description: description.to_owned(),
            dimensions,
            material,
            text_color,
            full_name,
        }
    }

    pub fn split(&self) -> Vec<Self> {
        self.clone()
            .dimensions
            .into_iter()
            .map(|sub_sticker_dims| {
                let mut substicker = self.clone();
                substicker.dimensions = vec![sub_sticker_dims];
                substicker
            })
            .collect()
    }
}

impl std::fmt::Display for Sticker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl PartialEq for Sticker {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.description == other.description
            && self.dimensions == other.dimensions
            && self.material == other.material
            && self.text_color == other.text_color
    }
}
