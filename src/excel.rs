use crate::sticker::{Color, Material, Sticker};
use chrono::Local;
use xlsxwriter::*;

pub fn write_table(stickers: &[Sticker]) -> Result<(), XlsxError> {
    let date_str = Local::now().format("%y_%m_%d").to_string();
    let filename = format!("{}_orders.xlsx", date_str);

    let workbook = Workbook::new(&filename).unwrap();
    let mut sheet = workbook.add_worksheet(None)?;

    // Headers
    let headers = ["code", "description", "dimensions", "material"];

    for (i, header) in headers.iter().enumerate() {
        sheet.write_string(0, i as u16, header, None)?;
    }

    // Data
    for (row, sticker) in stickers.iter().enumerate() {
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

        sheet.write_string((row + 1) as u32, 0, &sticker.code.to_string(), None)?;
        sheet.write_string((row + 1) as u32, 1, &sticker.description, None)?;
        sheet.write_string(
            (row + 1) as u32,
            2,
            &dims,
            Some(&sticker.text_color.clone().into()),
        )?;
        sheet.write_string(
            (row + 1) as u32,
            3,
            &sticker.material.to_string(),
            Some(&sticker.material.clone().into()),
        )?;
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
            Material::PVCRSLV => format::FormatColor::Purple,
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
