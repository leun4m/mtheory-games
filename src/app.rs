use crate::note::{
    Note, Scale, ScaleStep, ALL_NOTES, ALL_SCALES, ALL_SCALES_WEIGHTED, ALL_SCALE_STEPS,
    SCALE_STEPS_WEIGHTS,
};
use egui::Ui;
use rand::{
    distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng, seq::SliceRandom,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct MyEguiApp {
    status: String,
    key: Note,
    correct_answer: Note,
    step: ScaleStep,
    scale: Scale,
    option: [Note; 4],
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    fn handle_answer(&mut self, note: Note) {
        if note == self.correct_answer {
            self.status = "Richtig".into();
        } else {
            self.status = format!("Falsch, (Richtig wäre: {})", self.correct_answer);
        }
        self.next_note();
    }

    fn next_note(&mut self) {
        let mut rng = rand::thread_rng();
        self.scale = random_scale(&mut rng);
        self.key = self.scale[0];
        self.step = random_scale_step(&mut rng);
        self.option = random_notes(self.scale[self.step.ord()], &mut rng);
        self.correct_answer = self.scale[self.step.ord()];
    }

    fn add_option_button(&mut self, ui: &mut Ui, id: usize) {
        const BUTTON_SIZE: [f32; 2] = [50., 20.];
        if ui
            .add_sized(
                BUTTON_SIZE,
                egui::Button::new(format!("{}", self.option[id])),
            )
            .clicked()
        {
            self.handle_answer(self.option[id]);
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("mTheory Quiz!");
            ui.label(&self.status);

            if ui.button("Start").clicked() {
                self.status = "Starting...".to_string();
                self.next_note();
            }

            ui.label(format!("Key: {}", self.key));
            ui.label(format!("Step {}", self.step));
            ui.horizontal(|ui| {
                self.add_option_button(ui, 0);
                self.add_option_button(ui, 1);
                self.add_option_button(ui, 2);
                self.add_option_button(ui, 3);
            })
        });
    }
}

fn random_scale(rng: &mut ThreadRng) -> Scale {
    let weights = WeightedIndex::new(ALL_SCALES_WEIGHTED).unwrap();
    ALL_SCALES[weights.sample(rng)]
}

fn random_scale_step(rng: &mut ThreadRng) -> ScaleStep {
    let weights = WeightedIndex::new(SCALE_STEPS_WEIGHTS).unwrap();
    ALL_SCALE_STEPS[weights.sample(rng)]
}

fn random_note(rng: &mut ThreadRng) -> Note {
    *ALL_NOTES.choose(rng).unwrap()
}

fn random_notes(note: Note, rng: &mut ThreadRng) -> [Note; 4] {
    let mut result: [Note; 4] = Default::default();
    result[0] = note;
    for item in result.iter_mut().skip(1) {
        *item = random_note(rng);
    }
    result.shuffle(rng);
    result
}
