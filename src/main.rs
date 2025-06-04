use order_processor::parser;

fn main() {
    let file_names = vec![
        "297_CAU Resveratrol Lift Instant Firming Serum - 30 mL_30x70_PVC_R_OK",
        "205043_AV ETA Collect 50ml_40x45_PVC_R_OK_PF",
        "205475_RF VITALFAN PROGR Single 30k_58x75_36x73_paper green_dvoen stiker_OK",
        "205671_AV COUV STICK KORAL Spf30 4gr_25x80_PVC_OK_PF",
        "205813_KL BBC GD FIGUIER 75ml_40x20_PVC_R_OK",
    ];

    let orders = parser::parse_names(&file_names);

    for order in orders {
        println!("{order}");
    }

    // if let Err(e) = write_to_excel(&orders) {
    //     eprintln!("Failed to write Excel: {}", e);
    // }
}
