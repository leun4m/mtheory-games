use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use egui_extras::RetainedImage;

use crate::scales_trainer::ScaleTrainer;

pub struct RootApp {
    image: RetainedImage,
    scales_trainer: ScaleTrainer,
}

impl Default for RootApp {
    fn default() -> Self {
        let image_data_bkg = include_bytes!("../assets/logo.svg");
        let image = RetainedImage::from_svg_bytes("logo", image_data_bkg).unwrap();
        Self {
            image,
            scales_trainer: Default::default(),
        }
    }
}

const FONT_SIZE_HEADING: f32 = 48.0;
const FONT_SIZE_NORMAL: f32 = 32.0;
const FONT_SIZE_SMALL: f32 = 24.0;

impl RootApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Get current context style
        let mut style = (*cc.egui_ctx.style()).clone();

        // Redefine text_styles
        style.text_styles = [
            (Heading, FontId::new(FONT_SIZE_HEADING, Proportional)),
            (Body, FontId::new(FONT_SIZE_NORMAL, Proportional)),
            (Monospace, FontId::new(FONT_SIZE_NORMAL, Proportional)),
            (Button, FontId::new(FONT_SIZE_NORMAL, Proportional)),
            (Small, FontId::new(FONT_SIZE_SMALL, Proportional)),
        ]
        .into();

        // Mutate global style with above changes
        cc.egui_ctx.set_style(style);

        Self::default()
    }
}

impl eframe::App for RootApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.image.show_max_size(ui, egui::vec2(320.0, 160.0));
                ui.small("// Trainer");
            })
        });
        self.scales_trainer.update(ctx, frame);
    }
}
