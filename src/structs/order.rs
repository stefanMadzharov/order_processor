#[derive(Debug, Clone)]
pub struct Order {
    pub code: String,
    pub amount: u64,
    pub description: String,
}
