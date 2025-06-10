use order_processor::{
    parser::{self, ParseStickerError},
    sticker::{Color, Material, Sticker},
};
use std::str::FromStr;

#[test]
fn test_7099() {
    let s = Sticker::from_str("7099_LRS_НЕЖЕН САПУН ОБОГАТЕН С МАСЛА_60X40_PVC_R (2)").unwrap();
    assert_eq!(s.code, 7099);
    assert_eq!(s.dimensions, vec!["60x40"]);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_7129() {
    let s = Sticker::from_str("7129_LRS_НЕЖЕН ИЗМИВАЩ ГЕЛ БЕБЕ 80X55_PVC_R").unwrap();
    assert_eq!(s.code, 7129);
    assert_eq!(s.dimensions, vec!["80x55"]);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_7136() {
    let s = Sticker::from_str("7136_LRS_ХИДРАТИРАЩ КРЕМ БЕБЕ_58X43_PVC_R").unwrap();
    assert_eq!(s.code, 7136);
    assert_eq!(s.dimensions, vec!["58x43"]);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_7143() {
    let s = Sticker::from_str("7143_LRS_ПОЧИСТВАЩО МЛЯКО БЕБЕ_60X110_PVC_R").unwrap();
    assert_eq!(s.code, 7143);
    assert_eq!(s.dimensions, vec!["60x110"]);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_7303() {
    let s =
        Sticker::from_str("7303_LRS_SOS ВЪЗСТАНОВЯВАЩ БАЛСАМ БЕБЕ 50X30_PVC_R_TEMP_SIZE").unwrap();
    assert_eq!(s.dimensions, vec!["50x30"]);
    assert_eq!(s.code, 7303);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_7396() {
    let s = Sticker::from_str("7396_LRS_НЕЖЕН ИЗМИВАЩ ГЕЛ БЕБЕ ПЪЛНИТЕЛ 80X55_PVC_R").unwrap();
    assert_eq!(s.dimensions, vec!["80x55"]);
    assert_eq!(s.code, 7396);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_7624() {
    let s = Sticker::from_str("7624_LRS_ПОЧИСТВАЩА ВОДА БЕБЕ_60X110_PVC_R").unwrap();
    assert_eq!(s.dimensions, vec!["60x110"]);
    assert_eq!(s.code, 7624);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_7631() {
    let s = Sticker::from_str("7631_LRS_БЕБЕШКА ПАСТА ЗА ЗЪБИ_50X30_PVC_R").unwrap();
    assert_eq!(s.dimensions, vec!["50x30"]);
    assert_eq!(s.code, 7631);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_7860() {
    let s = Sticker::from_str("7860_LR_BOX BEBE_80X55_PVC_R").unwrap();
    assert_eq!(s.dimensions, vec!["80x55"]);
    assert_eq!(s.code, 7860);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_205475() {
    let s = Sticker::from_str(
        "205475_RF VITALFAN PROGR SINGLE 30K_58X75_36X73_PAPER GREEN_DVOEN STIKER_OK",
    )
    .unwrap();
    assert_eq!(s.code, 205475);
    assert_eq!(s.dimensions, vec!["58x75", "36x73"]);
    assert_eq!(s.material, Material::Paper);
    assert_eq!(s.text_color, Color::Green);
}

#[test]
fn test_234191() {
    let s = Sticker::from_str("234191_AV CLEAN GEL TUBE 200ML_50X50_PVC_R_OK_PF").unwrap();
    assert_eq!(s.code, 234191);
    assert_eq!(s.dimensions, vec!["50x50"]);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_235354() {
    let s = Sticker::from_str("235354_RF STYLE FIX GEL 150ML_45X101_PVC_R_SLV_OK_PF").unwrap();
    assert_eq!(s.code, 235354);
    assert_eq!(s.dimensions, vec!["45x101"]);
    assert_eq!(s.material, Material::PVCRSLV);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_237355() {
    let s = Sticker::from_str("237355_AD DERMALIB CICA CR REP 50ML_100X40_PVC_OK_PF").unwrap();
    assert_eq!(s.code, 237355);
    assert_eq!(s.dimensions, vec!["100x40"]);
    assert_eq!(s.material, Material::PVC);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_238309() {
    let s = Sticker::from_str("238309_AV TOL LOT 200ML_40X60_PVC_OK_PF").unwrap();
    assert_eq!(s.code, 238309);
    assert_eq!(s.dimensions, vec!["40x60"]);
    assert_eq!(s.material, Material::PVC);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_239680() {
    let s =
        Sticker::from_str("239680_AD EPITHELIALE AH MASSAGE OIL 100ML_45X101_PVC_OK_PF").unwrap();
    assert_eq!(s.dimensions, vec!["45x101"]);
    assert_eq!(s.code, 239680);
    assert_eq!(s.material, Material::PVC);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_240198() {
    let s =
        Sticker::from_str("240198_KL MASQUE REPA CAPUACY 3IN1 150ML_60X40_PVC_R_OK_PF").unwrap();
    assert_eq!(s.dimensions, vec!["60x40"]);
    assert_eq!(s.code, 240198);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_241438() {
    let s = Sticker::from_str("241438_DU KERTYOL PSO SHP 125ML_50X100_PAPER BLUE_OK_PF").unwrap();
    assert_eq!(s.code, 241438);
    assert_eq!(s.dimensions, vec!["50x100"]);
    assert_eq!(s.material, Material::Paper);
    assert_eq!(s.text_color, Color::Blue);
}

#[test]
fn test_247109() {
    let s = Sticker::from_str("247109_KL SHP GALANGA 200ML_60X40_PVC_R_OK_PF").unwrap();
    assert_eq!(s.dimensions, vec!["60x40"]);
    assert_eq!(s.code, 247109);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_253831() {
    let s = Sticker::from_str("253831_AD BIOLOGY HIALU SER 3IN1 30ML_40X90_PAPER(GR)BLK_OK_PF")
        .unwrap();
    assert_eq!(s.dimensions, vec!["40x90"]);
    assert_eq!(s.code, 253831);
    assert_eq!(s.material, Material::PaperGR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_259064() {
    let s = Sticker::from_str("259064_KL SHP PIVOINE 200ML_60X40_PVC_R_OK").unwrap();
    assert_eq!(s.code, 259064);
    assert_eq!(s.dimensions, vec!["60x40"]);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_259839() {
    let s = Sticker::from_str("259839_RF ABSOLU KERATIN CR 100ML_40X100_PAPER BLK_PF").unwrap();
    assert_eq!(s.code, 259839);
    assert_eq!(s.dimensions, vec!["40x100"]);
    assert_eq!(s.material, Material::Paper);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_261776() {
    let s = Sticker::from_str("261776_AV HYALURON ACTIVE B3 REFILL 50ML_50X22_PVC_OK_PF").unwrap();
    assert_eq!(s.code, 261776);
    assert_eq!(s.dimensions, vec!["50x22"]);
    assert_eq!(s.material, Material::PVC);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_261783() {
    let s = Sticker::from_str("261783_AV VITAMIN ACTIV CG SER 30ML_40X45_PVC_R_OK_PF").unwrap();
    assert_eq!(s.code, 261783);
    assert_eq!(s.dimensions, vec!["40x45"]);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_261788() {
    let s = Sticker::from_str("261788_AV VITAMIN ACTIV CG SERUM_27X40_PVC_PF").unwrap();
    assert_eq!(s.code, 261788);
    assert_eq!(s.dimensions, vec!["27x40"]);
    assert_eq!(s.material, Material::PVC);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_263673_gel() {
    let s =
        Sticker::from_str("263673_ELGY CLINIC SENSILEAVE GEL - TUBE_40X20_PVC_R_OK_PF").unwrap();
    assert_eq!(s.code, 263673);
    assert_eq!(s.dimensions, vec!["40x20"]);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_263673_box() {
    let s = Sticker::from_str("263673_ELGY CLINIC SENSILEAVE GEL - BOX_121X27_PAPER BLUE_OK_PF")
        .unwrap();
    assert_eq!(s.dimensions, vec!["121x27"]);
    assert_eq!(s.code, 263673);
    assert_eq!(s.material, Material::Paper);
    assert_eq!(s.text_color, Color::Blue);
}

#[test]
fn test_267995() {
    let s = Sticker::from_str("267995_AV SOL SPRAY 50 200ML_50X50_PVC_R_OK").unwrap();
    assert_eq!(s.code, 267995);
    assert_eq!(s.dimensions, vec!["50x50"]);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_270402() {
    let s = Sticker::from_str("270402_RF VOLUMEA SHP 200ML_100X40_PAPER GREEN").unwrap();
    assert_eq!(s.code, 270402);
    assert_eq!(s.dimensions, vec!["100x40"]);
    assert_eq!(s.material, Material::Paper);
    assert_eq!(s.text_color, Color::Green);
}

#[test]
fn test_270983() {
    let s = Sticker::from_str("270983_KL SHP MENTHE 200ML_60X40_PVC_R").unwrap();
    assert_eq!(s.code, 270983);
    assert_eq!(s.dimensions, vec!["60x40"]);
    assert_eq!(s.material, Material::PVCR);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_273656() {
    let s = Sticker::from_str("273656_DU MELAS FL INVISIBLE 30ML_40X60_PVC").unwrap();
    assert_eq!(s.code, 273656);
    assert_eq!(s.dimensions, vec!["40x60"]);
    assert_eq!(s.material, Material::PVC);
    assert_eq!(s.text_color, Color::Black);
}

#[test]
fn test_multiple_dimensions() {
    let stickers = parser::parse_names(
        vec![
            "205475_RF VITALFAN PROGR SINGLE 30K_58X75_36X73_10X320 130X450________PAPER GREEN_DVOEN STIKER_OK"
                .to_owned(),
        ]
        .as_slice(),
    );
    for (i, s) in stickers.into_iter().enumerate() {
        let s = s.unwrap();
        assert_eq!(s.code, 205475);
        println!("{s:?}");

        match i {
            0 => assert_eq!(s.dimensions, vec!["58x75"]),
            1 => assert_eq!(s.dimensions, vec!["36x73"]),
            2 => assert_eq!(s.dimensions, vec!["10x320"]),
            3 => assert_eq!(s.dimensions, vec!["130x450"]),
            _ => unreachable!(),
        }

        assert_eq!(s.material, Material::Paper);
        assert_eq!(s.text_color, Color::Green);
    }
}

#[test]
fn test_infer_with_one_typo() {
    let existing =
        Sticker::from_str("234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGHT_50X50_PVC_R_OK_PF")
            .unwrap();
    let error = ParseStickerError::MissingCode(
        "AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_60X60_PVC_R_OK_PF".into(),
    );

    let result =
        parser::try_infering_code_by_description_similiarity_measure(error, &vec![existing], 0.93);
    assert!(result.is_ok());
    assert_eq!(result.unwrap()[0].code, 234191);
}

#[test]
fn test_infer_with_missing_character() {
    let existing =
        Sticker::from_str("234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF")
            .unwrap();
    let error = ParseStickerError::MissingCode(
        "AV CLEAN GEL TUBE 200ML + RANDOM LENGT_70X40_PVC_R_OK_PF".into(),
    );

    let result =
        parser::try_infering_code_by_description_similiarity_measure(error, &vec![existing], 0.93);
    assert!(result.is_ok());
    assert_eq!(result.unwrap()[0].code, 234191);
}

#[test]
fn test_infer_with_character_swap() {
    let existing =
        Sticker::from_str("234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF")
            .unwrap();
    let error = ParseStickerError::MissingCode(
        "AV CLEAN GEL TUBE 200ML + RANDOM LENTGH_50X50_PVC_R_OK_PF".into(),
    );

    let result =
        parser::try_infering_code_by_description_similiarity_measure(error, &vec![existing], 0.93);
    assert!(result.is_ok());
    assert_eq!(result.unwrap()[0].code, 234191);
}

#[test]
fn test_fail_with_three_differences() {
    let existing =
        Sticker::from_str("234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF")
            .unwrap();
    let error = ParseStickerError::MissingCode(
        "AV CLEAN GEL TUBE 200ML + RANDO LENTG_50X50_PVC_R_OK_PF".into(),
    );

    let result =
        parser::try_infering_code_by_description_similiarity_measure(error, &vec![existing], 0.93);
    assert!(result.is_err());
}

#[test]
fn test_fail_with_unrelated_description() {
    let existing =
        Sticker::from_str("234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF")
            .unwrap();
    let error = ParseStickerError::MissingCode("FACE WASH FOAM FOR MEN_50X50_PVC_R_OK_PF".into());

    let result =
        parser::try_infering_code_by_description_similiarity_measure(error, &vec![existing], 0.93);
    assert!(result.is_err());
}
