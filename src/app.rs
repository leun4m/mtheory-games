use crate::note::{
    Note, Scale, ScaleStep, ALL_NOTES, ALL_SCALES, ALL_SCALES_WEIGHTED, ALL_SCALE_STEPS,
    SCALE_STEPS_WEIGHTS,
};
use chrono::{DateTime, Duration, Local};
use egui::FontFamily::Proportional;
use egui::{FontId, Ui};
use egui::{ProgressBar, TextStyle::*};
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
    start: Option<DateTime<Local>>,
    time_left: f32,
    is_running: bool,
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let ctx = egui::Context::default();
        let mut style: egui::Style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Body, FontId::new(24.0, Proportional)),
            (Button, FontId::new(24.0, Proportional)),
        ]
        .into();

        cc.egui_ctx.set_style(style);
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
        const BUTTON_SIZE: [f32; 2] = [80., 40.];
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

    fn calc_time(&mut self) {
        self.time_left = if let Some(start) = self.start {
            let aim = start + Duration::seconds(SECONDS_PER_GAME);
            (aim.timestamp() - Local::now().timestamp()) as f32 / 10.0
        } else {
            0.0
        };

        self.is_running = self.time_left > 0.0;
    }
}

const SECONDS_PER_GAME: i64 = 10;

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.calc_time();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("mTheory Quiz!");
            ui.label(&self.status);

            if self.is_running {
                self.status = String::new();
                ui.add(ProgressBar::new(self.time_left).animate(true));
                ui.label(format!("Key: {}", self.key));
                ui.label(format!("Step {}", self.step));
                ui.horizontal(|ui| {
                    self.add_option_button(ui, 0);
                    self.add_option_button(ui, 1);
                    self.add_option_button(ui, 2);
                    self.add_option_button(ui, 3);
                });
            } else {
                if ui.button("Start").clicked() {
                    self.status = "Starting...".to_string();
                    self.start = Some(Local::now());
                    self.next_note();
                    self.is_running = true;
                }
            }
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
