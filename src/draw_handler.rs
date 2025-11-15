use eframe::{Frame, App};
use egui::{CentralPanel, Grid, TopBottomPanel};
use crate::git_handle::GitStruct;

pub struct DrawHandler {
    git_struct: GitStruct,
}

impl<'a> App for DrawHandler {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            ui.label(format!("Commits loaded: {}", self.git_struct.get_len()));
        });

        ctx.request_repaint();
    }
}

impl DrawHandler {
    pub fn new(git_struct: GitStruct) -> Self {
        Self {
            git_struct,
        }
    }
}