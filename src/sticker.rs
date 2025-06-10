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

#[derive(Debug, Clone)]
pub struct Order {
    pub code: u64,
    pub amount: u64,
    pub description: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Material {
    Paper,
    PaperGR,
    PVC,
    PVCR,
    PVCRSLV,
    LEAFLET,
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let material_str = match self {
            Material::Paper => "PAPER",
            Material::PaperGR => "PAPER GR",
            Material::PVC => "PVC",
            Material::PVCR => "PVC R",
            Material::PVCRSLV => "PVC R SLV",
            Material::LEAFLET => "LEAFLET",
        };
        write!(f, "{}", material_str)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
