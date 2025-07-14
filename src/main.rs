use egui::ViewportBuilder;
use read_and_write_funcs::get_ids_from_user;

use crate::{
    api_funcs::get_item_funcs::{query_api}, get_minimal_lists::get_minimals, gui_funcs::egui_for_lists::HakuGIApp
};

pub mod other_helper_funcs;
pub mod parsed_models;
pub mod base_models;
pub mod character_funcs;
pub mod api_funcs;
pub mod gui_funcs;

use base_models::{character, weapon, tcg_cards, hakushin_lists};
use other_helper_funcs::{helper_funcs, read_and_write_funcs};
use api_funcs::{get_minimal_lists};

// TODO: Clean up!
#[tokio::main]
async fn main() {
    show_ui().await;
}

async fn show_ui() {
    let app = HakuGIApp::new().await;
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1100.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Genshin Data Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    ).unwrap();
}

async fn _non_ui_version() {
    let (_, _, _, artifacts) = get_minimals(false).await;
    let inputs: String = get_ids_from_user();

    query_api(&inputs, &artifacts).await;
}






