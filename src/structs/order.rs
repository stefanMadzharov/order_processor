#[derive(Debug, Clone)]
pub struct Order {
    pub code: u64,
    pub amount: u64,
    pub description: String,
}
