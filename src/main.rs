use chrono::Local;
use rayon::prelude::*;
use regex::Regex;
use xlsxwriter::*;

struct Order {
    code: String,
    description: String,
    dimensions: Vec<String>,
    material: String,
    color: String,
    double_sticker: bool,
}

impl Order {
    fn new(
        code: &str,
        description: &str,
        dimensions: Vec<String>,
        material: &str,
        color: &str,
        double_sticker: bool,
    ) -> Order {
        Order {
            code: code.to_owned(),
            description: description.to_owned(),
            dimensions,
            material: material.to_owned(),
            color: color.to_owned(),
            double_sticker,
        }
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let dims_str = if self.double_sticker {
            self.dimensions
                .iter()
                .enumerate()
                .map(|(i, d)| format!("{} - {}", i + 1, d))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            self.dimensions
                .first()
                .cloned()
                .unwrap_or_else(|| "N/A".to_string())
        };

        if self.double_sticker {
            write!(
            f,
            "Code: {}, Description: {}, Dimensions: {}, Material: {}, Color: {}, Double Sticker: {}",
            self.code, self.description, dims_str, self.material, self.color, self.double_sticker
        )
        } else {
            write!(
                f,
                "Code: {}, Description: {}, Dimensions: {}, Material: {}, Color: {}",
                self.code, self.description, dims_str, self.material, self.color,
            )
        }
    }
}

fn parse_names(names: &[&str]) -> Vec<Order> {
    let re = Regex::new(r"^(\d+)_(.+?)_((?:\d+x\d+(?:_\d+x\d+)*))").unwrap();
    let material_re = Regex::new(r"(PVC(?:_R)?|paper(?: [a-z]+)?)").unwrap();

    names
        .par_iter()
        .filter_map(|name| {
            let caps = re.captures(name)?;

            let code = caps.get(1)?.as_str();
            let description = caps.get(2)?.as_str();
            let dimensions_str = caps.get(3)?.as_str();

            let dimensions: Vec<String> =
                dimensions_str.split('_').map(|s| s.to_string()).collect();

            let material_match = material_re.find(name);
            let (material, color) = if let Some(mat) = material_match {
                let material_str = mat.as_str();
                if let Some(space_index) = material_str.find(' ') {
                    let (mat, col) = material_str.split_at(space_index);
                    (mat.trim(), col.trim())
                } else {
                    (material_str, "black")
                }
            } else {
                ("", "black")
            };

            let double_sticker = name.to_lowercase().contains("dvoen");

            Some(Order::new(
                code,
                description,
                dimensions,
                material,
                color,
                double_sticker,
            ))
        })
        .collect()
}

fn _write_to_excel(orders: &[Order]) -> Result<(), XlsxError> {
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
        let dims = if order.double_sticker {
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

        sheet.write_string((row + 1) as u32, 0, &order.code, None)?;
        sheet.write_string((row + 1) as u32, 1, &order.description, None)?;
        sheet.write_string((row + 1) as u32, 2, &dims, None)?;
        sheet.write_string((row + 1) as u32, 3, &order.material, None)?;
        sheet.write_string((row + 1) as u32, 4, &order.color, None)?;
        sheet.write_boolean((row + 1) as u32, 5, order.double_sticker, None)?;
    }

    workbook.close()?;
    println!("Excel file written to: {}", filename);
    Ok(())
}

fn main() {
    let file_names = vec![
        "297_CAU Resveratrol Lift Instant Firming Serum - 30 mL_30x70_PVC_R_OK",
        "205043_AV ETA Collect 50ml_40x45_PVC_R_OK_PF",
        "205475_RF VITALFAN PROGR Single 30k_58x75_36x73_paper green_dvoen stiker_OK",
        "205671_AV COUV STICK KORAL Spf30 4gr_25x80_PVC_OK_PF",
        "205813_KL BBC GD FIGUIER 75ml_40x20_PVC_R_OK",
    ];

    // let file_names = file_names.repeat(10000);

    let orders = parse_names(&file_names);

    for order in orders {
        println!("{order}");
    }

    // if let Err(e) = write_to_excel(&orders) {
    //     eprintln!("Failed to write Excel: {}", e);
    // }
}
