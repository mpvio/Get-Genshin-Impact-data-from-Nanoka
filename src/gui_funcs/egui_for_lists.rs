use eframe::egui;
use egui::{Layout, ScrollArea, TextEdit, Ui};
use tokio::runtime::Runtime;

use crate::{api_funcs::get_item_funcs::query_api, base_models::hakushin_lists::MinimalArtifactMap, gui_funcs::display_lists::{get_names, ItemNames, get_custom_name, filter_items}};

pub struct HakuGIApp {
    characters: Vec<ItemNames>,
    weapons: Vec<ItemNames>,
    cards: Vec<ItemNames>,
    artifacts: Vec<ItemNames>,
    query: String,
    arts: Option<MinimalArtifactMap>,
    outputs: Vec<String>,
    char_search: String,
    weap_search: String,
    arti_search: String,
    card_search: String
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
            outputs: Vec::<String>::new(),
            char_search: String::new(),
            weap_search: String::new(),
            arti_search: String::new(),
            card_search: String::new()
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
                &mut self.outputs,
                &mut self.char_search,
                &mut self.weap_search,
                &mut self.arti_search,
                &mut self.card_search
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
    outputs: &mut Vec<String>,
    char_search: &mut String,
    weap_search: &mut String,
    arti_search: &mut String,
    card_search: &mut String
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
                //title and search bar
                let _heading = ui.heading("Characters");
                ui.add(
                    TextEdit::singleline(char_search)
                    .hint_text("Search")
                    .min_size(egui::vec2(40.0, 0.0))
                );
                ui.set_min_height(min_height);
                ui.set_min_width(min_width);
                // defines a separate scroll area with unique id
                // scrolling here doesn't scroll other panels
                let _ch_scroll = ScrollArea::vertical().id_salt("ch").show(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui|{
                        for item in filter_items(&characters, &char_search) {
                            ui.label(format!("{:<10}: {:<20}", item.key, get_custom_name(item)));
                        }
                    });
                });
                //print!("({}, {}) ", _heading.rect.width(), _ch_scroll.inner_rect.width());
            });
            ui.separator();
            ui.vertical(|ui| {
                let _heading = ui.heading("Weapons");
                ui.add(
                    TextEdit::singleline(weap_search)
                    .hint_text("Search")
                    .min_size(egui::vec2(40.0, 0.0))
                );
                ui.set_min_height(min_height);
                ui.set_min_width(min_width);
                let _we_scroll = ScrollArea::vertical().id_salt("we").show(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui|{
                        for item in filter_items(&weapons, &weap_search) {
                            ui.label(format!("{:<6}: {:<25}", item.key, get_custom_name(item)));
                        }
                    });
                });
                //print!("({}, {}) ", heading.rect.width(), we_scroll.inner_rect.width());
            });
            ui.separator();
            ui.vertical(|ui| {
                // header + search bar
                let _heading = ui.heading("Artifacts");
                ui.add(
                    TextEdit::singleline(arti_search)
                    .hint_text("Search")
                    .min_size(egui::vec2(40.0, 0.0))
                );

                ui.set_min_height(min_height);
                ui.set_min_width(min_width);
                let _ar_scroll = ScrollArea::vertical().id_salt("ar").show(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui|{
                        for item in filter_items(&artifacts, &arti_search) {
                            ui.label(format!("{:<5}: {:<25}", item.key, get_custom_name(item)));
                        }
                    });
                });
                //print!("({}, {}) ", heading.rect.width(), ar_scroll.inner_rect.width());
            });
            ui.separator();
            ui.vertical(|ui| {
                // header + search bar
                let _heading = ui.heading("Cards");
                ui.add(
                    TextEdit::singleline(arti_search)
                    .hint_text("Search")
                    .min_size(egui::vec2(40.0, 0.0))
                );

                ui.set_min_height(min_height);
                ui.set_min_width(min_width);
                let _ca_scroll = ScrollArea::vertical().id_salt("ca").show(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui|{
                        for item in filter_items(&cards, &card_search) {
                            ui.label(format!("{:<6}: {:<25}", item.key, get_custom_name(item)));
                        }
                    });
                });
                
                //println!("({}, {})", heading.rect.width(), ca_scroll.inner_rect.width());
            });
        });
        ui.separator();

        ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
            // get inputs, save outputs
                ui.label("Enter ids to query:");
                ui.text_edit_singleline(query);
                if ui.button("Search").clicked()  { // && !query.is_empty()
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