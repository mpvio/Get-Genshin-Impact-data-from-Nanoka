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
    arts: Option<MinimalArtifactMap>,
    outputs: Vec<String>
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
            arts,
            outputs: Vec::<String>::new()
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
                &self.arts,
                &mut self.outputs
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
    outputs: &mut Vec<String>
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

        ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
            // ui.horizontal(|ui| {

            // });

            // get inputs, save outputs
                ui.label("Enter ids to query:");
                ui.text_edit_singleline(query);
                if ui.button("Search").clicked() && !query.is_empty() {

                    // clone params to access async function
                    let query_clone = query.clone();
                    let arts_clone = arts.clone();
                    //let mut outputs_clone = outputs.clone();

                    // create temporary thread to access async function
                    let query_result = std::thread::spawn(move || {
                        Runtime::new().unwrap().block_on(async {
                            query_api(&query_clone, &arts_clone).await
                            //String::from("todo: get actual outputs")
                        })
                    });

                    // set text to null once queried
                    query.clear();
                    match query_result.join() {
                        Ok(mut result) => {
                            outputs.append(&mut result);
                        },
                        Err(_) => {},
                    }
                };

            // display results
            ui.heading("Results:");
            ui.horizontal(|ui| {
                ui.set_min_height(min_height/2.0);
                ScrollArea::vertical().id_salt("out").show(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                        for o in outputs {
                            if !o.is_empty() {
                                ui.label(format!("{o}"));
                            }
                        }
                    });
                });
            });
        });


    });
}