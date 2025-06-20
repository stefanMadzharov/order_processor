pub mod configs;
pub mod excel;
pub mod parser;
#[cfg(feature = "error_handling")]
pub mod report;
pub mod runs;
pub mod structs {
    pub mod color;
    pub mod dimensions;
    pub mod material;
    pub mod order;
    pub mod parse_stcker_error;
    pub mod sticker;
}
