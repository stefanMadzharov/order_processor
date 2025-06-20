use super::parse_stcker_error::ParseStickerError;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum Color {
    Red,
    Green,
    Blue,
    #[default]
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

impl std::str::FromStr for Color {
    type Err = ParseStickerError;

    fn from_str(color_string: &str) -> Result<Self, Self::Err> {
        match color_string {
            s if s.contains("RED") => Ok(Color::Red),
            s if s.contains("GREEN") => Ok(Color::Green),
            s if s.contains("BLUE") => Ok(Color::Blue),
            s if s.contains("BLACK") || s.contains("BLK") => Ok(Color::Black),
            _ => Err(ParseStickerError::UnknownColor(color_string.to_string())),
        }
    }
}
