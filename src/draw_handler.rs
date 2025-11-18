use std::path::PathBuf;

use crate::git_handle::GitStruct;
use eframe::{App, Frame};
use egui::{Button, CentralPanel, Grid, TopBottomPanel, Ui, Vec2};
use git2::Oid;

pub struct DrawHandler {
    git_struct: GitStruct,
    current_files: Vec<(PathBuf, Oid)>,
    selected_file: Option<PathBuf>,
    file_content: String,
}

impl<'a> App for DrawHandler {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        let commit = self.git_struct.get_commit();

        TopBottomPanel::top("Top Panel").show(ctx, |ui| {
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

                ui.add_space((width - (width / 6.0 * 2.0)) / 2.0);

                let prev_button = ui.add_enabled(self.git_struct.get_idx() > 0, |ui: &mut Ui| {
                    ui.add_sized(Vec2::new(width / 6.0, height), |ui: &mut Ui| {
                        ui.button("Previous")
                    })
                });

                let next_button = ui.add_enabled(
                    self.git_struct.get_len() - 1 > self.git_struct.get_idx(),
                    |ui: &mut Ui| {
                        ui.add_sized(Vec2::new(width / 6.0, height), |ui: &mut Ui| {
                            ui.button("Next")
                        })
                    },
                );

                if prev_button.clicked() {
                    self.git_struct.decrement_idx();
                    self.reload_files();
                }

                if next_button.clicked() {
                    self.git_struct.increment_idx();
                    self.reload_files();
                }
            });
        });

        TopBottomPanel::bottom("Timeline Panel").show(ctx, |ui| {
            self.render_timeline(ui);
        });

        let files = self.git_struct.get_file_tree(commit);
        self.current_files = files;

        CentralPanel::default().show(ctx, |ui| {
            let available_height = ui.available_height();

            ui.horizontal(|ui| {
                ui.set_height(available_height);
                self.render_file_tree(ui);

                ui.separator();

                self.render_file_viewer(ui);
            });
        });

        ctx.request_repaint();
    }
}

impl DrawHandler {
    pub fn new(git_struct: GitStruct) -> Self {
        let commit = git_struct.get_commit();
        let current_files = git_struct.get_file_tree(commit);
        Self {
            git_struct,
            current_files,
            selected_file: None,
            file_content: String::new(),
        }
    }

