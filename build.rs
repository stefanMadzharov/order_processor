use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let input_path = "./dimensions.txt";

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let output_path = Path::new(&out_dir).join("generated_dimensions.rs");

    let file = File::open(input_path).expect("Cannot open dimensions.txt");
    let reader = BufReader::new(file);

    let mut output = String::from("pub const OFFICIAL_DIMENSIONS: &[Dimensions] = &[\n");

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if let Some((w, h)) = line.trim().split_once('x') {
            output.push_str(&format!(
                "    Dimensions {{ width: {}, height: {} }},\n",
                w.trim(),
                h.trim()
            ));
        }
    }

    output.push_str("];\n");

    fs::write(output_path, output).expect("Could not write generated_dimensions.rs");
}
