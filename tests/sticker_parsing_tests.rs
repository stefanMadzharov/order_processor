use order_processor::{
    parser,
    structs::{
        color::Color, dimensions::Dimensions, material::Material,
        parse_stcker_error::ParseStickerError, sticker::Sticker,
    },
};
use regex::Regex;

#[cfg(test)]
fn parse_stickers(name: &str) -> Vec<Sticker> {
    let code_re = Regex::new(r"^(\d{3,})").unwrap();
    let dimensions_re = Regex::new(r"\d+[ХX]\d+").unwrap();
    let material_re = Regex::new(
        r"(?i)PAPER(?:[_ (.]*GR[_ ).])?|LEAFLET|PP|PVC(?:[_ ().]*R(?:[_ ().]*SLV)?)?|SLV",
    )
    .unwrap();
    let color_re = Regex::new(r"(?i)BLK|BLACK|RED|GREEN|BLUE").unwrap();
    Sticker::parse_stickers(name, &code_re, &dimensions_re, &material_re, &color_re).unwrap()
}

#[test]
fn test_7099() {
    let s = parse_stickers("7099_LRS_НЕЖЕН САПУН ОБОГАТЕН С МАСЛА_60X40_PVC_R (2)");
    assert_eq!(s[0].code, 7099);
    assert_eq!(s[0].dimensions, "60x40".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_7129() {
    let s = parse_stickers("7129_LRS_НЕЖЕН ИЗМИВАЩ ГЕЛ БЕБЕ 80X55_PVC_R");
    assert_eq!(s[0].code, 7129);
    assert_eq!(s[0].dimensions, "80x55".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_7136() {
    let s = parse_stickers("7136_LRS_ХИДРАТИРАЩ КРЕМ БЕБЕ_58X43_PVC_R");
    assert_eq!(s[0].code, 7136);
    assert_eq!(s[0].dimensions, "58x43".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_7143() {
    let s = parse_stickers("7143_LRS_ПОЧИСТВАЩО МЛЯКО БЕБЕ_60X110_PVC_R");
    assert_eq!(s[0].code, 7143);
    assert_eq!(s[0].dimensions, "60x110".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_7303() {
    let s = parse_stickers("7303_LRS_SOS ВЪЗСТАНОВЯВАЩ БАЛСАМ БЕБЕ 50X30_PVC_R_TEMP_SIZE");
    assert_eq!(s[0].dimensions, "50x30".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].code, 7303);
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_7396() {
    let s = parse_stickers("7396_LRS_НЕЖЕН ИЗМИВАЩ ГЕЛ БЕБЕ ПЪЛНИТЕЛ 80X55_PVC_R");
    assert_eq!(s[0].dimensions, "80x55".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].code, 7396);
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_7624() {
    let s = parse_stickers("7624_LRS_ПОЧИСТВАЩА ВОДА БЕБЕ_60X110_PVC_R");
    assert_eq!(s[0].dimensions, "60x110".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].code, 7624);
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_7631() {
    let s = parse_stickers("7631_LRS_БЕБЕШКА ПАСТА ЗА ЗЪБИ_50X30_PVC_R");
    assert_eq!(s[0].dimensions, "50x30".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].code, 7631);
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_7860() {
    let s = parse_stickers("7860_LR_BOX BEBE_80X55_PVC_R");
    assert_eq!(s[0].dimensions, "80x55".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].code, 7860);
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_205475() {
    let s = parse_stickers(
        "205475_RF VITALFAN PROGR SINGLE 30K_58X75_36X73_PAPER GREEN_DVOEN STIKER_OK",
    );
    assert_eq!(s[0].code, 205475);
    assert_eq!(s[0].dimensions, "58x75".parse::<Dimensions>().unwrap(),);
    assert_eq!(s[1].dimensions, "36x73".parse::<Dimensions>().unwrap(),);
    assert_eq!(s[0].material, Material::Paper);
    assert_eq!(s[0].text_color, Color::Green);
}

#[test]
fn test_234191() {
    let s = parse_stickers("234191_AV CLEAN GEL TUBE 200ML_50X50_PVC_R_OK_PF");
    assert_eq!(s[0].code, 234191);
    assert_eq!(s[0].dimensions, "50x50".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_235354() {
    let s = parse_stickers("235354_RF STYLE FIX GEL 150ML_45X101_PVC_R_SLV_OK_PF");
    assert_eq!(s[0].code, 235354);
    assert_eq!(s[0].dimensions, "45x101".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVCRSLV);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_237355() {
    let s = parse_stickers("237355_AD DERMALIB CICA CR REP 50ML_100X40_PVC_OK_PF");
    assert_eq!(s[0].code, 237355);
    assert_eq!(s[0].dimensions, "100x40".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVC);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_238309() {
    let s = parse_stickers("238309_AV TOL LOT 200ML_40X60_PVC_OK_PF");
    assert_eq!(s[0].code, 238309);
    assert_eq!(s[0].dimensions, "40x60".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVC);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_239680() {
    let s = parse_stickers("239680_AD EPITHELIALE AH MASSAGE OIL 100ML_45X101_PVC_OK_PF");
    assert_eq!(s[0].dimensions, "45x101".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].code, 239680);
    assert_eq!(s[0].material, Material::PVC);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_240198() {
    let s = parse_stickers("240198_KL MASQUE REPA CAPUACY 3IN1 150ML_60X40_PVC_R_OK_PF");
    assert_eq!(s[0].dimensions, "60x40".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].code, 240198);
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_241438() {
    let s = parse_stickers("241438_DU KERTYOL PSO SHP 125ML_50X100_PAPER BLUE_OK_PF");
    assert_eq!(s[0].code, 241438);
    assert_eq!(s[0].dimensions, "50x100".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::Paper);
    assert_eq!(s[0].text_color, Color::Blue);
}

#[test]
fn test_247109() {
    let s = parse_stickers("247109_KL SHP GALANGA 200ML_60X40_PVC_R_OK_PF");
    assert_eq!(s[0].dimensions, "60x40".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].code, 247109);
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_253831() {
    let s = parse_stickers("253831_AD BIOLOGY HIALU SER 3IN1 30ML_40X90_PAPER(GR)BLK_OK_PF");
    assert_eq!(s[0].dimensions, "40x90".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].code, 253831);
    assert_eq!(s[0].material, Material::PaperGR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_259064() {
    let s = parse_stickers("259064_KL SHP PIVOINE 200ML_60X40_PVC_R_OK");
    assert_eq!(s[0].code, 259064);
    assert_eq!(s[0].dimensions, "60x40".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_259839() {
    let s = parse_stickers("259839_RF ABSOLU KERATIN CR 100ML_40X100_PAPER BLK_PF");
    assert_eq!(s[0].code, 259839);
    assert_eq!(s[0].dimensions, "40x100".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::Paper);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_261776() {
    let s = parse_stickers("261776_AV HYALURON ACTIVE B3 REFILL 50ML_50X22_PVC_OK_PF");
    assert_eq!(s[0].code, 261776);
    assert_eq!(s[0].dimensions, "50x22".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVC);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_261783() {
    let s = parse_stickers("261783_AV VITAMIN ACTIV CG SER 30ML_40X45_PVC_R_OK_PF");
    assert_eq!(s[0].code, 261783);
    assert_eq!(s[0].dimensions, "40x45".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_261788() {
    let s = parse_stickers("261788_AV VITAMIN ACTIV CG SERUM_27X40_PVC_PF");
    assert_eq!(s[0].code, 261788);
    assert_eq!(s[0].dimensions, "27x40".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVC);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_263673_gel() {
    let s = parse_stickers("263673_ELGY CLINIC SENSILEAVE GEL - TUBE_40X20_PVC_R_OK_PF");
    assert_eq!(s[0].code, 263673);
    assert_eq!(s[0].dimensions, "40x20".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_263673_box() {
    let s = parse_stickers("263673_ELGY CLINIC SENSILEAVE GEL - BOX_121X27_PAPER BLUE_OK_PF");
    assert_eq!(s[0].dimensions, "121x27".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].code, 263673);
    assert_eq!(s[0].material, Material::Paper);
    assert_eq!(s[0].text_color, Color::Blue);
}

#[test]
fn test_267995() {
    let s = parse_stickers("267995_AV SOL SPRAY 50 200ML_50X50_PVC_R_OK");
    assert_eq!(s[0].code, 267995);
    assert_eq!(s[0].dimensions, "50x50".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_270402() {
    let s = parse_stickers("270402_RF VOLUMEA SHP 200ML_100X40_PAPER GREEN");
    assert_eq!(s[0].code, 270402);
    assert_eq!(s[0].dimensions, "100x40".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::Paper);
    assert_eq!(s[0].text_color, Color::Green);
}

#[test]
fn test_270983() {
    let s = parse_stickers("270983_KL SHP MENTHE 200ML_60X40_PVC_R");
    assert_eq!(s[0].code, 270983);
    assert_eq!(s[0].dimensions, "60x40".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVCR);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_273656() {
    let s = parse_stickers("273656_DU MELAS FL INVISIBLE 30ML_40X60_PVC");
    assert_eq!(s[0].code, 273656);
    assert_eq!(s[0].dimensions, "40x60".parse::<Dimensions>().unwrap());
    assert_eq!(s[0].material, Material::PVC);
    assert_eq!(s[0].text_color, Color::Black);
}

#[test]
fn test_multiple_dimensions() {
    let stickers: Vec<Sticker> = parser::parse_names(
        vec![
            "205475_RF VITALFAN PROGR SINGLE 30K_58X75_36X73_10X320 130X450________PAPER GREEN_DVOEN STIKER_OK"
                .to_owned(),
        ]
        .as_slice(),
    ).into_iter().flat_map(|stickers| stickers.unwrap()).collect();

    for (i, s) in stickers.into_iter().enumerate() {
        assert_eq!(s.code, 205475);
        println!("{s:?}");

        match i {
            0 => assert_eq!(s.dimensions, "58x75".parse::<Dimensions>().unwrap()),
            1 => assert_eq!(s.dimensions, "36x73".parse::<Dimensions>().unwrap()),
            2 => assert_eq!(s.dimensions, "10x320".parse::<Dimensions>().unwrap()),
            3 => assert_eq!(s.dimensions, "130x450".parse::<Dimensions>().unwrap()),
            _ => unreachable!(),
        }

        assert_eq!(s.material, Material::Paper);
        assert_eq!(s.text_color, Color::Green);
    }
}

#[test]
fn test_infer_with_one_typo() {
    let existing =
        parse_stickers("234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGHT_50X50_PVC_R_OK_PF");
    let error = ParseStickerError::MissingCode(
        "AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_60X60_PVC_R_OK_PF".into(),
    );

    let result =
        parser::try_infering_code_by_description_similiarity_measure(error, &existing, 0.93);
    assert!(result.is_ok());
    assert_eq!(result.unwrap()[0].code, 234191);
}

#[test]
fn test_infer_with_missing_character() {
    let existing =
        parse_stickers("234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF");
    let error = ParseStickerError::MissingCode(
        "AV CLEAN GEL TUBE 200ML + RANDOM LENGT_70X40_PVC_R_OK_PF".into(),
    );

    let result =
        parser::try_infering_code_by_description_similiarity_measure(error, &existing, 0.93);
    assert!(result.is_ok());
    assert_eq!(result.unwrap()[0].code, 234191);
}

#[test]
fn test_infer_with_character_swap() {
    let existing =
        parse_stickers("234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF");
    let error = ParseStickerError::MissingCode(
        "AV CLEAN GEL TUBE 200ML + RANDOM LENTGH_50X50_PVC_R_OK_PF".into(),
    );

    let result =
        parser::try_infering_code_by_description_similiarity_measure(error, &existing, 0.93);
    assert!(result.is_ok());
    assert_eq!(result.unwrap()[0].code, 234191);
}

#[test]
fn test_fail_with_three_differences() {
    let existing =
        parse_stickers("234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF");
    let error = ParseStickerError::MissingCode(
        "AV CLEAN GEL TUBE 200ML + RANDO LENTG_50X50_PVC_R_OK_PF".into(),
    );

    let result =
        parser::try_infering_code_by_description_similiarity_measure(error, &existing, 0.93);
    assert!(result.is_err());
}

#[test]
fn test_fail_with_unrelated_description() {
    let existing =
        parse_stickers("234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF");
    let error = ParseStickerError::MissingCode("FACE WASH FOAM FOR MEN_50X50_PVC_R_OK_PF".into());

    let result =
        parser::try_infering_code_by_description_similiarity_measure(error, &existing, 0.93);
    assert!(result.is_err());
}

#[test]
fn test_double_sticker_dimensions() {
    let test_cases = vec![
        (
            "209989_DU DENSIAGE DOBAVKA 30TABL_30X101_45X59_PVC_R_SLV",
            vec!["30x101", "45x59"],
        ),
        (
            "207073 _RF VITALFAN SOL 30K_36X73_ 58X75_PAPER GREEN_DV_ST",
            vec!["36x73", "58x75"],
        ),
        (
            "254285_DU ANACAPS EXPERT 30CAPS_1ST 68X40_2ST 68X40_PAPER_BLUE_OK_PF",
            vec!["68x40", "68x40"],
        ),
        (
            "220763_RF VITALFAN PROGR SINGLE 30K_58X75_36X73_PAPER GREEN_DVOEN STIKER",
            vec!["58x75", "36x73"],
        ),
        (
            "211949_EL SD GELULE MINCEUR TABLETES_45X102_45X40_PVC_OK",
            vec!["45x40"],
        ),
        (
            "207687_AD HYDRALBA UV RICH CR 40ML_70X40_PVC&40X45_PVC R_OK",
            vec!["70x40", "40x45"],
        ),
        (
            "254310_DU ANACAPS REACTIV GEL 30U_40X68_42X50_PAPER BLUE_DB ST_PF",
            vec!["40x68", "42x50"],
        ),
        (
            "211949_EL SD GELULE MINCEUR TABLETES_45X102_45X40_PVC",
            vec!["45x40"],
        ),
        (
            "538752_AD DERMALIBUR CR BAR 100ML_40X100&40X45_DVA STIKERA_PVC_OK",
            vec!["40x100", "40x45"],
        ),
        (
            "254285_DU ANACAPS EXPERT 30CAPS_1ST 68X40_2ST 68X40_PAPER_BLUE_PF_2",
            vec!["68x40", "68x40"],
        ),
        (
            "515134_AD EXOMEGA DEFI 200ML_40X100_40X45_ДВОЕН СТИКЕР_PVC_OK",
            vec!["40x100", "40x45"],
        ),
        (
            "207586_AD DERMAL CR REP 100ML_40X100_22X106_OBEDINEN STIKER_PVC_OK",
            vec!["40x100", "22x106"],
        ),
        (
            "207686_AD HYDRALBA UV LEGERE TUBE 40ML_40X70_PVC_&_40X45_PVC_R_OK",
            vec!["40x70", "40x45"],
        ),
        (
            "209989_DU DENSIAGE DOBAVKA 30TABL_30X101_45X59_PVC_R_OK_SLV",
            vec!["30x101", "45x59"],
        ),
        (
            "207686_AD HYDRALBA UV LEGERE TUBE 40ML_40X70_PVC_&_40X45_PVC_R",
            vec!["40x70", "40x45"],
        ),
        (
            "205475_RF VITALFAN PROGR SINGLE 30K_58X75_40X68_PAPER GREEN_DVOEN STIKER",
            vec!["58x75", "40x68"],
        ),
        (
            "207873_BOX SPRAY ETA COLLECT_3X50ML_40X100_40X27_DV.ST.PVC_R_OK_PF",
            vec!["40x100", "40x27"],
        ),
    ];

    for (filename, expected_sizes) in test_cases {
        let stickers = parse_stickers(filename);
        for (i, expected) in expected_sizes.iter().enumerate() {
            let expected_dim = expected.parse::<Dimensions>().unwrap();
            assert_eq!(
                stickers[i].dimensions, expected_dim,
                "Mismatch in dimensions for file: {}",
                filename
            );
        }
    }
}