    fn render_file_tree(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.set_width(250.0);
            ui.heading("Files");

            let mut clicked_file: Option<(PathBuf, Oid)> = None;

            let available_height = ui.available_height();

            egui::ScrollArea::vertical()
                .id_salt("file tree")
                .max_height(available_height)
                .show(ui, |ui| {
                    if self.current_files.is_empty() {
                        ui.label("No files in this commit");
                        return;
                    }

                    for (path, blob_id) in &self.current_files {
                        let is_selected = self.selected_file.as_ref() == Some(path);

                        if ui
                            .selectable_label(is_selected, path.display().to_string())
                            .clicked()
                        {
                            clicked_file = Some((path.clone(), *blob_id));
                        }
                    }
                });

            if let Some((path, blob_id)) = clicked_file {
                self.handle_file_click(path, blob_id);
            }
        });
    }

    fn handle_file_click(&mut self, path: PathBuf, blob_id: Oid) {
        self.selected_file = Some(path);

        match self.git_struct.get_file_content(blob_id) {
            Ok(content) => {
                self.file_content = content;
            }
            Err(msg) => {
                self.file_content = format!("Error: {}", msg);
            }
        }
    }

    fn render_file_viewer(&self, ui: &mut Ui) {
        ui.vertical(|ui| {
            if let Some(path) = &self.selected_file {
                ui.heading(path.display().to_string());
            } else {
                ui.heading("No file selected");
            }

            ui.separator();

            let available_height = ui.available_height();

            egui::ScrollArea::both()
                .id_salt("file viewer")
                .max_height(available_height)
                .show(ui, |ui| {
                    if self.file_content.is_empty() {
                        ui.label("Select a file to view its contents");
                    } else {
                        ui.label(egui::RichText::new(&self.file_content).monospace());
                    }
                });
        });
    }

    fn reload_files(&mut self) {
        let commit = self.git_struct.get_commit();
        self.current_files = self.git_struct.get_file_tree(commit);

        if let Some(selected_path) = &self.selected_file {
            if let Some((_, blob_id)) = self
                .current_files
                .iter()
                .find(|(path, _)| path == selected_path)
            {
                self.handle_file_click(selected_path.clone(), *blob_id);
            } else {
                self.selected_file = None;
                self.file_content = String::new();
            }
        }
    }

    fn render_timeline(&mut self, ui: &mut Ui) {
        let total_commits = self.git_struct.get_len();

        if total_commits == 0 {
            ui.label("No commits loaded");
            return;
        }

        let current_idx = self.git_struct.get_idx();

        ui.vertical(|ui| {
            ui.set_height(80.0);
            ui.heading("Timeline");

            let desired_size = egui::Vec2::new(ui.available_width(), 40.0);
            let (response, painter) =
                ui.allocate_painter(desired_size, egui::Sense::click_and_drag());

            let rect = response.rect;

            painter.rect_filled(rect, 4.0, egui::Color32::from_gray(30));

            let line_y = rect.center().y;
            painter.line_segment(
                [
                    egui::pos2(rect.min.x + 10.0, line_y),
                    egui::pos2(rect.max.x - 10.0, line_y),
                ],
                egui::Stroke::new(2.0, egui::Color32::from_gray(100)),
            );

            let usable_width = rect.width() - 20.0;
            let start_x = rect.min.x + 10.0;

            for i in 0..total_commits {
                let x = start_x + (i as f32 / (total_commits - 1).max(1) as f32) * usable_width;
                let y = line_y;

                let (color, radius) = if i == current_idx {
                    (egui::Color32::from_rgb(100, 200, 255), 6.0)
                } else {
                    (egui::Color32::from_gray(150), 4.0)
                };

                painter.circle_filled(egui::pos2(x, y), radius, color);
            }

            if response.clicked() {
                if let Some(click_pos) = response.interact_pointer_pos() {
                    let relative_x = click_pos.x - start_x;
                    let normalized_x = (relative_x / usable_width).clamp(0.0, 1.0);
                    let clicked_idx = (normalized_x * (total_commits - 1) as f32).round() as usize;

                    self.jump_to_commit(clicked_idx);
                }
            }

            if response.hovered() {
                if let Some(hover_pos) = response.hover_pos() {
                    let relative_x = hover_pos.x - start_x;
                    let normalized_x = (relative_x / usable_width).clamp(0.0, 1.0);
                    let hovered_idx = (normalized_x * (total_commits - 1) as f32).round() as usize;

                    let x = start_x
                        + (hovered_idx as f32 / (total_commits - 1).max(1) as f32) * usable_width;
                    painter.circle_stroke(
                        egui::pos2(x, line_y),
                        8.0,
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 255, 100)),
                    );
                }
            }

            if response.dragged() {
                if let Some(drag_pos) = response.interact_pointer_pos() {
                    let relative_x = drag_pos.x - start_x;
                    let normalized_x = (relative_x / usable_width).clamp(0.0, 1.0);
                    let dragged_idx = (normalized_x * (total_commits - 1) as f32).round() as usize;

                    self.jump_to_commit(dragged_idx);
                }
            }

            ui.horizontal(|ui| {
                ui.label(format!("Commit {} of {}", current_idx + 1, total_commits));
            });
        });
    }

    fn jump_to_commit(&mut self, target_idx: usize) {
        let total = self.git_struct.get_len();
        if total == 0 {
            return;
        }

        let target_idx = target_idx.min(total - 1);

        let current = self.git_struct.get_idx();

        if target_idx > current {
            for _ in 0..(target_idx - current) {
                self.git_struct.increment_idx();
            }
        } else if target_idx < current {
            for _ in 0..(current - target_idx) {
                self.git_struct.decrement_idx();
            }
        }

        self.reload_files();
    }
}
