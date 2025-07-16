use crate::{
    configs::Configs,
    excel::parse_orders,
    structs::{dimensions::Dimensions, material::Material, sticker::Sticker},
};
use colored::*;
use std::{collections::HashMap, fs::File, io::Write};

fn group_material(material: &Material) -> &'static str {
    match material {
        Material::Paper | Material::PaperGR | Material::LEAFLET => "PAPER",
        Material::PVC | Material::PVCR | Material::PVCRSLV => "PVC",
    }
}

pub fn generate_material_report_for_orders(
    configs: &Configs,
    code_to_stickers_map: &HashMap<String, Vec<Sticker>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let orders = parse_orders(configs)?;
    let mut counts: HashMap<(String, Dimensions), u64> = HashMap::new();

    let mut missing_stickers: u32 = 0;

    for order in orders {
        if let Some(stickers) = code_to_stickers_map.get(&order.code) {
            for sticker in stickers {
                let group = group_material(&sticker.material).to_string();
                let key = (group, sticker.dimensions.clone());
                *counts.entry(key).or_insert(0) += order.amount;
            }
        } else {
            missing_stickers += 1;
        }
    }

    eprintln!(
        "\n{}: {}\n",
        "Missing sticker files".underline().bold().blue(),
        missing_stickers.to_string().yellow(),
    );

    let mut output = String::new();
    output.push_str("Needed Stickers Report\n");
    output.push_str("======================\n");

    let mut pvc_entries: Vec<(Dimensions, u64)> = Vec::new();
    let mut paper_entries: Vec<(Dimensions, u64)> = Vec::new();

    for ((mat, dims), count) in counts {
        match mat.as_str() {
            "PAPER" => paper_entries.push((dims, count)),
            "PVC" => pvc_entries.push((dims, count)),
            _ => panic!("Unknown material group: {}", mat),
        }
    }

    pvc_entries.sort_by_key(|(dims, _)| dims.to_string());
    paper_entries.sort_by_key(|(dims, _)| dims.to_string());

    output.push_str("\nPVC\n");
    for (dims, count) in pvc_entries {
        output.push_str(&format!("{dims} - {count}\n"));
    }

    output.push_str("\nPAPER\n");
    for (dims, count) in paper_entries {
        output.push_str(&format!("{dims} - {count}\n"));
    }

    let mut file = File::create("poruchka_stickeri.txt")?;
    file.write_all(output.as_bytes())?;

    Ok(())
}
