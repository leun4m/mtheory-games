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

#[derive(serde::Deserialize, serde::Serialize, Default, PartialEq, PartialOrd)]
#[serde(default)]
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
    score: i32,
    high_score: i32,
}

const POINTS_ON_CORRECT: i32 = 1;
const POINTS_ON_MISTAKE: i32 = -3;
const SECONDS_PER_GAME: i64 = 5;

const FONT_SIZE_HEADING: f32 = 48.0;
const FONT_SIZE_NORMAL: f32 = 32.0;
const FONT_SIZE_SMALL: f32 = 24.0;

impl MyEguiApp {
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

        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    fn handle_answer(&mut self, note: Note) {
        if note == self.correct_answer {
            self.status = "Richtig".into();
            self.score += POINTS_ON_CORRECT;
        } else {
            self.status = format!("Falsch, (Richtig wäre: {})", self.correct_answer);
            self.score += POINTS_ON_MISTAKE;
        }

        if self.score < 0 {
            self.score = 0;
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
        const BUTTON_SIZE: [f32; 2] = [80.0, 80.0];
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
            (aim.timestamp_millis() - Local::now().timestamp_millis()) as f32
                / (SECONDS_PER_GAME as f32 * 1000.0)
        } else {
            0.0
        };

        self.is_running = self.time_left > 0.0;
        if !self.is_running {
            self.high_score = std::cmp::max(self.score, self.high_score);
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.calc_time();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("mTheory Quiz!");
            ui.label(format!("Score: {}", self.score));
            ui.small(&self.status);

            if self.is_running {
                ui.add(ProgressBar::new(self.time_left));
                ui.label(format!("Key: {} - Scale Step: {}", self.key, self.step));
                ui.horizontal(|ui| {
                    self.add_option_button(ui, 0);
                    self.add_option_button(ui, 1);
                    self.add_option_button(ui, 2);
                    self.add_option_button(ui, 3);
                });
            } else if ui
                .add_sized([100.0, 50.0], egui::Button::new("Start"))
                .clicked()
            {
                self.status = "Starting...".to_string();
                self.start = Some(Local::now());
                self.score = 0;
                self.next_note();
                self.is_running = true;
            } else {
                ui.label(format!("Highscore: {}", self.high_score));
            }
        });

        ctx.request_repaint();
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
