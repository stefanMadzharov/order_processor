use crate::sticker::{Color, Material, Sticker};
use chrono::Local;
use xlsxwriter::prelude::*;
use xlsxwriter::*;

pub fn write_table(stickers: &[Sticker]) -> Result<(), XlsxError> {
    let date_str = Local::now().format("%y_%m_%d").to_string();
    let filename = format!("{}_orders.xlsx", date_str);

    let workbook = Workbook::new(&filename)?;
    let mut sheet = workbook.add_worksheet(None)?;

    let headers = ["code", "description", "dimensions", "material"];
    let mut col_widths = vec![
        headers[0].len(),
        headers[1].len(),
        headers[2].len(),
        headers[3].len(),
    ];

    // Define base cell format with borders
    let mut base_format = Format::new();
    base_format.set_border(FormatBorder::Thin);

    // Write headers
    for (i, header) in headers.iter().enumerate() {
        sheet.write_string(0, i as u16, header, Some(&base_format))?;
    }

    // Write data rows
    for (row_idx, sticker) in stickers.iter().enumerate() {
        let row = (row_idx + 1) as u32;

        let dims = if sticker.dimensions.len() > 1 {
            sticker
                .dimensions
                .iter()
                .enumerate()
                .map(|(i, d)| format!("{} - {}", i + 1, d))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            sticker
                .dimensions
                .first()
                .cloned()
                .unwrap_or_else(|| "N/A".to_string())
        };

        let values = [
            sticker.code.to_string(),
            sticker.description.clone(),
            dims,
            sticker.material.to_string(),
        ];

        for (col, value) in values.iter().enumerate() {
            let format = match col {
                2 => {
                    let mut f = Format::from(sticker.text_color.clone());
                    f.set_border(FormatBorder::Thin);
                    f
                }
                3 => {
                    let mut f = Format::from(sticker.material.clone());
                    f.set_border(FormatBorder::Thin);
                    f
                }
                _ => base_format.clone(),
            };

            sheet.write_string(row, col as u16, value, Some(&format))?;

            // Track maximum width for column
            col_widths[col] = col_widths[col].max(value.len());
        }
    }

    // Set column widths (add padding)
    for (i, width) in col_widths.iter().enumerate() {
        sheet.set_column(i as u16, i as u16, *width as f64 + 2.0, None)?;
    }

    workbook.close()?;
    println!("Excel file written to: {}", filename);
    Ok(())
}

impl From<Material> for Format {
    fn from(material: Material) -> Self {
        let mut format = Format::new();
        let color = match material {
            Material::PVC => format::FormatColor::Yellow,
            Material::PVCR => format::FormatColor::Orange,
            Material::PVCRSLV => format::FormatColor::Magenta,
            _ => format::FormatColor::White,
        };
        format.set_bg_color(color);
        format
    }
}

impl From<Color> for Format {
    fn from(material: Color) -> Self {
        let mut format = Format::new();
        let color = match material {
            Color::Black => format::FormatColor::Gray,
            Color::Green => format::FormatColor::Green,
            Color::Blue => format::FormatColor::Blue,
            Color::Red => format::FormatColor::Red,
        };
        format.set_bg_color(color);
        format
    }
}
