#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Material {
    Paper,
    PaperGR,
    PVC,
    PVCR,
    PVCRSLV,
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let material_str = match self {
            Material::Paper => "paper",
            Material::PaperGR => "paper GR",
            Material::PVC => "PVC",
            Material::PVCR => "PVC R",
            Material::PVCRSLV => "PVC R SLV",
        };
        write!(f, "{}", material_str)
    }
}

#[derive(Debug, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    Black,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_str = match self {
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Blue => "Blue",
            Color::Black => "Black",
        };
        write!(f, "{}", color_str)
    }
}
