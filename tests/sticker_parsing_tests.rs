#[cfg(test)]
mod tests {
    use order_processor::{
        parser,
        structs::{
            color::Color, dimensions::Dimensions, material::Material,
            parse_stcker_error::ParseStickerError, sticker::Sticker,
        },
    };

    #[test]
    fn test_7099() {
        let s = Sticker::parse_stickers("7099_LRS_НЕЖЕН САПУН ОБОГАТЕН С МАСЛА_60X40_PVC_R (2)")
            .unwrap();
        assert_eq!(s[0].code, 7099);
        assert_eq!(s[0].dimensions, "60x40".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_7129() {
        let s = Sticker::parse_stickers("7129_LRS_НЕЖЕН ИЗМИВАЩ ГЕЛ БЕБЕ 80X55_PVC_R").unwrap();
        assert_eq!(s[0].code, 7129);
        assert_eq!(s[0].dimensions, "80x55".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_7136() {
        let s = Sticker::parse_stickers("7136_LRS_ХИДРАТИРАЩ КРЕМ БЕБЕ_58X43_PVC_R").unwrap();
        assert_eq!(s[0].code, 7136);
        assert_eq!(s[0].dimensions, "58x43".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_7143() {
        let s = Sticker::parse_stickers("7143_LRS_ПОЧИСТВАЩО МЛЯКО БЕБЕ_60X110_PVC_R").unwrap();
        assert_eq!(s[0].code, 7143);
        assert_eq!(s[0].dimensions, "60x110".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_7303() {
        let s =
            Sticker::parse_stickers("7303_LRS_SOS ВЪЗСТАНОВЯВАЩ БАЛСАМ БЕБЕ 50X30_PVC_R_TEMP_SIZE")
                .unwrap();
        assert_eq!(s[0].dimensions, "50x30".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].code, 7303);
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_7396() {
        let s = Sticker::parse_stickers("7396_LRS_НЕЖЕН ИЗМИВАЩ ГЕЛ БЕБЕ ПЪЛНИТЕЛ 80X55_PVC_R")
            .unwrap();
        assert_eq!(s[0].dimensions, "80x55".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].code, 7396);
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_7624() {
        let s = Sticker::parse_stickers("7624_LRS_ПОЧИСТВАЩА ВОДА БЕБЕ_60X110_PVC_R").unwrap();
        assert_eq!(s[0].dimensions, "60x110".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].code, 7624);
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_7631() {
        let s = Sticker::parse_stickers("7631_LRS_БЕБЕШКА ПАСТА ЗА ЗЪБИ_50X30_PVC_R").unwrap();
        assert_eq!(s[0].dimensions, "50x30".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].code, 7631);
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_7860() {
        let s = Sticker::parse_stickers("7860_LR_BOX BEBE_80X55_PVC_R").unwrap();
        assert_eq!(s[0].dimensions, "80x55".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].code, 7860);
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_205475() {
        let s = Sticker::parse_stickers(
            "205475_RF VITALFAN PROGR SINGLE 30K_58X75_36X73_PAPER GREEN_DVOEN STIKER_OK",
        )
        .unwrap();
        assert_eq!(s[0].code, 205475);
        assert_eq!(s[0].dimensions, "58x75".parse::<Dimensions>().unwrap(),);
        assert_eq!(s[1].dimensions, "36x73".parse::<Dimensions>().unwrap(),);
        assert_eq!(s[0].material, Material::Paper);
        assert_eq!(s[0].text_color, Color::Green);
    }

    #[test]
    fn test_234191() {
        let s =
            Sticker::parse_stickers("234191_AV CLEAN GEL TUBE 200ML_50X50_PVC_R_OK_PF").unwrap();
        assert_eq!(s[0].code, 234191);
        assert_eq!(s[0].dimensions, "50x50".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_235354() {
        let s = Sticker::parse_stickers("235354_RF STYLE FIX GEL 150ML_45X101_PVC_R_SLV_OK_PF")
            .unwrap();
        assert_eq!(s[0].code, 235354);
        assert_eq!(s[0].dimensions, "45x101".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCRSLV);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_237355() {
        let s = Sticker::parse_stickers("237355_AD DERMALIB CICA CR REP 50ML_100X40_PVC_OK_PF")
            .unwrap();
        assert_eq!(s[0].code, 237355);
        assert_eq!(s[0].dimensions, "100x40".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVC);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_238309() {
        let s = Sticker::parse_stickers("238309_AV TOL LOT 200ML_40X60_PVC_OK_PF").unwrap();
        assert_eq!(s[0].code, 238309);
        assert_eq!(s[0].dimensions, "40x60".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVC);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_239680() {
        let s =
            Sticker::parse_stickers("239680_AD EPITHELIALE AH MASSAGE OIL 100ML_45X101_PVC_OK_PF")
                .unwrap();
        assert_eq!(s[0].dimensions, "45x101".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].code, 239680);
        assert_eq!(s[0].material, Material::PVC);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_240198() {
        let s =
            Sticker::parse_stickers("240198_KL MASQUE REPA CAPUACY 3IN1 150ML_60X40_PVC_R_OK_PF")
                .unwrap();
        assert_eq!(s[0].dimensions, "60x40".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].code, 240198);
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_241438() {
        let s = Sticker::parse_stickers("241438_DU KERTYOL PSO SHP 125ML_50X100_PAPER BLUE_OK_PF")
            .unwrap();
        assert_eq!(s[0].code, 241438);
        assert_eq!(s[0].dimensions, "50x100".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::Paper);
        assert_eq!(s[0].text_color, Color::Blue);
    }

    #[test]
    fn test_247109() {
        let s = Sticker::parse_stickers("247109_KL SHP GALANGA 200ML_60X40_PVC_R_OK_PF").unwrap();
        assert_eq!(s[0].dimensions, "60x40".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].code, 247109);
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_253831() {
        let s = Sticker::parse_stickers(
            "253831_AD BIOLOGY HIALU SER 3IN1 30ML_40X90_PAPER(GR)BLK_OK_PF",
        )
        .unwrap();
        assert_eq!(s[0].dimensions, "40x90".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].code, 253831);
        assert_eq!(s[0].material, Material::PaperGR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_259064() {
        let s = Sticker::parse_stickers("259064_KL SHP PIVOINE 200ML_60X40_PVC_R_OK").unwrap();
        assert_eq!(s[0].code, 259064);
        assert_eq!(s[0].dimensions, "60x40".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_259839() {
        let s = Sticker::parse_stickers("259839_RF ABSOLU KERATIN CR 100ML_40X100_PAPER BLK_PF")
            .unwrap();
        assert_eq!(s[0].code, 259839);
        assert_eq!(s[0].dimensions, "40x100".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::Paper);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_261776() {
        let s = Sticker::parse_stickers("261776_AV HYALURON ACTIVE B3 REFILL 50ML_50X22_PVC_OK_PF")
            .unwrap();
        assert_eq!(s[0].code, 261776);
        assert_eq!(s[0].dimensions, "50x22".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVC);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_261783() {
        let s = Sticker::parse_stickers("261783_AV VITAMIN ACTIV CG SER 30ML_40X45_PVC_R_OK_PF")
            .unwrap();
        assert_eq!(s[0].code, 261783);
        assert_eq!(s[0].dimensions, "40x45".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_261788() {
        let s = Sticker::parse_stickers("261788_AV VITAMIN ACTIV CG SERUM_27X40_PVC_PF").unwrap();
        assert_eq!(s[0].code, 261788);
        assert_eq!(s[0].dimensions, "27x40".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVC);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_263673_gel() {
        let s =
            Sticker::parse_stickers("263673_ELGY CLINIC SENSILEAVE GEL - TUBE_40X20_PVC_R_OK_PF")
                .unwrap();
        assert_eq!(s[0].code, 263673);
        assert_eq!(s[0].dimensions, "40x20".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_263673_box() {
        let s = Sticker::parse_stickers(
            "263673_ELGY CLINIC SENSILEAVE GEL - BOX_121X27_PAPER BLUE_OK_PF",
        )
        .unwrap();
        assert_eq!(s[0].dimensions, "121x27".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].code, 263673);
        assert_eq!(s[0].material, Material::Paper);
        assert_eq!(s[0].text_color, Color::Blue);
    }

    #[test]
    fn test_267995() {
        let s = Sticker::parse_stickers("267995_AV SOL SPRAY 50 200ML_50X50_PVC_R_OK").unwrap();
        assert_eq!(s[0].code, 267995);
        assert_eq!(s[0].dimensions, "50x50".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_270402() {
        let s = Sticker::parse_stickers("270402_RF VOLUMEA SHP 200ML_100X40_PAPER GREEN").unwrap();
        assert_eq!(s[0].code, 270402);
        assert_eq!(s[0].dimensions, "100x40".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::Paper);
        assert_eq!(s[0].text_color, Color::Green);
    }

    #[test]
    fn test_270983() {
        let s = Sticker::parse_stickers("270983_KL SHP MENTHE 200ML_60X40_PVC_R").unwrap();
        assert_eq!(s[0].code, 270983);
        assert_eq!(s[0].dimensions, "60x40".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCR);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_273656() {
        let s = Sticker::parse_stickers("273656_DU MELAS FL INVISIBLE 30ML_40X60_PVC").unwrap();
        assert_eq!(s[0].code, 273656);
        assert_eq!(s[0].dimensions, "40x60".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVC);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_269226() {
        let s = Sticker::parse_stickers("269226_ULTRA FLUIDE_RADIANCE_80X25_PVC_REGULJAREN_OK")
            .unwrap();
        assert_eq!(s[0].code, 269226);
        assert_eq!(s[0].dimensions, "80x25".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVC);
        assert_eq!(s[0].text_color, Color::Black);
    }

    #[test]
    fn test_268175() {
        let s = Sticker::parse_stickers("268175_DU KELUAL SQUANORM OILY SHP 200ML_50X30_PVC-R")
            .unwrap();
        assert_eq!(s[0].code, 268175);
        assert_eq!(s[0].dimensions, "50x30".parse::<Dimensions>().unwrap());
        assert_eq!(s[0].material, Material::PVCR);
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
        let existing = Sticker::parse_stickers(
            "234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGHT_50X50_PVC_R_OK_PF",
        )
        .unwrap();
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
        let existing = Sticker::parse_stickers(
            "234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF",
        )
        .unwrap();
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
        let existing = Sticker::parse_stickers(
            "234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF",
        )
        .unwrap();
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
        let existing = Sticker::parse_stickers(
            "234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF",
        )
        .unwrap();
        let error = ParseStickerError::MissingCode(
            "AV CLEAN GEL TUBE 200ML + RANDO LENTG_50X50_PVC_R_OK_PF".into(),
        );

        let result =
            parser::try_infering_code_by_description_similiarity_measure(error, &existing, 0.93);
        assert!(result.is_err());
    }

    #[test]
    fn test_fail_with_unrelated_description() {
        let existing = Sticker::parse_stickers(
            "234191_AV CLEAN GEL TUBE 200ML + RANDOM LENGTH_50X50_PVC_R_OK_PF",
        )
        .unwrap();
        let error =
            ParseStickerError::MissingCode("FACE WASH FOAM FOR MEN_50X50_PVC_R_OK_PF".into());

        let result =
            parser::try_infering_code_by_description_similiarity_measure(error, &existing, 0.93);
        assert!(result.is_err());
    }

    #[test]
    fn test_double_sticker_dimensions_and_materials_colors() {
        use Color::*;
        use Material::*;

        let test_cases = vec![
            (
                "209989_DU DENSIAGE DOBAVKA 30TABL_30X101_45X59_PVC_R_SLV",
                vec!["30x101", "45x59"],
                vec![PVCRSLV, PVCRSLV],
                vec![Black, Black],
            ),
            (
                "207073 _RF VITALFAN SOL 30K_36X73_ 58X75_PAPER GREEN_DV_ST",
                vec!["36x73", "58x75"],
                vec![Paper, Paper],
                vec![Green, Green],
            ),
            (
                "254285_DU ANACAPS EXPERT 30CAPS_1ST 68X40_2ST 68X40_PAPER_BLUE_OK_PF",
                vec!["68x40", "68x40"],
                vec![Paper, Paper],
                vec![Blue, Blue],
            ),
            (
                "211949_EL SD GELULE MINCEUR TABLETES_45X102_45X40_PVC",
                vec!["45x40"],
                vec![PVC],
                vec![Black],
            ),
            (
                "207687_AD HYDRALBA UV RICH CR 40ML_70X40_PVC&40X45_PVC R_OK",
                vec!["70x40", "40x45"],
                vec![PVC, PVCR],
                vec![Black, Black],
            ),
            (
                "254310_DU ANACAPS REACTIV GEL 30U_40X68_42X50_PAPER BLUE_DB ST_PF",
                vec!["40x68", "42x50"],
                vec![Paper, Paper],
                vec![Blue, Blue],
            ),
            (
                "538752_AD DERMALIBUR CR BAR 100ML_40X100&40X45_DVA STIKERA_PVC_OK",
                vec!["40x100", "40x45"],
                vec![PVC, PVC],
                vec![Black, Black],
            ),
            (
                "515134_AD EXOMEGA DEFI 200ML_40X100_40X45_ДВОЕН СТИКЕР_PVC_OK",
                vec!["40x100", "40x45"],
                vec![PVC, PVC],
                vec![Black, Black],
            ),
            (
                "207686_AD HYDRALBA UV LEGERE TUBE 40ML_40X70_PVC_&_40X45_PVC_R_OK",
                vec!["40x70", "40x45"],
                vec![PVC, PVCR],
                vec![Black, Black],
            ),
            (
                "207873_BOX SPRAY ETA COLLECT_3X50ML_40X100_40X27_DV.ST.PVC_R_OK_PF",
                vec!["40x100", "40x27"],
                vec![PVCR, PVCR],
                vec![Black, Black],
            ),
            (
                "205475_RF VITALFAN PROGR SINGLE 30K_58X75_40X68_PAPER GREEN_DVOEN STIKER",
                vec!["58x75", "40x68"],
                vec![Paper, Paper],
                vec![Green, Green],
            ),
        ];

        for (filename, expected_sizes, expected_materials, expected_colors) in test_cases {
            let stickers = Sticker::parse_stickers(filename).unwrap();

            for i in 0..expected_sizes.len() {
                let expected_dim = expected_sizes[i].parse::<Dimensions>().unwrap();
                assert_eq!(
                    stickers[i].dimensions, expected_dim,
                    "Mismatch in dimensions for file: {}",
                    filename
                );

                assert_eq!(
                    stickers[i].material, expected_materials[i],
                    "Mismatch in material for sticker {} in file: {}",
                    i, filename
                );

                assert_eq!(
                    stickers[i].text_color, expected_colors[i],
                    "Mismatch in color for sticker {} in file: {}",
                    i, filename
                );
            }
        }
    }
}
