use core::f32;

use crate::git_handle::GitStruct;
use eframe::{App, Frame};
use egui::{Button, CentralPanel, Grid, TopBottomPanel, Ui, Vec2, text_selection::text_cursor_state::ccursor_previous_word};

pub struct DrawHandler {
    git_struct: GitStruct,
}

impl<'a> App for DrawHandler {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            let commit = self.git_struct.get_commit();
            ui.label(format!("Current Commit: {}", commit));
            ui.label(format!("Commited on: {}", self.git_struct.get_date(commit)));
            ui.label(format!(
                "By: {} at {}",
                self.git_struct
                    .get_author(commit)
                    .0
                    .unwrap_or("Name not found".to_string()),
                self.git_struct
                    .get_author(commit)
                    .1
                    .unwrap_or("Email not found".to_string())
            ));
            ui.separator();
            ui.horizontal_centered(|ui| {
                let width = ui.available_width();
                let height = ui.available_height();

                ui.add_space(width / 3.0);

                let prev_button = ui.add_enabled(self.git_struct.get_idx() > 0, |ui: &mut Ui| {
                    ui.add_sized(Vec2::new(width / 6.0, height), |ui: &mut Ui| {
                        ui.button("Previous")
                    })
                });

                let next_button = ui.add_enabled(self.git_struct.get_len() - 1 > self.git_struct.get_idx(), |ui: &mut Ui| {
                    ui.add_sized(Vec2::new(width / 6.0, height), |ui: &mut Ui| {
                        ui.button("Next")
                    })
                });
                
                if prev_button.clicked() {
                    self.git_struct.decrement_idx();
                }

                if next_button.clicked() {
                    self.git_struct.increment_idx();
                }

            });
        });

        ctx.request_repaint();
    }
}

impl DrawHandler {
    pub fn new(git_struct: GitStruct) -> Self {
        Self { git_struct }
    }
}
