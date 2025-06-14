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
