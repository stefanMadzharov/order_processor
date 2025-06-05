use crate::sticker::Sticker;
use chrono::Local;
use xlsxwriter::*;

fn _write_to_excel(orders: &[Sticker]) -> Result<(), XlsxError> {
    let date_str = Local::now().format("%y_%m_%d").to_string();
    let filename = format!("{}_orders.xlsx", date_str);

    let workbook = Workbook::new(&filename).unwrap();
    let mut sheet = workbook.add_worksheet(None)?;

    // Headers
    let headers = [
        "code",
        "description",
        "dimensions",
        "material",
        "color",
        "double_sticker",
    ];

    for (i, header) in headers.iter().enumerate() {
        sheet.write_string(0, i as u16, header, None)?;
    }

    // Data
    for (row, order) in orders.iter().enumerate() {
        let dims = if order.dimensions.len() > 1 {
            order
                .dimensions
                .iter()
                .enumerate()
                .map(|(i, d)| format!("{} - {}", i + 1, d))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            order
                .dimensions
                .first()
                .cloned()
                .unwrap_or_else(|| "N/A".to_string())
        };

        sheet.write_string((row + 1) as u32, 0, &order.code.to_string(), None)?;
        sheet.write_string((row + 1) as u32, 1, &order.description, None)?;
        sheet.write_string((row + 1) as u32, 2, &dims, None)?;
        sheet.write_string((row + 1) as u32, 3, &order.material.to_string(), None)?;
        sheet.write_string((row + 1) as u32, 4, &order.text_color.to_string(), None)?;
    }

    workbook.close()?;
    println!("Excel file written to: {}", filename);
    Ok(())
}
