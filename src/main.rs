use std::collections::BinaryHeap;

use altab::{entries::ResultEntry, Altab};
use eframe::egui;
use egui::{Color32, RichText, ScrollArea};
mod altab;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

struct MyEguiApp {
    query: String,
    selected_index: usize,
    altab: Altab,
    entries: BinaryHeap<ResultEntry>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        MyEguiApp {
            query: "".to_owned(),
            selected_index: 0,
            altab: Altab::new(),
            entries: BinaryHeap::new(),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.entries.extend(self.altab.get_recv().try_iter());
        // for item in self.altab.get_recv().try_iter() {
        //     println!("Entry: {}", item.name);
        //     self.entries.push(item);
        // }
        egui::CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical()
                .max_height(200.0)
                .auto_shrink([false; 2]);
            let (current_scroll, max_scroll) = scroll_area
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        self.selected_index =
                            std::cmp::min(self.selected_index, self.entries.len());
                        for (index, item) in self.entries.iter().enumerate() {
                            ui.horizontal(|ui| {
                                _ = ui.button("");
                                let text = item.name.clone();
                                ui.label(format!("{0:3}", item.score));
                                if index == self.selected_index {
                                    let response = ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(text)
                                                .color(Color32::WHITE)
                                                .background_color(Color32::DARK_GRAY),
                                        );
                                    });
                                    response.response.scroll_to_me(None);
                                } else {
                                    ui.colored_label(Color32::WHITE, text);
                                }
                            });
                        }
                    });
                    let margin = ui.visuals().clip_rect_margin;
                    let current_scroll = ui.clip_rect().top() - ui.min_rect().top() + margin;
                    let max_scroll =
                        ui.min_rect().height() - ui.clip_rect().height() + 2.0 * margin;
                    (current_scroll, max_scroll)
                })
                .inner;
            ui.separator();
            if ui.text_edit_singleline(&mut self.query).changed() {
                self.altab.search(self.query.to_owned());
                self.entries.clear();
                println!("Clear! New query: {}", self.query);
            }
            if ctx.input().key_pressed(egui::Key::ArrowDown) {
                self.selected_index = std::cmp::min(49, self.selected_index + 1);
            } else if ctx.input().key_pressed(egui::Key::ArrowUp) {
                self.selected_index = self.selected_index.saturating_sub(1);
            }
            ui.label(format!(
                "Scroll offset: {:.0}/{:.0} px",
                current_scroll, max_scroll
            ));
            ui.label(format!(
                "selected index: {:.0}/{:.0} px",
                self.selected_index, self.selected_index
            ));
        });
    }
}
