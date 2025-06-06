use crate::sticker::{Color, Material, Order, Sticker};
use calamine::{open_workbook_auto, Data, DataType, Reader};
use chrono::Local;
use std::collections::HashMap;
use std::error::Error;
use xlsxwriter::prelude::*;
use xlsxwriter::*;

type Coord = (usize, usize);

/// Parse orders from an Excel file
pub fn parse_orders(file_path: &str) -> Result<Vec<Order>, Box<dyn Error>> {
    let mut workbook = open_workbook_auto(&file_path)?;
    let range = workbook.worksheet_range("Sheet1")?;

    let cell1 = find_keyword_cell(&range, 6, 40, &["БГ СТИКЕР", "Френски код", "French code"])?;
    let cell2 = find_row_keyword_in_same_row(&range, cell1.0, 10, &["ПОРЪКА", "БРОЙ", "Order"])?;
    let cell3 = find_row_keyword_in_same_row(&range, cell1.0, 10, &["Описание", "Description"])?;

    let orders = extract_orders(&range, cell1.1, cell2.1, cell3.1, cell1.0)?;
    Ok(orders)
}

fn find_keyword_cell(
    range: &calamine::Range<Data>,
    max_cols: usize,
    max_rows: usize,
    keywords: &[&str],
) -> Result<Coord, Box<dyn Error>> {
    for row in 0..max_rows {
        for col in 0..max_cols {
            if let Some(val) = range.get((row, col)) {
                if let Some(s) = val.get_string() {
                    if keywords
                        .iter()
                        .any(|k| s.to_lowercase().contains(&k.to_lowercase()))
                    {
                        return Ok((row, col));
                    }
                }
            }
        }
    }
    Err("Keyword cell not found".into())
}

fn find_row_keyword_in_same_row(
    range: &calamine::Range<Data>,
    row: usize,
    max_cols: usize,
    keywords: &[&str],
) -> Result<Coord, Box<dyn Error>> {
    for col in 0..max_cols {
        if let Some(val) = range.get((row, col)) {
            if let Some(s) = val.get_string() {
                if keywords
                    .iter()
                    .any(|k| s.to_lowercase().contains(&k.to_lowercase()))
                {
                    return Ok((row, col));
                }
            }
        }
    }
    Err("Row keyword cell not found".into())
}

fn extract_orders(
    range: &calamine::Range<Data>,
    col_code: usize,
    col_amount: usize,
    col_description: usize,
    start_row: usize,
) -> Result<Vec<Order>, Box<dyn Error>> {
    let mut result = Vec::new();
    let mut started = false;

    for row in (start_row + 1)..range.height() {
        let code_opt = get_u64_from_cell(range.get((row, col_code)));

        match code_opt {
            Some(code) => {
                started = true;

                let amount = get_u64_from_cell(range.get((row, col_amount)))
                    .ok_or_else(|| format!("Invalid value in amount column at row {row}"))?;

                let description = range
                    .get((row, col_description))
                    .and_then(|cell| cell.get_string())
                    .unwrap_or("")
                    .trim()
                    .to_string();

                result.push(Order {
                    code,
                    amount,
                    descriptions: description,
                });
            }
            None if started => break,
            None => continue,
        }
    }

    Ok(result)
}

fn get_u64_from_cell(cell: Option<&Data>) -> Option<u64> {
    match cell {
        Some(Data::Int(n)) => Some(*n as u64),
        Some(Data::Float(f)) if *f >= 0.0 && f.fract() == 0.0 => Some(*f as u64),
        Some(Data::String(s)) => s.trim().parse::<u64>().ok(),
        _ => None,
    }
}

pub fn write_sizes_table(
    orders: &[(u64, u64)],
    code_to_stickers: &HashMap<u64, Vec<Sticker>>,
) -> Result<(), XlsxError> {
    let date_str = Local::now().format("%y_%m_%d").to_string();
    let filename = format!("{}_orders.xlsx", date_str);

    let workbook = Workbook::new(&filename)?;
    let mut sheet = workbook.add_worksheet(None)?;

    // Header row
    let headers = ["code", "description", "dimensions", "material", "amount"];
    let mut col_widths = headers.iter().map(|h| h.len()).collect::<Vec<_>>();

    // Base format with border
    let mut base_format = Format::new();
    base_format.set_border(FormatBorder::Thin);

    // Orange format for amount column
    let mut amount_format = base_format.clone();
    amount_format.set_bg_color(FormatColor::Orange);

    // Write headers
    for (col, header) in headers.iter().enumerate() {
        sheet.write_string(0, col as u16, header, Some(&base_format))?;
    }

    let mut row = 1; // start from second row (first is header)

    for (code, amount) in orders {
        if let Some(stickers) = code_to_stickers.get(code) {
            for sticker in stickers {
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
                    amount.to_string(),
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
                        4 => amount_format.clone(), // "amount" column
                        _ => base_format.clone(),
                    };

                    sheet.write_string(row, col as u16, value, Some(&format))?;

                    // Track max width
                    col_widths[col] = col_widths[col].max(value.len());
                }

                row += 1;
            }
        }
    }

    // Adjust column widths
    for (col, width) in col_widths.iter().enumerate() {
        sheet.set_column(col as u16, col as u16, *width as f64 + 2.0, None)?;
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
