use eframe::egui;
use eframe::egui::{FontData, FontDefinitions, FontFamily};
use std::collections::BTreeMap;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    //
    // Load font =====================================
    //
    fonts.font_data.insert(
        "Segment7Standard".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/Segment7Standard.otf")),
    );

    let mut newfam = BTreeMap::new();
    newfam.insert(
        FontFamily::Name("Segment7Standard".into()),
        vec!["Segment7Standard".to_owned()],
    );
    fonts.families.append(&mut newfam);

    //
    // Set fonts =====================================
    //
    ctx.set_fonts(fonts);
}
