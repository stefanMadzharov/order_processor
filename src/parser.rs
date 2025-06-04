use crate::order::Order;
use rayon::prelude::*;
use regex::Regex;

pub fn parse_names(names: &[&str]) -> Vec<Order> {
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
