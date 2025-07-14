use eframe::egui;
use egui::{Layout, ScrollArea, Ui};
use tokio::runtime::Runtime;

use crate::{api_funcs::get_item_funcs::query_api, base_models::hakushin_lists::MinimalArtifactMap, gui_funcs::display_lists::{get_names, ItemNames}};

pub struct HakuGIApp {
    characters: Vec<ItemNames>,
    weapons: Vec<ItemNames>,
    cards: Vec<ItemNames>,
    artifacts: Vec<ItemNames>,
    query: String,
    arts: Option<MinimalArtifactMap>
}

impl HakuGIApp {
    pub async fn new() -> Self {
        let (
            characters, 
            weapons, 
            cards, 
            artifacts,
            arts
        ) = get_names().await;
        Self {
            characters,
            weapons,
            cards,
            artifacts,
            query: String::new(),
            arts
        }
    }
}

impl eframe::App for HakuGIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            show_names_on_ui(
                ui, 
                &self.characters, 
                &self.weapons, 
                &self.cards, 
                &self.artifacts, 
                &mut self.query, 
                &self.arts
            );
        });
    }
}

pub fn show_names_on_ui(
    ui: &mut Ui,
    characters: &Vec<ItemNames>,
    weapons: &Vec<ItemNames>,
    cards: &Vec<ItemNames>,
    artifacts: &Vec<ItemNames>,
    query: &mut String,
    arts: &Option<MinimalArtifactMap>,
    //runtime: &mut Option<Runtime>
) {
    let min_height = 400.0;
    let min_width = 100.0;
    // contains lists AND text box
    ui.vertical(|ui| {
        // each component within ui.horizontal will display left to right
        // contains four lists
        ui.horizontal(|ui| {
            // each component within ui.vertical will display one on top of the other
            ui.vertical(|ui| {
                ui.heading("Characters");
                ui.set_min_height(min_height);
                ui.set_min_width(min_width);
                // defines a separate scroll area with unique id
                // scrolling here doesn't scroll other panels
                ScrollArea::vertical().id_salt("ch").show(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui|{
                        for item in characters {
                            ui.label(format!("{:<10}: {:<20}", item.key, item.name));
                        }
                    });
                });
            });
            ui.separator();
            ui.vertical(|ui| {
                ui.heading("Weapons");
                ui.set_min_height(min_height);
                ui.set_min_width(min_width);
                ScrollArea::vertical().id_salt("we").show(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui|{
                        for item in weapons {
                            ui.label(format!("{:<6}: {:<25}", item.key, item.name));
                        }
                    });
                });
            });
            ui.separator();
            ui.vertical(|ui| {
                ui.heading("Artifacts");
                ui.set_min_height(min_height);
                ui.set_min_width(min_width);
                ScrollArea::vertical().id_salt("ar").show(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui|{
                        for item in artifacts {
                            ui.label(format!("{:<5}: {:<25}", item.key, item.name));
                        }
                    });
                });
            });
            ui.separator();
            ui.vertical(|ui| {
                ui.heading("Cards");
                ui.set_min_height(min_height);
                ui.set_min_width(min_width);
                ScrollArea::vertical().id_salt("ca").show(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui|{
                        for item in cards {
                            let name = if item.key == "1506" {
                                "Wanderer"
                            } else {
                                &item.name
                            };
                            ui.label(format!("{:<6}: {:<25}", item.key, name));
                        }
                    });
                });
            });
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Enter ids to query:");
            ui.text_edit_singleline(query);
            if ui.button("Search").clicked() && !query.is_empty() {

                // clone params to access async function
                let query_clone = query.clone();
                let arts_clone = arts.clone();

                // create temporary thread to access async function
                std::thread::spawn(move || {
                    Runtime::new().unwrap().block_on(async {
                        query_api(&query_clone, &arts_clone).await;
                    })
                });

                // set text to null once queried
                query.clear();
            };
        });
    });
}