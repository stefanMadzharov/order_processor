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